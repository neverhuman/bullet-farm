//! Prior-or-complete-next setup transaction with a final family commit marker.

use std::{
    fs::{self, OpenOptions},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use super::{CloneTransport, STAGING_PREFIX, checkout_locked_commit};
use crate::{
    checkout::{ensure_regular_file, required_members, verify_family, verify_hub, verify_member},
    coord::CoordError,
    family_lock::{FamilyLock, LockedMember},
};

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Boundary {
    CloneComplete,
    CheckoutComplete,
    CandidateValidated,
    BeforeMemberPublish,
    AfterMemberPublish,
    ManifestFileSynced,
    ManifestLinked,
    ManifestDirectorySynced,
}

pub(super) trait FaultInjector {
    fn reach(&self, boundary: Boundary, member: Option<&str>) -> Result<(), CoordError>;
}

pub(super) struct NoFault;

impl FaultInjector for NoFault {
    fn reach(&self, _boundary: Boundary, _member: Option<&str>) -> Result<(), CoordError> {
        Ok(())
    }
}

#[cfg(test)]
pub(super) struct ExactOnlyValidator;

#[cfg(test)]
impl CandidateValidator for ExactOnlyValidator {
    fn validate(&self, _candidate: &CandidateFamily<'_>) -> Result<(), CoordError> {
        Ok(())
    }
}

struct CandidateMember<'a> {
    lock: &'a LockedMember,
    path: PathBuf,
    parent: PathBuf,
    staged: bool,
}

pub(super) struct CandidateFamily<'a> {
    hub: &'a Path,
    family_root: &'a Path,
    allowed_signers: &'a Path,
    lock: &'a FamilyLock,
    members: Vec<CandidateMember<'a>>,
}

impl CandidateFamily<'_> {
    pub(super) fn path(&self, name: &str) -> Result<&Path, CoordError> {
        if name == "bullet-farm" {
            return Ok(self.hub);
        }
        self.members
            .iter()
            .find(|entry| entry.lock.name == name)
            .map(|entry| entry.path.as_path())
            .ok_or_else(|| {
                CoordError::new(
                    "FAMILY_MEMBER_MISSING",
                    format!("candidate family has no {name}"),
                )
            })
    }

    fn verify_exact(&self, verifier: &dyn FamilyVerifier) -> Result<(), CoordError> {
        verifier.verify_hub(self.lock, self.hub, self.family_root, self.allowed_signers)?;
        for entry in &self.members {
            verifier.verify_member(entry.lock, &entry.path, &entry.parent, self.allowed_signers)?;
        }
        Ok(())
    }
}

pub(super) trait CandidateValidator {
    fn validate(&self, candidate: &CandidateFamily<'_>) -> Result<(), CoordError>;
}

trait FamilyVerifier {
    fn verify_hub(
        &self,
        lock: &FamilyLock,
        hub: &Path,
        family_root: &Path,
        allowed_signers: &Path,
    ) -> Result<(), CoordError>;

    fn verify_member(
        &self,
        member: &LockedMember,
        path: &Path,
        parent: &Path,
        allowed_signers: &Path,
    ) -> Result<(), CoordError>;

    fn verify_family(
        &self,
        family_root: &Path,
        hub: &Path,
        lock: &FamilyLock,
    ) -> Result<(), CoordError>;
}

struct ExactVerifier;

impl FamilyVerifier for ExactVerifier {
    fn verify_hub(
        &self,
        lock: &FamilyLock,
        hub: &Path,
        family_root: &Path,
        allowed_signers: &Path,
    ) -> Result<(), CoordError> {
        verify_hub(lock, hub, family_root, allowed_signers)
    }

    fn verify_member(
        &self,
        member: &LockedMember,
        path: &Path,
        parent: &Path,
        allowed_signers: &Path,
    ) -> Result<(), CoordError> {
        verify_member(member, path, parent, allowed_signers)
    }

    fn verify_family(
        &self,
        family_root: &Path,
        hub: &Path,
        lock: &FamilyLock,
    ) -> Result<(), CoordError> {
        verify_family(family_root, hub, lock)
    }
}

struct Controls<'a> {
    faults: &'a dyn FaultInjector,
    verifier: &'a dyn FamilyVerifier,
}

pub(super) fn install_transaction(
    family_root: &Path,
    hub_root: &Path,
    lock: &FamilyLock,
    offline: bool,
    transport: &dyn CloneTransport,
    validator: &dyn CandidateValidator,
    faults: &dyn FaultInjector,
) -> Result<(), CoordError> {
    install_with_verifier(
        family_root,
        hub_root,
        lock,
        offline,
        transport,
        validator,
        Controls {
            faults,
            verifier: &ExactVerifier,
        },
    )
}

fn install_with_verifier(
    family_root: &Path,
    hub_root: &Path,
    lock: &FamilyLock,
    offline: bool,
    transport: &dyn CloneTransport,
    validator: &dyn CandidateValidator,
    controls: Controls<'_>,
) -> Result<(), CoordError> {
    let Controls { faults, verifier } = controls;
    let required = required_members(hub_root)?;
    lock.validate_required_members(&required)?;
    let allowed_signers = hub_root.join("release/allowed_signers");
    ensure_regular_file(&allowed_signers, "allowed signers")?;
    verifier.verify_hub(lock, hub_root, family_root, &allowed_signers)?;
    let (committed, manifest_bytes) = inspect_manifest(family_root, hub_root)?;
    let staging = (!committed)
        .then(|| Staging::create(family_root))
        .transpose()?;

    let mut members = Vec::with_capacity(lock.member.len());
    for member in &lock.member {
        let target = family_root.join(&member.name);
        match fs::symlink_metadata(&target) {
            Ok(_) => {
                verifier.verify_member(member, &target, family_root, &allowed_signers)?;
                members.push(CandidateMember {
                    lock: member,
                    path: target,
                    parent: family_root.to_path_buf(),
                    staged: false,
                });
            }
            Err(error) if error.kind() == ErrorKind::NotFound && committed => {
                return Err(CoordError::new(
                    "FAMILY_COMMIT_INCOMPLETE",
                    format!(
                        "the durable family manifest covers missing {}; preserve the root and repair from the signed release",
                        member.name
                    ),
                ));
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {
                let stage = staging.as_ref().expect("uncommitted setup has staging");
                let path = stage.path.join(&member.name);
                transport.clone_member(member, &path, offline)?;
                faults.reach(Boundary::CloneComplete, Some(&member.name))?;
                checkout_locked_commit(member, &path)?;
                faults.reach(Boundary::CheckoutComplete, Some(&member.name))?;
                verifier.verify_member(member, &path, &stage.path, &allowed_signers)?;
                members.push(CandidateMember {
                    lock: member,
                    path,
                    parent: stage.path.clone(),
                    staged: true,
                });
            }
            Err(error) => return Err(CoordError::io(error)),
        }
    }

    let candidate = CandidateFamily {
        hub: hub_root,
        family_root,
        allowed_signers: &allowed_signers,
        lock,
        members,
    };
    validator.validate(&candidate)?;
    candidate.verify_exact(verifier)?;
    faults.reach(Boundary::CandidateValidated, None)?;

    if committed {
        return verifier.verify_family(family_root, hub_root, lock);
    }
    for entry in candidate.members.iter().filter(|entry| entry.staged) {
        faults.reach(Boundary::BeforeMemberPublish, Some(&entry.lock.name))?;
        let target = family_root.join(&entry.lock.name);
        publish_checkout_no_replace(&entry.path, &target, &entry.lock.name)?;
        sync_directory(family_root)?;
        faults.reach(Boundary::AfterMemberPublish, Some(&entry.lock.name))?;
    }
    verifier.verify_family(family_root, hub_root, lock)?;
    publish_manifest(
        family_root,
        hub_root,
        &manifest_bytes,
        &staging.expect("uncommitted setup has staging").path,
        faults,
    )?;
    verifier.verify_family(family_root, hub_root, lock)
}

fn inspect_manifest(family_root: &Path, hub_root: &Path) -> Result<(bool, Vec<u8>), CoordError> {
    let source = fs::read(hub_root.join("repos.manifest.toml")).map_err(CoordError::io)?;
    let destination = family_root.join("repos.manifest.toml");
    match fs::symlink_metadata(&destination) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(manifest_conflict(
                    "family-root repos.manifest.toml is not a regular file",
                ));
            }
            if fs::read(&destination).map_err(CoordError::io)? != source {
                return Err(manifest_conflict(
                    "family-root repos.manifest.toml differs from the signed hub manifest",
                ));
            }
            Ok((true, source))
        }
        Err(error) if error.kind() == ErrorKind::NotFound => Ok((false, source)),
        Err(error) => Err(CoordError::io(error)),
    }
}

fn publish_manifest(
    family_root: &Path,
    hub_root: &Path,
    expected: &[u8],
    staging: &Path,
    faults: &dyn FaultInjector,
) -> Result<(), CoordError> {
    let (exists, current) = inspect_manifest(family_root, hub_root)?;
    if exists || current != expected {
        return Err(manifest_conflict(
            "family manifest appeared or changed before final publication",
        ));
    }
    let temporary = staging.join("manifest.tmp");
    let mut guard = TemporaryFile::new(temporary);
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(guard.path())
        .map_err(CoordError::io)?;
    file.write_all(expected)
        .and_then(|()| file.sync_all())
        .map_err(CoordError::io)?;
    faults.reach(Boundary::ManifestFileSynced, None)?;
    let destination = family_root.join("repos.manifest.toml");
    fs::hard_link(guard.path(), &destination).map_err(|error| {
        manifest_conflict(format!("cannot publish without replacement: {error}"))
    })?;
    faults.reach(Boundary::ManifestLinked, None)?;
    sync_directory(family_root)?;
    faults.reach(Boundary::ManifestDirectorySynced, None)?;
    guard.remove()?;
    sync_directory(staging)
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub(super) fn publish_checkout_no_replace(
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
pub(super) fn publish_checkout_no_replace(
    _staged: &Path,
    _target: &Path,
    member: &str,
) -> Result<(), CoordError> {
    Err(CoordError::new(
        "UNSUPPORTED_PLATFORM_CONTAINMENT",
        format!("atomic no-replace publication is unavailable for {member}"),
    ))
}

fn sync_directory(path: &Path) -> Result<(), CoordError> {
    fs::File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(CoordError::io)
}

fn manifest_conflict(reason: impl Into<String>) -> CoordError {
    CoordError::new("FAMILY_MANIFEST_CONFLICT", reason)
}

struct TemporaryFile {
    path: PathBuf,
}

impl TemporaryFile {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn remove(&mut self) -> Result<(), CoordError> {
        fs::remove_file(&self.path).map_err(CoordError::io)?;
        self.path.clear();
        Ok(())
    }
}

impl Drop for TemporaryFile {
    fn drop(&mut self) {
        if !self.path.as_os_str().is_empty() {
            let _ = fs::remove_file(&self.path);
        }
    }
}

struct Staging {
    family_root: PathBuf,
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
                Ok(()) => {
                    return Ok(Self {
                        family_root: family_root.to_path_buf(),
                        path,
                    });
                }
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
        let safe = self.path.parent() == Some(self.family_root.as_path())
            && self
                .path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(STAGING_PREFIX))
            && fs::symlink_metadata(&self.path).is_ok_and(|metadata| {
                metadata.file_type().is_dir() && !metadata.file_type().is_symlink()
            });
        if safe {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}
