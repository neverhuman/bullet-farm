//! Bounded external commands used by setup.

use std::{ffi::OsStr, path::Path, process::Command, time::Duration};

use super::GIT_BIN;
use crate::{
    coord::CoordError,
    process::{Limits, run_bounded},
};

const GIT_LIMITS: Limits = Limits {
    timeout: Duration::from_secs(600),
    stdout_bytes: 16 * 1024 * 1024,
    stderr_bytes: 16 * 1024 * 1024,
};
const TOOL_LIMITS: Limits = Limits {
    timeout: Duration::from_secs(1_800),
    stdout_bytes: 16 * 1024 * 1024,
    stderr_bytes: 16 * 1024 * 1024,
};

pub(super) fn run_tool(repo: &Path, program: &str, args: &[&str]) -> Result<(), CoordError> {
    let output = run_bounded(
        Command::new(program).current_dir(repo).args(args),
        program,
        TOOL_LIMITS,
    )?;
    if output.status.success() {
        Ok(())
    } else {
        Err(CoordError::new(
            "SETUP_COMMAND_FAILED",
            format!(
                "{program} failed in {} with {}",
                repo.display(),
                output.status
            ),
        ))
    }
}

pub(super) fn run_git(repo: Option<&Path>, args: &[&OsStr]) -> Result<(), CoordError> {
    let mut command = Command::new(GIT_BIN);
    if let Some(repo) = repo {
        command.arg("-C").arg(repo);
    }
    let output = run_bounded(
        command
            .args(args)
            .env_clear()
            .env("LC_ALL", "C")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_TERMINAL_PROMPT", "0"),
        "Git setup operation",
        GIT_LIMITS,
    )?;
    if output.status.success() {
        Ok(())
    } else {
        Err(CoordError::new(
            "GIT_SETUP_FAILED",
            format!("Git setup operation failed with {}", output.status),
        ))
    }
}
