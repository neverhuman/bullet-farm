use std::{
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};

use bullet_family::release::{RELEASE_MANIFEST_SCHEMA_VERSION, ReleaseManifest};

const PRINCIPAL: &str = "fixture@bullet.invalid";
const TAG: &str = "v0.1.0-fixture.1";
static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);
const TARGETS: [&str; 5] = [
    "aarch64-apple-darwin",
    "aarch64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "x86_64-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
];

struct Fixture {
    root: PathBuf,
    bundle: PathBuf,
    allowed_signers: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "bullet-release-bundle-{}-{sequence}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove stale fixture");
        }
        let bundle = root.join("bundle");
        fs::create_dir_all(bundle.join("packages")).expect("fixture directories");
        let key = root.join("release-key");
        command(
            &root,
            "/usr/bin/ssh-keygen",
            &["-q", "-t", "ed25519", "-N", "", "-f", text(&key)],
        );
        let public_key = fs::read_to_string(key.with_extension("pub")).expect("public key");
        let fingerprint = fingerprint(&root, &key.with_extension("pub"));
        let allowed_signers = root.join("allowed_signers");
        fs::write(
            &allowed_signers,
            format!(
                "{PRINCIPAL} namespaces=\"bullet-farm-release\" {}\n",
                public_key.trim()
            ),
        )
        .expect("allowed signers");

        let family_lock = bundle.join("family.lock");
        fs::write(&family_lock, family_lock_text()).expect("family lock");
        let mut manifest = format!(
            concat!(
                "release_manifest_schema_version = \"{}\"\n",
                "family_lock_schema_version = \"3\"\n",
                "family = \"bullet-farm\"\n",
                "tag = \"{}\"\n",
                "hub_commit_oid = \"sha1:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"\n",
                "hub_tree_oid = \"sha1:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\"\n",
                "release_signing_identity = \"{}|ed25519|{}\"\n",
            ),
            RELEASE_MANIFEST_SCHEMA_VERSION, TAG, PRINCIPAL, fingerprint
        );
        append_file_table(&mut manifest, "family_lock", &bundle, "family.lock");

        for target in TARGETS {
            let archive_extension = if target == "x86_64-pc-windows-msvc" {
                "zip"
            } else {
                "tar.zst"
            };
            let archive = format!("packages/bullet-farm-{target}.{archive_extension}");
            let sbom = format!("packages/bullet-farm-{target}.cdx.json");
            let provenance = format!("packages/bullet-farm-{target}.intoto.jsonl");
            for (path, bytes) in [
                (&archive, format!("archive:{target}\n")),
                (
                    &sbom,
                    format!("{{\"bomFormat\":\"CycloneDX\",\"target\":\"{target}\"}}\n"),
                ),
                (
                    &provenance,
                    format!(
                        "{{\"_type\":\"https://in-toto.io/Statement/v1\",\"target\":\"{target}\"}}\n"
                    ),
                ),
            ] {
                fs::write(bundle.join(path), bytes).expect("release payload");
                sign(&root, &key, &bundle.join(path));
            }
            writeln!(manifest, "[[package]]").unwrap();
            writeln!(manifest, "target = {target:?}").unwrap();
            append_signed_table(&mut manifest, "package.archive", &bundle, &archive);
            append_signed_table(&mut manifest, "package.sbom", &bundle, &sbom);
            append_signed_table(&mut manifest, "package.provenance", &bundle, &provenance);
        }
        let manifest_path = bundle.join("release-manifest.toml");
        fs::write(&manifest_path, manifest).expect("release manifest");
        sign(&root, &key, &manifest_path);
        Self {
            root,
            bundle,
            allowed_signers,
        }
    }

    fn verify(&self) -> Result<String, bullet_family::coord::CoordError> {
        bullet_family::cli::run(
            [
                "bullet-family".into(),
                "release".into(),
                "verify".into(),
                "--bundle".into(),
                self.bundle.to_str().expect("UTF-8 path").into(),
                "--allowed-signers".into(),
                self.allowed_signers.to_str().expect("UTF-8 path").into(),
            ],
            Ok(self.bundle.clone()),
        )
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).expect("remove fixture");
    }
}

#[test]
fn signed_five_platform_bundle_verifies_twice_without_mutation() {
    let fixture = Fixture::new();
    let before = snapshot(&fixture.root);
    let first = fixture.verify().expect("first verification");
    let second = fixture.verify().expect("second verification");
    assert_eq!(first, second);
    assert!(first.contains("5 packages"));
    assert_eq!(snapshot(&fixture.root), before);
}

#[test]
fn payload_and_symlink_substitution_fail_closed() {
    let fixture = Fixture::new();
    let payload = fixture
        .bundle
        .join("packages/bullet-farm-aarch64-apple-darwin.tar.zst");
    fs::write(&payload, "mutated\n").expect("mutate payload");
    assert_eq!(
        fixture.verify().unwrap_err().code(),
        "INVALID_RELEASE_BUNDLE"
    );

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        fs::remove_file(&payload).expect("remove payload");
        symlink("../family.lock", &payload).expect("replace with symlink");
        assert_eq!(
            fixture.verify().unwrap_err().code(),
            "INVALID_RELEASE_BUNDLE"
        );
    }
}

#[test]
fn schema_refuses_unknown_fields_missing_targets_and_wrong_lock_version() {
    let fixture = Fixture::new();
    let bytes = fs::read(fixture.bundle.join("release-manifest.toml")).expect("manifest");
    let text = String::from_utf8(bytes).expect("UTF-8 manifest");
    let unknown = text.replacen(
        "family = \"bullet-farm\"",
        "family = \"bullet-farm\"\nunexpected = true",
        1,
    );
    assert_eq!(
        ReleaseManifest::parse(unknown.as_bytes())
            .unwrap_err()
            .code(),
        "INVALID_RELEASE_MANIFEST"
    );

    let mut parsed = ReleaseManifest::parse(text.as_bytes()).expect("valid manifest");
    parsed.package.pop();
    let missing = toml::to_string(&parsed).expect("encode missing target");
    assert_eq!(
        ReleaseManifest::parse(missing.as_bytes())
            .unwrap_err()
            .code(),
        "INVALID_RELEASE_MANIFEST"
    );

    let wrong_lock = text.replacen(
        "family_lock_schema_version = \"3\"",
        "family_lock_schema_version = \"2\"",
        1,
    );
    assert_eq!(
        ReleaseManifest::parse(wrong_lock.as_bytes())
            .unwrap_err()
            .code(),
        "UNSUPPORTED_SCHEMA"
    );
}

fn family_lock_text() -> String {
    concat!(
        "schema_version = \"3\"\n",
        "family = \"bullet-farm\"\n",
        "tag = \"v0.1.0-fixture.1\"\n",
        "schema_bundle_hash = \"blake3:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc\"\n",
        "[[member]]\n",
        "name = \"bullet-kernel\"\n",
        "jeryu_url = \"https://jeryu.example/root/bullet-kernel.git\"\n",
        "jeryu_slug = \"root/bullet-kernel\"\n",
        "tag = \"v0.1.0-fixture.1\"\n",
        "commit_oid = \"sha1:dddddddddddddddddddddddddddddddddddddddd\"\n",
        "tree_oid = \"sha1:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\"\n",
        "release_signing_identity = \"fixture@bullet.invalid|ed25519|SHA256:abc+123=\"\n",
        "artifact = []\n",
        "[[member.lockfile]]\n",
        "path = \"Cargo.lock\"\n",
        "digest = \"blake3:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"\n",
    )
    .to_owned()
}

fn append_signed_table(output: &mut String, table: &str, bundle: &Path, payload: &str) {
    append_file_table(output, &format!("{table}.file"), bundle, payload);
    append_file_table(
        output,
        &format!("{table}.signature"),
        bundle,
        &format!("{payload}.sig"),
    );
}

fn append_file_table(output: &mut String, table: &str, bundle: &Path, relative: &str) {
    let bytes = fs::read(bundle.join(relative)).expect("fixture file");
    writeln!(output, "[{table}]").unwrap();
    writeln!(output, "path = {relative:?}").unwrap();
    writeln!(output, "size = {}", bytes.len()).unwrap();
    writeln!(output, "digest = {:?}", digest(&bytes)).unwrap();
}

fn sign(cwd: &Path, key: &Path, payload: &Path) {
    command(
        cwd,
        "/usr/bin/ssh-keygen",
        &[
            "-Y",
            "sign",
            "-f",
            text(key),
            "-n",
            "bullet-farm-release",
            text(payload),
        ],
    );
}

fn fingerprint(cwd: &Path, public_key: &Path) -> String {
    let output = command_output(
        cwd,
        "/usr/bin/ssh-keygen",
        &["-lf", text(public_key), "-E", "sha256"],
    );
    String::from_utf8(output.stdout)
        .expect("UTF-8 fingerprint")
        .split_whitespace()
        .nth(1)
        .expect("fingerprint field")
        .to_owned()
}

fn digest(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3::hash(bytes).to_hex())
}

fn command(cwd: &Path, program: &str, args: &[&str]) {
    let output = command_output(cwd, program, args);
    assert!(
        output.status.success(),
        "{program} {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn command_output(cwd: &Path, program: &str, args: &[&str]) -> std::process::Output {
    Command::new(program)
        .current_dir(cwd)
        .args(args)
        .env_clear()
        .env("LC_ALL", "C")
        .output()
        .expect("run fixture command")
}

fn snapshot(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    fn visit(root: &Path, current: &Path, files: &mut Vec<(PathBuf, Vec<u8>)>) {
        for entry in fs::read_dir(current).expect("read fixture directory") {
            let entry = entry.expect("fixture entry");
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, files);
            } else {
                files.push((
                    path.strip_prefix(root)
                        .expect("relative path")
                        .to_path_buf(),
                    fs::read(path).expect("fixture bytes"),
                ));
            }
        }
    }
    let mut files = Vec::new();
    visit(root, root, &mut files);
    files.sort_by(|left, right| left.0.cmp(&right.0));
    files
}

fn text(path: &Path) -> &str {
    path.to_str().expect("fixture path is UTF-8")
}
