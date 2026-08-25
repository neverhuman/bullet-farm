//! `bullet-family check release --report`: deterministic, portable, never green.

use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

const LOCK: &str = include_str!("fixtures/release-truth/family.lock");
const RELEASE_INDEX: &str = "# Fixture release contract\n\nStatus: **BLOCKED — fixture**  \nOwner: fixture\nLast reviewed: 2026-01-01\n";
const MANIFEST: &str = "schema_version = \"1.2.0\"\nfamily = \"bullet-farm\"\numbrella_repo = \"bullet-farm\"\nrequired_repos = [\"bullet-farm\", \"bullet-kernel\", \"bullet-git\", \"bullet-portal\"]\n";

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn hub_only() -> Self {
        let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "bullet-release-truth-{}-{sequence}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove stale fixture");
        }
        let hub = root.join("bullet-farm");
        write(
            &hub.join("Cargo.toml"),
            "[package]\nname='fixture-hub'\nversion='0.0.0'\n",
        );
        write(&hub.join("family.lock"), LOCK);
        write(&hub.join("scripts/setup.sh"), "#!/bin/sh\nexit 1\n");
        write(&hub.join("repos.manifest.toml"), MANIFEST);
        write(&hub.join("docs/release.md"), RELEASE_INDEX);
        Self { root }
    }

    fn family() -> Self {
        let fixture = Self::hub_only();
        write(&fixture.root.join("repos.manifest.toml"), MANIFEST);
        for name in [
            "bullet-farm",
            "bullet-kernel",
            "bullet-git",
            "bullet-portal",
        ] {
            let repo = fixture.root.join(name);
            fs::create_dir_all(&repo).expect("repository fixture");
            write(&repo.join("README.md"), "fixture\n");
            git(&repo, &["init", "-q"]);
            git(&repo, &["config", "user.name", "Truth Fixture"]);
            git(&repo, &["config", "user.email", "truth@example.invalid"]);
            git(&repo, &["add", "."]);
            git(
                &repo,
                &[
                    "-c",
                    "commit.gpgsign=false",
                    "commit",
                    "-q",
                    "-m",
                    "fixture",
                    "--date",
                    "2026-01-02T03:04:05Z",
                ],
            );
        }
        fixture
    }

    fn hub(&self) -> PathBuf {
        self.root.join("bullet-farm")
    }

    fn run(&self, tail: &[&str]) -> Output {
        let mut args = vec!["--root", self.root.to_str().expect("UTF-8 fixture")];
        args.extend_from_slice(tail);
        Command::new(env!("CARGO_BIN_EXE_bullet-family"))
            .args(args)
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("run bullet-family")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).expect("remove fixture");
    }
}

fn write(path: &Path, content: &str) {
    fs::create_dir_all(path.parent().expect("fixture parent")).expect("fixture directories");
    fs::write(path, content).expect("fixture file");
}

fn git(repository: &Path, args: &[&str]) {
    let output = Command::new("/usr/bin/git")
        .arg("-C")
        .arg(repository)
        .args(args)
        .env_clear()
        .env("HOME", "/")
        .env("LC_ALL", "C")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_COMMITTER_DATE", "2026-01-02T03:04:05Z")
        .output()
        .expect("fixture Git");
    assert!(output.status.success(), "fixture Git failed: {output:?}");
}

fn head(repository: &Path) -> String {
    let output = Command::new("/usr/bin/git")
        .args(["-C", repository.to_str().unwrap(), "rev-parse", "HEAD"])
        .output()
        .expect("fixture Git");
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn gate_lines(page: &str) -> Vec<&str> {
    let (_, gates) = page.split_once("## Gates").expect("gates section");
    let (gates, _) = gates.split_once("## Excluded").expect("excluded section");
    gates
        .lines()
        .filter(|line| {
            line.contains("**")
                || line.contains("- Why it matters:")
                || line.contains("- Acceptance:")
        })
        .collect()
}

fn assert_never_closed(page: &str) {
    let lines = gate_lines(page);
    assert_eq!(lines.len(), 26 * 3);
    for line in lines {
        for token in line.split(|byte: char| !byte.is_ascii_alphanumeric()) {
            assert!(
                !matches!(
                    token.to_ascii_lowercase().as_str(),
                    "verified" | "proven" | "done" | "complete"
                ),
                "unreceipted claim reads as closed: {line}"
            );
        }
    }
}

#[test]
fn portable_report_matches_the_golden_page_from_a_hub_only_checkout() {
    let fixture = Fixture::hub_only();
    let first = fixture.run(&["check", "release", "--report", "--portable"]);
    let second = fixture.run(&["check", "release", "--report", "--portable"]);
    assert_eq!(first.status.code(), Some(3), "{first:?}");
    assert!(first.stderr.is_empty());
    assert_eq!(first.stdout, second.stdout);
    let page = String::from_utf8(first.stdout).unwrap();
    let golden = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/release-truth/golden.md"),
    )
    .expect("golden page");
    assert_eq!(page, golden, "portable page drifted from golden");
    assert!(!page.contains(fixture.root.to_str().unwrap()));
    assert_never_closed(&page);
}

#[test]
fn live_report_binds_subjects_and_check_report_freshness() {
    let fixture = Fixture::family();
    let first = fixture.run(&["check", "release", "--report"]);
    let second = fixture.run(&["check", "release", "--report"]);
    assert_eq!(first.status.code(), Some(3), "{first:?}");
    assert_eq!(first.stdout, second.stdout);
    let page = String::from_utf8(first.stdout).unwrap();
    assert!(page.contains("RELEASE DECISION: BLOCKED"));
    assert!(page.contains("hub HEAD committed 2026-01-02T03:04:05+00:00"));
    assert!(page.contains(&format!("| hub | `{}` |", fixture.hub().display())));
    let hub_head = head(&fixture.hub());
    assert!(page.contains(&format!("| bullet-farm | `sha1:{hub_head}` | `sha1:")));
    assert!(page.contains("| clean |"));
    assert!(page.contains("| binds current HEADs | NO — bullet-farm locked `4d7f2173"));
    assert!(page.contains("| Mechanical gates (fast) | NOT RUN (no generated check report) |"));
    assert!(page.contains("| Mechanical gates (required) | NOT RUN (no generated check report) |"));
    assert!(page.contains("| Evidence completeness | 0 of 26 receipted |"));
    assert!(page.contains("| Release review | HOLD"));
    assert!(page.contains("| Deployment match | N/A"));
    assert!(page.contains("| Post-deploy survival | NOT ESTABLISHED |"));
    assert_never_closed(&page);

    let stale = format!(
        "{{\"schema_version\":2,\"command\":\"check\",\"tier\":\"FAST\",\"status\":\"PASS\",\"gates\":[{{\"id\":\"fast.hub\",\"status\":\"PASS\",\"class\":\"COMPONENT\",\"detail\":\"d\",\"repair\":null,\"subjects\":[{{\"repository\":\"bullet-farm\",\"commit_oid\":\"sha1:{}\",\"tree_oid\":\"sha1:{}\"}}]}}]}}",
        "0".repeat(40),
        "1".repeat(40)
    );
    write(
        &fixture.hub().join(".bullet-family/check-fast.json"),
        &stale,
    );
    let fresh = stale.replace(&"0".repeat(40), &hub_head);
    write(
        &fixture.hub().join(".bullet-family/check-required.json"),
        &fresh.replace("FAST", "REQUIRED"),
    );
    write(&fixture.hub().join("UNTRACKED"), "dirty\n");
    let output = fixture.run(&["check", "release", "--report"]);
    assert_eq!(output.status.code(), Some(3));
    let page = String::from_utf8(output.stdout).unwrap();
    assert!(page.contains("| Mechanical gates (fast) | STALE — PASS over 1 gates recorded against other subjects (bullet-farm recorded sha1:0000"));
    assert!(page.contains("| Mechanical gates (required) | PASS — 1 gates on current HEADs |"));
    assert!(page.contains("| bullet-farm | `sha1:"));
    assert!(page.contains("| dirty (3 entries) |"));
    assert!(page.contains("| fast check report | `.bullet-family/check-fast.json` | blake3:"));
}

#[test]
fn report_mode_is_release_only_and_strict() {
    let fixture = Fixture::hub_only();
    for args in [
        vec!["check", "fast", "--report"],
        vec!["check", "required", "--report", "--portable"],
        vec!["check", "release", "--portable"],
        vec!["check", "release", "--report", "--json"],
    ] {
        let output = fixture.run(&args);
        assert_eq!(output.status.code(), Some(2), "args={args:?}");
        assert!(output.stdout.is_empty(), "args={args:?}");
        assert!(String::from_utf8(output.stderr).unwrap().contains("USAGE"));
    }
}
