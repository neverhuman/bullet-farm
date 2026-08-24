use std::{fs, path::Path, process::Command};

use bullet_family::family_lock::{load, verify_hub_checkout, verify_locked_checkout};

const TAG: &str = "v1.0.0-fixture.1";
const PRINCIPAL: &str = "fixture@bullet.invalid";

#[test]
fn signed_generation_and_exact_checkout_verification_are_non_circular() {
    let root = fixture_root();
    let key = root.join("release-key");
    command(
        &root,
        "/usr/bin/ssh-keygen",
        &["-q", "-t", "ed25519", "-N", "", "-f", text(&key)],
    );
    let public_key = fs::read_to_string(key.with_extension("pub")).unwrap();
    let allowed = format!("{PRINCIPAL} namespaces=\"git\" {}\n", public_key.trim());

    let hub = root.join("bullet-farm");
    let kernel = root.join("bullet-kernel");
    init_repo(&hub, &key, Some(&allowed));
    init_repo(&kernel, &key, None);
    fs::write(
        root.join("repos.manifest.toml"),
        format!(
            concat!(
                "family = \"bullet-farm\"\n",
                "required_repos = [\"bullet-farm\", \"bullet-kernel\"]\n",
                "[[repo]]\nname = \"bullet-farm\"\npath = \"{root}/bullet-farm\"\n",
                "[[repo]]\nname = \"bullet-kernel\"\npath = \"{root}/bullet-kernel\"\n",
                "jeryu_url = \"https://jeryu.example/root/bullet-kernel.git\"\n",
                "jeryu_slug = \"root/bullet-kernel\"\n",
            ),
            root = root.display()
        ),
    )
    .unwrap();

    bullet_family::family_lock::run(&root, &["generate".into(), "--tag".into(), TAG.into()])
        .expect("generation does not require a pre-existing hub tag");
    assert!(hub.join("family.lock").is_file());
    assert!(git_ok(&hub, &["rev-parse", &format!("refs/tags/{TAG}")]).is_none());

    git(&hub, &["add", "family.lock"]);
    git(&hub, &["commit", "-m", "Bind fixture family"]);
    sign_tag(&hub, &key);

    let lock = load(&hub.join("family.lock")).expect("strict generated lock");
    assert_eq!(
        lock.member.len(),
        1,
        "hub must not be serialized as a member"
    );
    assert!(lock.member("bullet-farm").is_none());
    let allowed_signers = hub.join("release/allowed_signers");
    let signer = verify_hub_checkout(&lock, &hub, &allowed_signers).expect("signed hub subject");
    assert!(signer.starts_with(&format!("{PRINCIPAL}|ed25519|SHA256:")));
    verify_locked_checkout(
        lock.member("bullet-kernel").unwrap(),
        &kernel,
        &allowed_signers,
    )
    .expect("exact signed non-hub subject");
    bullet_family::family_lock::run(&root, &["verify".into(), "--tag".into(), TAG.into()])
        .expect("complete lock verification");

    fs::write(
        root.join("repos.manifest.toml"),
        concat!(
            "family = \"bullet-farm\"\n",
            "required_repos = [\"bullet-farm\", \"bullet-kernel\"]\n",
        ),
    )
    .unwrap();
    bullet_family::family_lock::run(&root, &["verify".into(), "--tag".into(), TAG.into()])
        .expect("verification needs no mutable source table");

    fs::write(kernel.join("changed-after-tag.txt"), "new head\n").unwrap();
    git(&kernel, &["add", "changed-after-tag.txt"]);
    git(&kernel, &["commit", "-m", "Move past locked subject"]);
    let error = verify_locked_checkout(
        lock.member("bullet-kernel").unwrap(),
        &kernel,
        &allowed_signers,
    )
    .expect_err("changed HEAD must not satisfy the exact lock");
    assert_eq!(error.code(), "LOCKED_COMMIT_MISMATCH");
    fs::remove_dir_all(root).unwrap();
}

fn init_repo(repo: &Path, key: &Path, allowed_signers: Option<&str>) {
    fs::create_dir_all(repo.join("agent")).unwrap();
    fs::write(repo.join("Cargo.lock"), "# fixture dependency lock\n").unwrap();
    fs::write(repo.join("generated.txt"), "generated fixture\n").unwrap();
    fs::write(
        repo.join("agent/generated-zones.toml"),
        concat!(
            "[[zone]]\n",
            "path = \"generated.txt\"\n",
            "source = \"fixture generator\"\n",
            "owner = \"contracts\"\n",
        ),
    )
    .unwrap();
    if let Some(allowed) = allowed_signers {
        fs::create_dir_all(repo.join("release")).unwrap();
        fs::create_dir_all(repo.join("crates/bullet-wire")).unwrap();
        fs::write(repo.join("release/allowed_signers"), allowed).unwrap();
        fs::write(
            repo.join("crates/bullet-wire/schema.rs"),
            "pub struct Fixture;\n",
        )
        .unwrap();
    }
    command(repo, "/usr/bin/git", &["init", "--quiet"]);
    git(repo, &["config", "user.name", "Fixture Release"]);
    git(repo, &["config", "user.email", PRINCIPAL]);
    git(repo, &["add", "."]);
    git(repo, &["commit", "-m", "Fixture source"]);
    if allowed_signers.is_none() {
        sign_tag(repo, key);
    }
}

fn sign_tag(repo: &Path, key: &Path) {
    git(
        repo,
        &[
            "-c",
            "gpg.format=ssh",
            "-c",
            &format!("user.signingkey={}", key.display()),
            "tag",
            "-s",
            "-m",
            "Fixture release",
            TAG,
        ],
    );
}

fn git(repo: &Path, args: &[&str]) {
    command(repo, "/usr/bin/git", args);
}

fn git_ok(repo: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("/usr/bin/git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .env_clear()
        .env("LC_ALL", "C")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    output
        .status
        .success()
        .then(|| String::from_utf8(output.stdout).unwrap())
}

fn command(cwd: &Path, program: &str, args: &[&str]) {
    let output = Command::new(program)
        .current_dir(cwd)
        .args(args)
        .env_clear()
        .env("LC_ALL", "C")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{program} {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn fixture_root() -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("bullet-family-lock-git-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    fs::create_dir(&root).unwrap();
    root
}

fn text(path: &Path) -> &str {
    path.to_str().expect("fixture path is UTF-8")
}
