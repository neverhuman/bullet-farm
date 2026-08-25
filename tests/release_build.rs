//! Fail-closed acceptance for the single-target release builder.
//!
//! These cases prove the refusals that keep an unbuildable or unverifiable
//! archive from ever being produced. Producing a real archive needs a clean
//! four-repository family and a full locked release compile, which is the
//! operator lane documented in `docs/runbooks/release-build.md`.

#![cfg(target_os = "linux")]

use std::{fs, path::Path, process::Command};

use bullet_family::release;

const TARGET: &str = "x86_64-unknown-linux-gnu";

fn build(args: &[&str]) -> bullet_family::coord::CoordError {
    let args = std::iter::once("build")
        .chain(args.iter().copied())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    release::run(&args).expect_err("release build refuses this input")
}

fn git(repository: &Path, args: &[&str]) {
    let status = Command::new("/usr/bin/git")
        .arg("-C")
        .arg(repository)
        .args([
            "-c",
            "user.name=Release Build Test",
            "-c",
            "user.email=test@example.invalid",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "init.defaultBranch=main",
        ])
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_TERMINAL_PROMPT", "0")
        .status()
        .expect("git runs");
    assert!(status.success(), "git {args:?}");
}

/// A single-member family whose only repository is a real ordinary checkout.
fn family(root: &Path) {
    fs::write(
        root.join("repos.manifest.toml"),
        "schema_version = \"1.2.0\"\nfamily = \"bullet-farm\"\nrequired_repos = [\"bullet-farm\"]\n",
    )
    .expect("manifest");
    let member = root.join("bullet-farm");
    fs::create_dir(&member).expect("member");
    fs::write(member.join("family.lock"), FIXTURE_LOCK).expect("lock");
    git(&member, &["init", "--quiet"]);
    git(&member, &["add", "--all"]);
    git(&member, &["commit", "--quiet", "--message", "fixture"]);
}

const FIXTURE_LOCK: &str = concat!(
    "schema_version = \"2\"\nfamily = \"bullet-farm\"\ntag = \"v0.1.0-alpha.4\"\n",
    "schema_bundle_hash = \"blake3:00\"\n\n[[member]]\nname = \"bullet-farm\"\n",
    "tag = \"v0.1.0-alpha.4\"\ncommit_oid = \"0000000000000000000000000000000000000000\"\n",
    "schema_bundle_hash = \"blake3:00\"\n",
    "release_signing_identity = \"bot@jekko.ai|ed25519|SHA256:+FbqtZF+hPgrjJRh5Oq5gNUKUmtCykur2ZEUKAFRv+Y\"\n",
    "generated_client_hash = \"blake3:00\"\n",
);

#[test]
fn every_other_release_target_is_refused_by_name() {
    let out = tempfile::tempdir().expect("out");
    for target in [
        "aarch64-unknown-linux-gnu",
        "x86_64-apple-darwin",
        "aarch64-apple-darwin",
        "x86_64-pc-windows-msvc",
        "x86_64-unknown-linux-musl",
        "",
    ] {
        let error = build(&[
            "--target",
            target,
            "--out",
            out.path().join("bundle").to_str().expect("path"),
        ]);
        assert_eq!(error.code(), "UNSUPPORTED_RELEASE_TARGET", "{target}");
        assert!(
            error.to_string().contains("five-archive contract")
                || error.to_string().contains("five"),
            "{target}: {error}"
        );
    }
}

#[test]
fn the_output_path_must_be_absolute_and_absent() {
    let out = tempfile::tempdir().expect("out");
    let existing = out.path().join("already-here");
    fs::create_dir(&existing).expect("existing");
    let root = tempfile::tempdir().expect("family");
    let root = root.path().canonicalize().expect("canonical family");
    family(&root);
    assert_eq!(
        build(&[
            "--target",
            TARGET,
            "--out",
            "relative/bundle",
            "--family-root",
            root.to_str().expect("path"),
        ])
        .code(),
        "INVALID_RELEASE_BUILD_INPUT"
    );
    assert_eq!(
        build(&[
            "--target",
            TARGET,
            "--out",
            existing.to_str().expect("path"),
            "--family-root",
            root.to_str().expect("path"),
        ])
        .code(),
        "RELEASE_OUTPUT_EXISTS"
    );
}

#[test]
fn a_dirty_member_refuses_before_any_output_is_created() {
    let root = tempfile::tempdir().expect("family");
    let root = root.path().canonicalize().expect("canonical family");
    family(&root);
    fs::write(root.join("bullet-farm").join("scratch.txt"), "dirty").expect("dirty file");
    let out = tempfile::tempdir().expect("out");
    let bundle = out.path().join("bundle");
    let error = build(&[
        "--target",
        TARGET,
        "--out",
        bundle.to_str().expect("path"),
        "--family-root",
        root.to_str().expect("path"),
    ]);
    assert_eq!(error.code(), "DIRTY_SOURCE");
    assert!(
        !bundle.exists(),
        "a refused build must not create its output directory"
    );
}

#[test]
fn incomplete_arguments_are_refused_with_the_exact_usage() {
    for args in [
        vec![],
        vec!["--target", TARGET],
        vec!["--out", "/absolute/bundle"],
        vec!["--target", TARGET, "--out"],
    ] {
        assert_eq!(build(&args).code(), "USAGE", "{args:?}");
    }
    assert_eq!(
        build(&[
            "--target",
            TARGET,
            "--out",
            "/absolute/bundle",
            "--offline",
            "--offline",
        ])
        .code(),
        "DUPLICATE_OPTION"
    );
    assert_eq!(
        build(&[
            "--target",
            TARGET,
            "--out",
            "/absolute/bundle",
            "--unknown",
            "x",
        ])
        .code(),
        "USAGE"
    );
}
