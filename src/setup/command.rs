//! Admitted, bounded external commands used by setup.

mod environment;

#[cfg(test)]
mod tests;

use std::{
    ffi::{OsStr, OsString},
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

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
        let npm = CommandSpec::from_admitted(
            ToolIdentity::Npm,
            node,
            vec![npm_cli.path.as_os_str().to_owned()],
            vec![npm_cli],
        )?;
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
            Command::new(&program.path)
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
        self.program.verify()?;
        for companion in &self.companions {
            companion.verify()?;
        }
        let mut command = Command::new(&self.program.path);
        command.current_dir(repo).args(&self.prefix_args).args(args);
        environment.apply(&mut command);
        let output = run_bounded(&mut command, self.identity.label(), TOOL_LIMITS)?;
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
}

#[derive(Clone, Copy, Debug)]
enum ToolIdentity {
    Cargo,
    Node,
    Npm,
    Bash,
}

impl ToolIdentity {
    const fn label(self) -> &'static str {
        match self {
            Self::Cargo => "Cargo setup operation",
            Self::Node => "Node setup runtime",
            Self::Npm => "npm setup operation",
            Self::Bash => "Bash setup operation",
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

#[derive(Debug)]
struct AdmittedFile {
    label: &'static str,
    path: PathBuf,
    fingerprint: [u8; 32],
    executable: bool,
}

impl AdmittedFile {
    fn admit(label: &'static str, path: &Path, executable: bool) -> Result<Self, CoordError> {
        if !path.is_absolute() {
            return Err(tool_error(
                "SETUP_TOOL_PATH_NOT_ABSOLUTE",
                label,
                "path must be absolute",
            ));
        }
        let canonical = fs::canonicalize(path).map_err(|error| {
            tool_error(
                "SETUP_TOOL_UNAVAILABLE",
                label,
                format!("{} cannot be resolved: {error}", path.display()),
            )
        })?;
        if canonical != path {
            return Err(tool_error(
                "SETUP_TOOL_PATH_NOT_CANONICAL",
                label,
                format!("use the canonical path {}", canonical.display()),
            ));
        }
        let fingerprint = file_fingerprint(label, &canonical, executable)?;
        Ok(Self {
            label,
            path: canonical,
            fingerprint,
            executable,
        })
    }

    fn verify(&self) -> Result<(), CoordError> {
        let actual = file_fingerprint(self.label, &self.path, self.executable)?;
        if actual != self.fingerprint {
            return Err(tool_error(
                "SETUP_TOOL_CHANGED",
                self.label,
                "file bytes changed after admission",
            ));
        }
        Ok(())
    }
}

fn file_fingerprint(
    label: &'static str,
    path: &Path,
    executable: bool,
) -> Result<[u8; 32], CoordError> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        tool_error(
            "SETUP_TOOL_UNAVAILABLE",
            label,
            format!("{} cannot be inspected: {error}", path.display()),
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
        return Err(tool_error(
            "SETUP_TOOL_NOT_REGULAR",
            label,
            "path must be a non-symlink regular file",
        ));
    }
    if metadata.len() > MAX_TOOL_BYTES {
        return Err(tool_error(
            "SETUP_TOOL_TOO_LARGE",
            label,
            "file exceeds the 512 MiB admission limit",
        ));
    }
    #[cfg(unix)]
    if executable && metadata.permissions().mode() & 0o111 == 0 {
        return Err(tool_error(
            "SETUP_TOOL_NOT_EXECUTABLE",
            label,
            "file has no executable mode bit",
        ));
    }
    let mut file = File::open(path).map_err(|error| {
        tool_error(
            "SETUP_TOOL_UNAVAILABLE",
            label,
            format!("{} cannot be opened: {error}", path.display()),
        )
    })?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer).map_err(|error| {
            tool_error(
                "SETUP_TOOL_UNAVAILABLE",
                label,
                format!("{} cannot be read: {error}", path.display()),
            )
        })?;
        if count == 0 {
            return Ok(*hasher.finalize().as_bytes());
        }
        hasher.update(&buffer[..count]);
    }
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
