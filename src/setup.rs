//! Fail-closed installation of the exact family described by `family.lock`.

mod args;
mod command;

use std::{
    ffi::OsStr,
    fs::{self, OpenOptions},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    checkout::{
        ensure_regular_directory, ensure_regular_file, required_members, verify_family, verify_hub,
        verify_member,
    },
    coord::CoordError,
    family_lock::{self, FamilyLock, LockedMember},
};
use args::parse_args;
use command::{SetupEnvironment, Toolchain, run_git};

const GIT_BIN: &str = "/usr/bin/git";
const BASH_BIN: &str = "/bin/bash";
const STAGING_PREFIX: &str = ".bullet-family-setup.";

pub fn run(
    current_dir: &Path,
    explicit_root: Option<&str>,
    args: &[String],
) -> Result<String, CoordError> {
    let options = parse_args(explicit_root, args)?;
    require_setup_containment()?;
    let hub_root = crate::doctor::discover_hub(current_dir, None)?;
    let family_root = canonical_family_root(&options.root)?;
    let expected_hub = family_root.join("bullet-farm");
    if expected_hub.canonicalize().map_err(CoordError::io)?
        != hub_root.canonicalize().map_err(CoordError::io)?
    {
        return Err(CoordError::new(
            "HUB_LOCATION_MISMATCH",
            format!(
                "setup root must contain the invoking hub at {}",
                expected_hub.display()
            ),
        ));
    }
    let lock = family_lock::load(&hub_root.join("family.lock"))?;
    let toolchain = Toolchain::admit(
        options.cargo_bin.as_deref(),
        options.node_bin.as_deref(),
        options.npm_cli.as_deref(),
    )?;
    let environment = SetupEnvironment::create(&family_root, &toolchain)?;
    install(
        &family_root,
        &hub_root,
        &lock,
        options.offline,
        &JeryuTransport,
    )?;
    install_dependencies(
        &family_root,
        &hub_root,
        options.offline,
        &toolchain,
        &environment,
    )?;
    verify_family(&family_root, &hub_root, &lock)?;
    Ok(format!(
        "setup complete: {} members at {}",
        lock.member.len() + 1,
        lock.tag
    ))
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
fn require_setup_containment() -> Result<(), CoordError> {
    Ok(())
}

#[cfg(not(all(target_os = "linux", target_env = "gnu")))]
fn require_setup_containment() -> Result<(), CoordError> {
    Err(CoordError::new(
        "UNSUPPORTED_PLATFORM_CONTAINMENT",
        "setup cannot mutate on this platform until process-tree termination and atomic no-replace publication have equivalent release evidence",
    ))
}

fn canonical_family_root(root: &Path) -> Result<PathBuf, CoordError> {
    ensure_regular_directory(root, "setup root")?;
    root.canonicalize().map_err(CoordError::io)
}

trait CloneTransport {
    fn clone_member(
        &self,
        member: &LockedMember,
        destination: &Path,
        offline: bool,
    ) -> Result<(), CoordError>;
}

struct JeryuTransport;

impl CloneTransport for JeryuTransport {
    fn clone_member(
        &self,
        member: &LockedMember,
        destination: &Path,
        offline: bool,
    ) -> Result<(), CoordError> {
        if offline {
            return Err(CoordError::new(
                "OFFLINE_SOURCE_UNAVAILABLE",
                format!(
                    "{} is absent and no verified local object cache is configured",
                    member.name
                ),
            ));
        }
        let source = member.jeryu_url.as_deref().ok_or_else(|| {
            CoordError::new(
                "SOURCE_METADATA_MISSING",
                format!("{} has no authenticated Jeryu URL", member.name),
            )
        })?;
        clone_repository(OsStr::new(source), destination, false)
    }
}

fn install(
    family_root: &Path,
    hub_root: &Path,
    lock: &FamilyLock,
    offline: bool,
    transport: &dyn CloneTransport,
) -> Result<(), CoordError> {
    let required = required_members(hub_root)?;
    lock.validate_required_members(&required)?;
    let allowed_signers = hub_root.join("release/allowed_signers");
    ensure_regular_file(&allowed_signers, "allowed signers")?;
    verify_hub(lock, hub_root, family_root, &allowed_signers)?;
    preflight_manifest(family_root, hub_root)?;

    let mut missing = Vec::new();
    for member in &lock.member {
        let target = family_root.join(&member.name);
        match fs::symlink_metadata(&target) {
            Ok(_) => verify_member(member, &target, family_root, &allowed_signers)?,
            Err(error) if error.kind() == ErrorKind::NotFound => missing.push(member),
            Err(error) => return Err(CoordError::io(error)),
        }
    }
    if missing.is_empty() {
        publish_manifest(family_root, hub_root)?;
        return Ok(());
    }
    let staging = Staging::create(family_root)?;
    for member in &missing {
        let staged = staging.path.join(&member.name);
        transport.clone_member(member, &staged, offline)?;
        checkout_locked_commit(member, &staged)?;
        verify_member(member, &staged, &staging.path, &allowed_signers)?;
    }
    for member in missing {
        let staged = staging.path.join(&member.name);
        let target = family_root.join(&member.name);
        publish_checkout_no_replace(&staged, &target, &member.name)?;
        sync_directory(family_root)?;
    }
    publish_manifest(family_root, hub_root)?;
    Ok(())
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
fn publish_checkout_no_replace(
    staged: &Path,
    target: &Path,
    member: &str,
) -> Result<(), CoordError> {
    use nix::{
        errno::Errno,
        fcntl::{RenameFlags, renameat2},
    };

    renameat2(None, staged, None, target, RenameFlags::RENAME_NOREPLACE).map_err(|error| {
        if error == Errno::EEXIST {
            CoordError::new(
                "CHECKOUT_CONFLICT",
                format!("{member} appeared during setup; the existing path was preserved"),
            )
        } else {
            CoordError::new(
                "CHECKOUT_PUBLICATION_FAILED",
                format!("cannot publish {member} without replacement: {error}"),
            )
        }
    })
}

#[cfg(not(all(target_os = "linux", target_env = "gnu")))]
fn publish_checkout_no_replace(
    _staged: &Path,
    _target: &Path,
    member: &str,
) -> Result<(), CoordError> {
    Err(CoordError::new(
        "UNSUPPORTED_PLATFORM_CONTAINMENT",
        format!(
            "atomic no-replace publication is unavailable for {member} on this platform; no checkout was published"
        ),
    ))
}

fn preflight_manifest(family_root: &Path, hub_root: &Path) -> Result<(), CoordError> {
    let source = fs::read(hub_root.join("repos.manifest.toml")).map_err(CoordError::io)?;
    let destination = family_root.join("repos.manifest.toml");
    match fs::symlink_metadata(&destination) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(CoordError::new(
                    "FAMILY_MANIFEST_CONFLICT",
                    "family-root repos.manifest.toml is not a regular file",
                ));
            }
            if fs::read(&destination).map_err(CoordError::io)? != source {
                return Err(CoordError::new(
                    "FAMILY_MANIFEST_CONFLICT",
                    "family-root repos.manifest.toml differs from the signed hub manifest",
                ));
            }
        }
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(CoordError::io(error)),
    }
    Ok(())
}

fn publish_manifest(family_root: &Path, hub_root: &Path) -> Result<(), CoordError> {
    preflight_manifest(family_root, hub_root)?;
    let destination = family_root.join("repos.manifest.toml");
    if destination.exists() {
        return Ok(());
    }
    let bytes = fs::read(hub_root.join("repos.manifest.toml")).map_err(CoordError::io)?;
    let temporary = family_root.join(format!(
        "{STAGING_PREFIX}manifest.{}.tmp",
        std::process::id()
    ));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temporary)
        .map_err(CoordError::io)?;
    if let Err(error) = file.write_all(&bytes).and_then(|()| file.sync_all()) {
        let _ = fs::remove_file(&temporary);
        return Err(CoordError::io(error));
    }
    if let Err(error) = fs::hard_link(&temporary, &destination) {
        let _ = fs::remove_file(&temporary);
        return Err(CoordError::new(
            "FAMILY_MANIFEST_PUBLICATION_FAILED",
            error.to_string(),
        ));
    }
    sync_directory(family_root)?;
    fs::remove_file(&temporary).map_err(CoordError::io)?;
    sync_directory(family_root)
}

fn checkout_locked_commit(member: &LockedMember, repo: &Path) -> Result<(), CoordError> {
    let commit = member
        .commit_oid
        .split_once(':')
        .map(|(_, oid)| oid)
        .ok_or_else(|| CoordError::new("INVALID_FAMILY_LOCK", "commit OID lacks algorithm"))?;
    run_git(
        Some(repo),
        &[
            OsStr::new("-c"),
            OsStr::new("core.hooksPath=/dev/null"),
            OsStr::new("-c"),
            OsStr::new("core.autocrlf=false"),
            OsStr::new("checkout"),
            OsStr::new("--detach"),
            OsStr::new("--force"),
            OsStr::new(commit),
        ],
    )
}

fn clone_repository(
    source: &OsStr,
    destination: &Path,
    permit_local: bool,
) -> Result<(), CoordError> {
    let local_policy = if permit_local { "always" } else { "never" };
    run_git(
        None,
        &[
            OsStr::new("-c"),
            OsStr::new(if permit_local {
                "protocol.file.allow=always"
            } else {
                "protocol.file.allow=never"
            }),
            OsStr::new("clone"),
            OsStr::new("--no-checkout"),
            OsStr::new("--no-local"),
            OsStr::new("--config"),
            OsStr::new("core.hooksPath=/dev/null"),
            OsStr::new("--config"),
            OsStr::new("core.autocrlf=false"),
            OsStr::new(source),
            destination.as_os_str(),
        ],
    )
    .map_err(|_| {
        CoordError::new(
            "SOURCE_CLONE_FAILED",
            format!("Git refused the locked source (file protocol {local_policy})"),
        )
    })
}

fn install_dependencies(
    family_root: &Path,
    hub_root: &Path,
    offline: bool,
    toolchain: &Toolchain,
    environment: &SetupEnvironment,
) -> Result<(), CoordError> {
    for repo in [
        hub_root.to_path_buf(),
        family_root.join("bullet-kernel"),
        family_root.join("bullet-git"),
    ] {
        let mut args = vec!["fetch", "--locked"];
        if offline {
            args.push("--offline");
        }
        toolchain.run_cargo(&repo, &args, environment)?;
    }
    let portal = family_root.join("bullet-portal");
    let mut npm_args = vec!["ci", "--ignore-scripts", "--no-audit", "--no-fund"];
    if offline {
        npm_args.push("--offline");
    }
    toolchain.run_npm(&portal, &npm_args, environment)?;
    toolchain.run_cargo(
        hub_root,
        &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "bullet-wire",
            "--bin",
            "bullet-contract",
            "--",
            "check",
            "--root",
            ".",
        ],
        environment,
    )?;
    toolchain.run_cargo(
        &family_root.join("bullet-kernel"),
        &[
            "run",
            "--locked",
            "--quiet",
            "-p",
            "bullet",
            "--",
            "contracts",
            "check",
        ],
        environment,
    )?;
    toolchain.run_bash(
        hub_root,
        &["scripts/sync-family-contracts.sh", "check"],
        environment,
    )
}

fn sync_directory(path: &Path) -> Result<(), CoordError> {
    fs::File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(CoordError::io)
}

struct Staging {
    path: PathBuf,
}

impl Staging {
    fn create(family_root: &Path) -> Result<Self, CoordError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| CoordError::new("CLOCK_BEFORE_EPOCH", error.to_string()))?
            .as_nanos();
        for sequence in 0..64 {
            let path = family_root.join(format!(
                "{STAGING_PREFIX}{}.{}.{}",
                std::process::id(),
                now,
                sequence
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self { path }),
                Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(CoordError::io(error)),
            }
        }
        Err(CoordError::new(
            "STAGING_COLLISION",
            "could not allocate a unique setup staging directory",
        ))
    }
}

impl Drop for Staging {
    fn drop(&mut self) {
        let safe_name = self
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with(STAGING_PREFIX));
        if safe_name
            && fs::symlink_metadata(&self.path).is_ok_and(|metadata| {
                metadata.file_type().is_dir() && !metadata.file_type().is_symlink()
            })
        {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

#[cfg(test)]
mod tests;
