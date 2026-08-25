use std::{fs, path::PathBuf, process::Command};

use bullet_family::family_lock::{
    FamilyLock, LOCK_SCHEMA_VERSION, LockedFile, LockedMember, load, parse,
};

fn digest(byte: char) -> String {
    format!("blake3:{}", byte.to_string().repeat(64))
}

fn oid(byte: char) -> String {
    format!("sha1:{}", byte.to_string().repeat(40))
}

fn file(path: &str, byte: char) -> LockedFile {
    LockedFile {
        path: path.to_owned(),
        digest: digest(byte),
    }
}

fn valid_lock() -> FamilyLock {
    FamilyLock {
        schema_version: LOCK_SCHEMA_VERSION.to_owned(),
        family: "bullet-farm".to_owned(),
        tag: "v1.0.0".to_owned(),
        schema_bundle_hash: digest('a'),
        member: vec![LockedMember {
            name: "bullet-kernel".to_owned(),
            jeryu_url: Some("https://jeryu.example/root/bullet-kernel.git".to_owned()),
            jeryu_slug: Some("root/bullet-kernel".to_owned()),
            tag: "v1.0.0".to_owned(),
            commit_oid: oid('e'),
            tree_oid: oid('f'),
            release_signing_identity: "release@bullet.farm|ed25519|SHA256:abc+123=".to_owned(),
            lockfile: vec![file("Cargo.lock", '1')],
            artifact: vec![
                file("contracts/generated/a.ts", '2'),
                file("contracts/generated/b.ts", '3'),
            ],
        }],
    }
}

fn encoded(lock: &FamilyLock) -> Vec<u8> {
    toml::to_string_pretty(lock)
        .expect("encode fixture")
        .into_bytes()
}

type LockMutation = Box<dyn Fn(&mut FamilyLock)>;

#[test]
fn strict_schema_accepts_complete_install_authority() {
    let lock = parse(&encoded(&valid_lock())).expect("valid schema 3 lock");
    assert_eq!(lock.member("bullet-kernel").unwrap().tree_oid, oid('f'));
    assert!(lock.member("bullet-farm").is_none());
    lock.validate_required_members(&["bullet-kernel".into(), "bullet-farm".into()])
        .expect("exact member set");
    assert!(
        lock.validate_required_members(&["bullet-farm".into()])
            .is_err()
    );
}

#[test]
fn strict_schema_rejects_hostile_identity_and_path_mutations() {
    let mutations: Vec<LockMutation> = vec![
        Box::new(|lock| lock.member[0].commit_oid = "a".repeat(40)),
        Box::new(|lock| lock.member[0].tree_oid = format!("sha1:{}", "A".repeat(40))),
        Box::new(|lock| lock.member[0].artifact[0].path = "../escape".to_owned()),
        Box::new(|lock| lock.member[0].artifact[0].path = ".git/config".to_owned()),
        Box::new(|lock| lock.member[0].artifact.swap(0, 1)),
        Box::new(|lock| lock.member[0].jeryu_url = None),
        Box::new(|lock| {
            lock.member[0].jeryu_url = Some("https://jeryu.example/root/different.git".to_owned());
        }),
        Box::new(|lock| lock.member[0].lockfile[0].digest = digest('A')),
        Box::new(|lock| lock.member[0].name = "bullet-farm".to_owned()),
    ];
    for mutate in mutations {
        let mut lock = valid_lock();
        mutate(&mut lock);
        assert!(parse(&encoded(&lock)).is_err());
    }
}

#[test]
fn strict_schema_rejects_unknown_duplicate_legacy_and_oversized_documents() {
    let text = String::from_utf8(encoded(&valid_lock())).unwrap();
    assert!(parse(format!("{text}\nunexpected = true\n").as_bytes()).is_err());
    assert!(
        parse(
            text.replacen(
                "schema_version",
                "schema_version = \"3\"\nschema_version",
                1
            )
            .as_bytes()
        )
        .is_err()
    );
    assert_eq!(
        parse(b"schema_version = \"2\"\n").unwrap_err().code(),
        "UNSUPPORTED_SCHEMA"
    );
    assert!(parse(&vec![b'a'; 1024 * 1024 + 1]).is_err());
}

#[cfg(unix)]
#[test]
fn lock_loader_rejects_symlinks() {
    use std::os::unix::fs::symlink;

    let root = fixture_root("symlink");
    let target = root.join("target.lock");
    let link = root.join("family.lock");
    fs::write(&target, encoded(&valid_lock())).unwrap();
    symlink(&target, &link).unwrap();
    assert!(load(&link).is_err());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn generation_fails_before_write_without_authenticated_sources() {
    let root = fixture_root("missing-source");
    for member in ["bullet-farm", "bullet-kernel"] {
        fs::create_dir(root.join(member)).unwrap();
    }
    fs::write(
        root.join("repos.manifest.toml"),
        format!(
            concat!(
                "family = \"bullet-farm\"\n",
                "required_repos = [\"bullet-farm\", \"bullet-kernel\"]\n",
                "[[repo]]\nname = \"bullet-farm\"\npath = \"{root}/bullet-farm\"\n",
                "[[repo]]\nname = \"bullet-kernel\"\npath = \"{root}/bullet-kernel\"\n",
                "jeryu_slug = \"root/bullet-kernel\"\n",
            ),
            root = root.display()
        ),
    )
    .unwrap();
    let error = bullet_family::family_lock::run(
        &root,
        &["generate".into(), "--tag".into(), "v1.0.0".into()],
    )
    .expect_err("missing URL must block before Git/tag access");
    assert_eq!(error.code(), "SOURCE_METADATA_UNAVAILABLE");
    assert!(!root.join("bullet-farm/family.lock").exists());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn public_cli_admits_only_generate_and_verify() {
    let root = fixture_root("cli-vocabulary");
    let hub = root.join("bullet-farm");
    fs::create_dir(&hub).unwrap();
    fs::write(
        root.join("repos.manifest.toml"),
        "family = \"bullet-farm\"\nrequired_repos = [\"bullet-farm\"]\n",
    )
    .unwrap();
    let lock = hub.join("family.lock");
    fs::write(&lock, "schema_version = \"2\"\n").unwrap();

    let usage = lock_cli(&root, &[]);
    assert_eq!(usage.status.code(), Some(2));
    let usage_error = String::from_utf8(usage.stderr).unwrap();
    assert!(usage_error.contains("lock <generate|verify>"));
    assert!(!usage_error.contains("lock <generate|check>"));

    let legacy = lock_cli(&root, &["lock", "check", "--tag", "v1.0.0"]);
    assert_eq!(legacy.status.code(), Some(2));
    let legacy_error = String::from_utf8(legacy.stderr).unwrap();
    assert!(legacy_error.contains("USAGE"));
    assert!(legacy_error.contains("lock <generate|verify>"));
    assert!(!legacy_error.contains("generate|check"));

    let verify = lock_cli(&root, &["lock", "verify", "--tag", "v1.0.0"]);
    assert_eq!(verify.status.code(), Some(4));
    assert!(
        String::from_utf8(verify.stderr)
            .unwrap()
            .contains("UNSUPPORTED_SCHEMA")
    );
    assert_eq!(
        fs::read_to_string(&lock).unwrap(),
        "schema_version = \"2\"\n"
    );
    fs::remove_dir_all(root).unwrap();
}

fn lock_cli(root: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_bullet-family"))
        .arg("--root")
        .arg(root)
        .args(args)
        .output()
        .expect("run bullet-family lock command")
}

fn fixture_root(name: &str) -> PathBuf {
    let root =
        std::env::temp_dir().join(format!("bullet-family-lock-{name}-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    fs::create_dir(&root).unwrap();
    root
}
