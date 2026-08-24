use std::{path::Path, process::Command};

use super::model::{DoctorCheck, FamilyLock};

const GIT_BIN: &str = "/usr/bin/git";

pub(super) fn check_hub_checkout(hub_root: &Path) -> DoctorCheck {
    let dot_git = hub_root.join(".git");
    if dot_git.is_dir() {
        DoctorCheck::pass(
            "hub_checkout",
            "hub is an ordinary Git checkout (.git is a directory)",
        )
    } else if dot_git.is_file() {
        DoctorCheck::blocked(
            "hub_checkout",
            "hub is a linked Git worktree (.git is a file)",
            "clone the hub as an ordinary repository; Bullet Farm forbids worktrees",
        )
    } else {
        DoctorCheck::blocked(
            "hub_checkout",
            "hub has no .git directory",
            "start from an ordinary clone of the signed Bullet Farm hub tag",
        )
    }
}

pub(super) fn check_tools() -> DoctorCheck {
    let probes = [
        ("git", GIT_BIN, &["--version"] as &[&str]),
        ("rustc", "rustc", &["--version"]),
        ("cargo", "cargo", &["--version"]),
        ("rustup", "rustup", &["--version"]),
        ("rustfmt", "rustfmt", &["--version"]),
        ("clippy", "cargo", &["clippy", "--version"]),
        ("node", "node", &["--version"]),
        ("npm", "npm", &["--version"]),
        ("just", "just", &["--version"]),
    ];
    let mut versions = Vec::new();
    let mut missing = Vec::new();
    for (label, command, args) in probes {
        match command_version(command, args) {
            Ok(version) => versions.push(format!("{label}={version}")),
            Err(reason) => missing.push(format!("{label} ({reason})")),
        }
    }
    if missing.is_empty() {
        DoctorCheck::pass("toolchain", versions.join("; "))
    } else {
        DoctorCheck::blocked(
            "toolchain",
            format!("missing or unusable tools: {}", missing.join(", ")),
            "install the pinned Rust and Node toolchains plus git, rustup, rustfmt, clippy, npm, and just; rerun doctor",
        )
    }
}

pub(super) fn check_source_metadata(lock: &FamilyLock) -> DoctorCheck {
    let missing = lock
        .member
        .iter()
        .filter(|member| member.name != "bullet-farm")
        .filter(|member| {
            member.jeryu_url.as_deref().is_none_or(str::is_empty)
                && member.source_url.as_deref().is_none_or(str::is_empty)
        })
        .map(|member| member.name.as_str())
        .collect::<Vec<_>>();
    let missing_slugs = lock
        .member
        .iter()
        .filter(|member| member.name != "bullet-farm")
        .filter(|member| member.jeryu_slug.as_deref().is_none_or(str::is_empty))
        .map(|member| member.name.as_str())
        .collect::<Vec<_>>();
    if missing.is_empty() && missing_slugs.is_empty() {
        DoctorCheck::pass(
            "source_metadata",
            "every non-hub member has an immutable Jeryu source URL and slug",
        )
    } else {
        DoctorCheck::blocked(
            "source_metadata",
            format!(
                "family.lock lacks clone URLs for [{}] and Jeryu slugs for [{}]",
                missing.join(", "),
                missing_slugs.join(", ")
            ),
            "publish signed member tags, add immutable Jeryu source URLs/slugs and artifact checksums to the lock, then regenerate and verify it",
        )
    }
}

pub(super) fn check_family_layout(
    hub_root: &Path,
    family_root: Option<&Path>,
    lock: &FamilyLock,
) -> Vec<DoctorCheck> {
    let Some(family_root) = family_root else {
        return vec![DoctorCheck::blocked(
            "family_layout",
            "only the hub checkout is present; no outer repos.manifest.toml was found",
            "safe hub-only setup is not available until the lock carries verified sources and bullet-family setup is implemented; do not treat scripts/setup.sh as a fresh-clone installer",
        )];
    };
    let mut absent = Vec::new();
    let mut worktrees = Vec::new();
    let mut wrong_heads = Vec::new();
    let mut dirty = Vec::new();
    for member in &lock.member {
        let repo = if member.name == "bullet-farm" {
            hub_root.to_path_buf()
        } else {
            family_root.join(&member.name)
        };
        if !repo.join(".git").exists() {
            absent.push(member.name.clone());
            continue;
        }
        if !repo.join(".git").is_dir() {
            worktrees.push(member.name.clone());
            continue;
        }
        if member.name != "bullet-farm" {
            match git(&repo, &["rev-parse", "HEAD"]) {
                Ok(head) if head.trim() == member.commit_oid => {}
                Ok(head) => wrong_heads.push(format!(
                    "{} expected {} found {}",
                    member.name,
                    member.commit_oid,
                    head.trim()
                )),
                Err(reason) => wrong_heads.push(format!("{} ({reason})", member.name)),
            }
        }
        match git(
            &repo,
            &["status", "--porcelain=v2", "--untracked-files=all"],
        ) {
            Ok(status) if status.is_empty() => {}
            Ok(_) => dirty.push(member.name.clone()),
            Err(reason) => dirty.push(format!("{} ({reason})", member.name)),
        }
    }
    vec![
        layout_result(&absent, &worktrees),
        oid_result(&wrong_heads),
        cleanliness_result(&dirty),
    ]
}

fn layout_result(absent: &[String], worktrees: &[String]) -> DoctorCheck {
    if absent.is_empty() && worktrees.is_empty() {
        DoctorCheck::pass(
            "family_layout",
            "all locked members are ordinary sibling checkouts",
        )
    } else {
        DoctorCheck::blocked(
            "family_layout",
            format!(
                "missing members [{}]; forbidden worktrees [{}]",
                absent.join(", "),
                worktrees.join(", ")
            ),
            "create ordinary canonical clones for missing members; never create Git worktrees",
        )
    }
}

fn oid_result(wrong_heads: &[String]) -> DoctorCheck {
    if wrong_heads.is_empty() {
        DoctorCheck::pass(
            "member_oids",
            "every present non-hub member is at its locked commit OID",
        )
    } else {
        DoctorCheck::blocked(
            "member_oids",
            wrong_heads.join("; "),
            "recreate each clean canonical clone at the exact locked OID; never reset a dirty shared checkout",
        )
    }
}

fn cleanliness_result(dirty: &[String]) -> DoctorCheck {
    if dirty.is_empty() {
        DoctorCheck::pass("clean_checkouts", "every present family checkout is clean")
    } else {
        DoctorCheck::blocked(
            "clean_checkouts",
            format!("dirty checkouts: {}", dirty.join(", ")),
            "finish and hand off active claims; do not clean, reset, or stage another agent's changes",
        )
    }
}

fn command_version(command: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!("exit {:?}", output.status.code()));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout
        .lines()
        .next()
        .unwrap_or("version unavailable")
        .trim();
    Ok(line.chars().take(160).collect())
}

fn git(repo: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new(GIT_BIN)
        .arg("-C")
        .arg(repo)
        .args(args)
        .env_clear()
        .env("LC_ALL", "C")
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!("git exited {:?}", output.status.code()));
    }
    String::from_utf8(output.stdout).map_err(|_| "git emitted non-UTF-8 output".to_owned())
}
