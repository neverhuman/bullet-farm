//! Admitted, bounded external commands used by setup.

mod environment;
mod subject;

#[cfg(test)]
mod tests;

use std::{
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
    time::Duration,
};

use self::subject::AdmittedFile;
use super::{BASH_BIN, GIT_BIN};
use crate::{
    coord::CoordError,
    process::{Limits, run_bounded},
};

pub(super) use environment::SetupEnvironment;

const GIT_LIMITS: Limits = Limits {
    timeout: Duration::from_secs(600),
    stdout_bytes: 16 * 1024 * 1024,
    stderr_bytes: 16 * 1024 * 1024,
};
const PROBE_LIMITS: Limits = Limits {
    timeout: Duration::from_secs(10),
    stdout_bytes: 64 * 1024,
    stderr_bytes: 64 * 1024,
};
const TOOL_LIMITS: Limits = Limits {
    timeout: Duration::from_secs(1_800),
    stdout_bytes: 16 * 1024 * 1024,
    stderr_bytes: 16 * 1024 * 1024,
};
const MAX_TOOL_BYTES: u64 = 512 * 1024 * 1024;

static ADMITTED_GIT: OnceLock<CommandSpec> = OnceLock::new();

#[derive(Debug)]
pub(super) struct Toolchain {
    cargo: CommandSpec,
    npm: CommandSpec,
    bash: CommandSpec,
    trusted_path: OsString,
}

impl Toolchain {
    pub(super) fn admit(
        cargo: Option<&Path>,
        node: Option<&Path>,
        npm_cli: Option<&Path>,
    ) -> Result<Self, CoordError> {
        let cargo = required_path(cargo, "Cargo")?;
        let node = required_path(node, "Node")?;
        let npm_cli = required_path(npm_cli, "npm CLI")?;
        let cargo = CommandSpec::admit(ToolIdentity::Cargo, cargo, Vec::new(), Vec::new())?;
        let node = AdmittedFile::admit("Node", node, true)?;
        CommandSpec::probe_identity(ToolIdentity::Node, &node, &[], &[])?;
        let npm_cli = AdmittedFile::admit("npm CLI", npm_cli, false)?;
        let npm_subject = npm_cli.execution_path().into_os_string();
        let npm =
            CommandSpec::from_admitted(ToolIdentity::Npm, node, vec![npm_subject], vec![npm_cli])?;
        let bash_path = fs::canonicalize(BASH_BIN).map_err(|error| {
            tool_error(
                "SETUP_TOOL_UNAVAILABLE",
                "Bash",
                format!("{BASH_BIN} cannot be resolved: {error}"),
            )
        })?;
        let bash = CommandSpec::admit(ToolIdentity::Bash, &bash_path, Vec::new(), Vec::new())?;
        let trusted_path = trusted_path([&cargo, &npm, &bash])?;
        Ok(Self {
            cargo,
            npm,
            bash,
            trusted_path,
        })
    }

    pub(super) fn run_cargo(
        &self,
        repo: &Path,
        args: &[&str],
        environment: &SetupEnvironment,
    ) -> Result<(), CoordError> {
        self.cargo.run(repo, args, environment)
    }

    pub(super) fn run_npm(
        &self,
        repo: &Path,
        args: &[&str],
        environment: &SetupEnvironment,
    ) -> Result<(), CoordError> {
        self.npm.run(repo, args, environment)
    }

    pub(super) fn run_bash(
        &self,
        repo: &Path,
        args: &[&str],
        environment: &SetupEnvironment,
    ) -> Result<(), CoordError> {
        self.bash.run(repo, args, environment)
    }

    pub(super) fn trusted_path(&self) -> &OsStr {
        &self.trusted_path
    }
}

#[derive(Debug)]
struct CommandSpec {
    identity: ToolIdentity,
    program: AdmittedFile,
    prefix_args: Vec<OsString>,
    companions: Vec<AdmittedFile>,
}

impl CommandSpec {
    fn admit(
        identity: ToolIdentity,
        program: &Path,
        prefix_args: Vec<OsString>,
        companions: Vec<AdmittedFile>,
    ) -> Result<Self, CoordError> {
        let program = AdmittedFile::admit(identity.label(), program, true)?;
        Self::from_admitted(identity, program, prefix_args, companions)
    }

    fn from_admitted(
        identity: ToolIdentity,
        program: AdmittedFile,
        prefix_args: Vec<OsString>,
        companions: Vec<AdmittedFile>,
    ) -> Result<Self, CoordError> {
        Self::probe_identity(identity, &program, &prefix_args, &companions)?;
        Ok(Self {
            identity,
            program,
            prefix_args,
            companions,
        })
    }

    fn probe_identity(
        identity: ToolIdentity,
        program: &AdmittedFile,
        prefix_args: &[OsString],
        companions: &[AdmittedFile],
    ) -> Result<(), CoordError> {
        program.verify()?;
        for companion in companions {
            companion.verify()?;
        }
        let path = probe_path(&program.path)?;
        let output = run_bounded(
            Command::new(program.execution_path())
                .args(prefix_args)
                .arg("--version")
                .env_clear()
                .env("HOME", "/")
                .env("PATH", path)
                .env("LC_ALL", "C")
                .env("GIT_CONFIG_NOSYSTEM", "1")
                .env("GIT_CONFIG_GLOBAL", "/dev/null")
                .env("GIT_TERMINAL_PROMPT", "0"),
            identity.label(),
            PROBE_LIMITS,
        )?;
        let stdout = std::str::from_utf8(&output.stdout).map_err(|_| {
            tool_error(
                "SETUP_TOOL_IDENTITY_MISMATCH",
                identity.label(),
                "version output is not UTF-8",
            )
        })?;
        let version = stdout.lines().next().unwrap_or_default();
        if !output.status.success() || !identity.matches(version) {
            return Err(tool_error(
                "SETUP_TOOL_IDENTITY_MISMATCH",
                identity.label(),
                "bounded --version probe did not identify the required tool",
            ));
        }
        Ok(())
    }

    fn run(
        &self,
        repo: &Path,
        args: &[&str],
        environment: &SetupEnvironment,
    ) -> Result<(), CoordError> {
        self.run_after_verify(repo, args, environment, || Ok(()))
    }

    fn run_after_verify(
        &self,
        repo: &Path,
        args: &[&str],
        environment: &SetupEnvironment,
        after_verify: impl FnOnce() -> Result<(), CoordError>,
    ) -> Result<(), CoordError> {
        environment.verify()?;
        self.verify_sources()?;
        after_verify()?;
        let mut command = Command::new(self.program.execution_path());
        command.current_dir(repo).args(&self.prefix_args).args(args);
        environment.apply(&mut command);
        let output = run_bounded(&mut command, self.identity.label(), TOOL_LIMITS);
        environment.verify()?;
        self.verify_sources()?;
        let output = output?;
        if output.status.success() {
            Ok(())
        } else {
            Err(CoordError::new(
                "SETUP_COMMAND_FAILED",
                format!(
                    "{} failed in {} with {}",
                    self.identity.label(),
                    repo.display(),
                    output.status
                ),
            ))
        }
    }

    fn run_git(&self, repo: Option<&Path>, args: &[&OsStr]) -> Result<(), CoordError> {
        self.run_git_after_verify(repo, args, || Ok(()))
    }

    fn run_git_after_verify(
        &self,
        repo: Option<&Path>,
        args: &[&OsStr],
        after_verify: impl FnOnce() -> Result<(), CoordError>,
    ) -> Result<(), CoordError> {
        self.verify_sources()?;
        after_verify()?;
        let mut command = Command::new(self.program.execution_path());
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
            self.identity.label(),
            GIT_LIMITS,
        );
        self.verify_sources()?;
        let output = output?;
        if output.status.success() {
            Ok(())
        } else {
            Err(CoordError::new(
                "GIT_SETUP_FAILED",
                format!("Git setup operation failed with {}", output.status),
            ))
        }
    }

    fn verify_sources(&self) -> Result<(), CoordError> {
        self.program.verify()?;
        for companion in &self.companions {
            companion.verify()?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
enum ToolIdentity {
    Cargo,
    Node,
    Npm,
    Bash,
    Git,
}

impl ToolIdentity {
    const fn label(self) -> &'static str {
        match self {
            Self::Cargo => "Cargo setup operation",
            Self::Node => "Node setup runtime",
            Self::Npm => "npm setup operation",
            Self::Bash => "Bash setup operation",
            Self::Git => "Git setup operation",
        }
    }

    fn matches(self, version: &str) -> bool {
        match self {
            Self::Cargo => version
                .strip_prefix("cargo ")
                .and_then(|rest| rest.split_whitespace().next())
                .is_some_and(is_version),
            Self::Node => version.strip_prefix('v').is_some_and(is_version),
            Self::Npm => is_version(version),
            Self::Bash => version.starts_with("GNU bash, version "),
            Self::Git => version.strip_prefix("git version ").is_some_and(is_version),
        }
    }
}

fn is_version(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'+' | b'_'))
        && value
            .split('.')
            .take(2)
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
}

fn required_path<'a>(path: Option<&'a Path>, label: &str) -> Result<&'a Path, CoordError> {
    path.ok_or_else(|| {
        tool_error(
            "SETUP_TOOL_MISSING",
            label,
            "an explicit canonical absolute path is required",
        )
    })
}

fn tool_error(code: &'static str, label: &str, detail: impl AsRef<str>) -> CoordError {
    CoordError::new(code, format!("{label}: {}", detail.as_ref()))
}

fn probe_path(program: &Path) -> Result<OsString, CoordError> {
    let parent = program.parent().ok_or_else(|| {
        tool_error(
            "SETUP_TOOL_PATH_NOT_CANONICAL",
            "setup tool",
            "canonical program has no parent",
        )
    })?;
    std::env::join_paths([parent, Path::new("/usr/bin")])
        .map_err(|error| tool_error("SETUP_TOOL_PATH_INVALID", "setup tool", error.to_string()))
}

fn trusted_path<'a>(
    commands: impl IntoIterator<Item = &'a CommandSpec>,
) -> Result<OsString, CoordError> {
    let mut paths = Vec::new();
    for command in commands {
        if let Some(parent) = command.program.path.parent()
            && !paths.iter().any(|path| path == parent)
        {
            paths.push(parent.to_path_buf());
        }
    }
    let system = PathBuf::from("/usr/bin");
    if !paths.contains(&system) {
        paths.push(system);
    }
    std::env::join_paths(paths).map_err(|error| {
        tool_error(
            "SETUP_TOOL_PATH_INVALID",
            "setup toolchain",
            error.to_string(),
        )
    })
}

pub(super) fn run_git(repo: Option<&Path>, args: &[&OsStr]) -> Result<(), CoordError> {
    admitted_git()?.run_git(repo, args)
}

fn admitted_git() -> Result<&'static CommandSpec, CoordError> {
    if let Some(git) = ADMITTED_GIT.get() {
        return Ok(git);
    }
    let canonical = fs::canonicalize(GIT_BIN).map_err(|error| {
        tool_error(
            "SETUP_TOOL_UNAVAILABLE",
            "Git setup operation",
            format!("{GIT_BIN} cannot be resolved: {error}"),
        )
    })?;
    let candidate = CommandSpec::admit(ToolIdentity::Git, &canonical, Vec::new(), Vec::new())?;
    let _ = ADMITTED_GIT.set(candidate);
    Ok(ADMITTED_GIT
        .get()
        .expect("Git command is initialized after successful admission"))
}
