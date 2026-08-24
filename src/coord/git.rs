use std::path::Path;
use std::process::Command;

use super::{CoordError, validate_path, validate_repo_name};

const GIT_BIN: &str = "/usr/bin/git";

pub(super) fn verify_commit_paths(
    family_root: &Path,
    repo: &str,
    commit_oid: &str,
    expected_paths: &[String],
) -> Result<(), CoordError> {
    validate_repo_name(repo)?;
    let repo_root = family_root.join(repo);
    if !repo_root.join(".git").is_dir() {
        return Err(CoordError::new(
            "REPOSITORY_NOT_FOUND",
            format!("{} is not a Git repository", repo_root.display()),
        ));
    }
    git(
        &repo_root,
        &["cat-file", "-e", &format!("{commit_oid}^{{commit}}")],
    )?;
    let output = git(
        &repo_root,
        &[
            "diff-tree",
            "--root",
            "--no-commit-id",
            "--name-only",
            "-r",
            commit_oid,
        ],
    )?;
    let mut actual = output
        .lines()
        .map(validate_path)
        .collect::<Result<Vec<_>, _>>()?;
    actual.sort();
    actual.dedup();
    if actual != expected_paths {
        return Err(CoordError::new(
            "COMMIT_PATH_MISMATCH",
            format!(
                "commit {commit_oid} paths {:?} differ from handed-off paths {:?}",
                actual, expected_paths
            ),
        ));
    }
    Ok(())
}

fn git(repo_root: &Path, args: &[&str]) -> Result<String, CoordError> {
    let output = Command::new(GIT_BIN)
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .env_clear()
        .env("LC_ALL", "C")
        .output()
        .map_err(CoordError::io)?;
    if !output.status.success() {
        return Err(CoordError::new(
            "COMMIT_NOT_FOUND",
            format!(
                "Git could not resolve the receipted commit in {}",
                repo_root.display()
            ),
        ));
    }
    String::from_utf8(output.stdout)
        .map_err(|_| CoordError::new("INVALID_GIT_OUTPUT", "Git emitted non-UTF-8 paths"))
}
