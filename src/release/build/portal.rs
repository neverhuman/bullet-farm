//! Portal build from the exact committed subject, never from a tracked dist.

use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

use serde::Deserialize;

use super::{BuildPlan, cargo::RecordedCommand, failed, invalid, subject::MemberSubject};
use crate::{
    coord::CoordError,
    process::{Limits, run_bounded},
};

const MANIFEST_NAME: &str = ".bullet-portal-bundle-v1.json";
const MAX_MANIFEST_BYTES: u64 = 2 * 1024 * 1024;
const NPM_LIMITS: Limits = Limits {
    timeout: Duration::from_secs(3600),
    stdout_bytes: 32 * 1024 * 1024,
    stderr_bytes: 32 * 1024 * 1024,
};

/// The verified Portal bundle this build embedded.
pub(super) struct PortalOutput {
    pub(super) dist: PathBuf,
    pub(super) root: String,
    pub(super) manifest: PortalManifest,
    pub(super) package_lock: Vec<u8>,
}

#[derive(Deserialize)]
pub(super) struct PortalManifest {
    pub(super) schema_version: String,
    pub(super) source: PortalSource,
    pub(super) package_lock: PortalLock,
    pub(super) files: Vec<PortalFile>,
    pub(super) total_size: u64,
    pub(super) root: String,
}

#[derive(Deserialize)]
pub(super) struct PortalSource {
    pub(super) repository: String,
    pub(super) commit_oid: String,
    pub(super) tree_oid: String,
}

#[derive(Deserialize)]
pub(super) struct PortalLock {
    pub(super) path: String,
    pub(super) size: u64,
    pub(super) blake3: String,
}

#[derive(Deserialize)]
pub(super) struct PortalFile {
    pub(super) path: String,
    pub(super) size: u64,
    pub(super) blake3: String,
}

/// Clones the committed Portal subject into the build scratch directory, builds
/// it there, and generates and re-checks its own bundle manifest. No tracked
/// checkout is written to and no committed `dist/` is ever created.
pub(super) fn build(
    plan: &BuildPlan,
    commands: &mut Vec<RecordedCommand>,
) -> Result<PortalOutput, CoordError> {
    let subject = plan.member("bullet-portal")?;
    let root = plan.scratch.join("bullet-portal");
    if std::fs::symlink_metadata(&root).is_ok() {
        return Err(CoordError::new(
            "RELEASE_OUTPUT_EXISTS",
            format!("{} already exists", root.display()),
        ));
    }
    clone(plan, subject, &root, commands)?;
    npm(plan, &root, &npm_install_args(plan), commands)?;
    npm(
        plan,
        &root,
        &["run".to_owned(), "build".to_owned()],
        commands,
    )?;
    npm(
        plan,
        &root,
        &["run".to_owned(), "bundle:generate".to_owned()],
        commands,
    )?;
    npm(
        plan,
        &root,
        &["run".to_owned(), "bundle:check".to_owned()],
        commands,
    )?;
    let dist = root.join("dist");
    let manifest = read_manifest(&dist)?;
    for digest in std::iter::once(&manifest.root)
        .chain(std::iter::once(&manifest.package_lock.blake3))
        .chain(manifest.files.iter().map(|file| &file.blake3))
    {
        crate::release::schema::validate_digest(digest)?;
    }
    if manifest.schema_version != "bullet.portal.bundle.v1" {
        return Err(CoordError::new(
            "UNSUPPORTED_SCHEMA",
            format!(
                "Portal bundle manifest schema {} is unsupported",
                manifest.schema_version
            ),
        ));
    }
    if manifest.source.repository != "bullet-portal"
        || manifest.source.commit_oid != subject.commit_oid
        || manifest.source.tree_oid != subject.tree_oid
    {
        return Err(CoordError::new(
            "RELEASE_PORTAL_BUNDLE_INVALID",
            "the Portal bundle manifest binds a different Git subject than the admitted member",
        ));
    }
    verify_manifest_files(&dist, &manifest)?;
    let package_lock = std::fs::read(root.join("package-lock.json")).map_err(CoordError::io)?;
    if super::digest_bytes(&package_lock) != manifest.package_lock.blake3
        || package_lock.len() as u64 != manifest.package_lock.size
        || manifest.package_lock.path != "package-lock.json"
    {
        return Err(CoordError::new(
            "RELEASE_PORTAL_BUNDLE_INVALID",
            "the Portal bundle manifest does not bind the exact package-lock.json bytes",
        ));
    }
    Ok(PortalOutput {
        dist,
        root: manifest.root.clone(),
        manifest,
        package_lock,
    })
}

/// Re-reads every emitted file and refuses any manifest that disagrees with the
/// bytes on disk, before those bytes reach the Rust embedding build script.
fn verify_manifest_files(dist: &Path, manifest: &PortalManifest) -> Result<(), CoordError> {
    if manifest.files.is_empty() || manifest.files.len() > 2048 {
        return Err(CoordError::new(
            "RELEASE_PORTAL_BUNDLE_INVALID",
            "the Portal bundle manifest names no files or exceeds its entry bound",
        ));
    }
    let mut total = 0_u64;
    for file in &manifest.files {
        let path = admit_relative(dist, &file.path)?;
        let (digest, size) = super::digest_path(&path)?;
        if size != file.size || digest != file.blake3 {
            return Err(CoordError::new(
                "RELEASE_PORTAL_BUNDLE_INVALID",
                format!("{} differs from the Portal bundle manifest", file.path),
            ));
        }
        total = total
            .checked_add(size)
            .ok_or_else(|| invalid("Portal bundle byte total overflowed"))?;
    }
    if total != manifest.total_size {
        return Err(CoordError::new(
            "RELEASE_PORTAL_BUNDLE_INVALID",
            "the Portal bundle manifest total size differs from its own file records",
        ));
    }
    Ok(())
}

fn admit_relative(dist: &Path, relative: &str) -> Result<PathBuf, CoordError> {
    if relative.is_empty()
        || relative.len() > 240
        || !relative.is_ascii()
        || relative.starts_with('/')
        || relative.contains('\\')
        || relative.split('/').any(|segment| {
            segment.is_empty() || matches!(segment, "." | "..") || segment.starts_with('.')
        })
    {
        return Err(CoordError::new(
            "RELEASE_PORTAL_BUNDLE_INVALID",
            format!("unsafe Portal bundle path {relative:?}"),
        ));
    }
    let path = dist.join(relative);
    let metadata = std::fs::symlink_metadata(&path).map_err(CoordError::io)?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
        return Err(CoordError::new(
            "RELEASE_PORTAL_BUNDLE_INVALID",
            format!("{relative} is not a regular non-symlink file"),
        ));
    }
    Ok(path)
}

fn read_manifest(dist: &Path) -> Result<PortalManifest, CoordError> {
    let path = dist.join(MANIFEST_NAME);
    let metadata = std::fs::symlink_metadata(&path).map_err(CoordError::io)?;
    if !metadata.file_type().is_file() || metadata.len() == 0 || metadata.len() > MAX_MANIFEST_BYTES
    {
        return Err(CoordError::new(
            "RELEASE_PORTAL_BUNDLE_INVALID",
            "the Portal bundle manifest is missing or outside its byte bound",
        ));
    }
    let bytes = std::fs::read(&path).map_err(CoordError::io)?;
    serde_json::from_slice(&bytes).map_err(|error| {
        CoordError::new(
            "RELEASE_PORTAL_BUNDLE_INVALID",
            format!("invalid Portal bundle manifest: {error}"),
        )
    })
}

fn npm_install_args(plan: &BuildPlan) -> Vec<String> {
    let mut args = vec![
        "ci".to_owned(),
        "--ignore-scripts".to_owned(),
        "--no-audit".to_owned(),
        "--no-fund".to_owned(),
    ];
    args.push(if plan.offline {
        "--offline".to_owned()
    } else {
        "--prefer-offline".to_owned()
    });
    args
}

fn clone(
    plan: &BuildPlan,
    subject: &MemberSubject,
    root: &Path,
    commands: &mut Vec<RecordedCommand>,
) -> Result<(), CoordError> {
    let source = subject
        .path
        .to_str()
        .ok_or_else(|| failed("the Portal checkout path is not UTF-8"))?;
    let destination = root
        .to_str()
        .ok_or_else(|| failed("the Portal scratch path is not UTF-8"))?;
    let commit = subject
        .commit_oid
        .split_once(':')
        .ok_or_else(|| invalid("the Portal commit OID is not algorithm-tagged"))?
        .1;
    super::subject::git_bytes(
        &plan.tools,
        &plan.family_root,
        &["clone", "--no-hardlinks", "--quiet", source, destination],
    )?;
    super::subject::git_bytes(
        &plan.tools,
        root,
        &["checkout", "--quiet", "--detach", commit],
    )?;
    for args in [
        vec![
            "clone".to_owned(),
            "--no-hardlinks".to_owned(),
            "--quiet".to_owned(),
            source.to_owned(),
            destination.to_owned(),
        ],
        vec![
            "checkout".to_owned(),
            "--quiet".to_owned(),
            "--detach".to_owned(),
            commit.to_owned(),
        ],
    ] {
        commands.push(RecordedCommand {
            program: plan.tools.git.display().to_string(),
            args,
            cwd: plan.family_root.display().to_string(),
            env: Vec::new(),
        });
    }
    Ok(())
}

fn npm(
    plan: &BuildPlan,
    root: &Path,
    args: &[String],
    commands: &mut Vec<RecordedCommand>,
) -> Result<(), CoordError> {
    let tools = &plan.tools;
    let node_bin = tools
        .node
        .parent()
        .ok_or_else(|| failed("the admitted node has no parent directory"))?;
    let mut path = OsString::from(node_bin);
    path.push(":/usr/bin:/bin");
    let mut env = vec![
        (
            "PATH".to_owned(),
            path.to_str()
                .ok_or_else(|| failed("the Portal build PATH is not UTF-8"))?
                .to_owned(),
        ),
        ("LC_ALL".to_owned(), "C".to_owned()),
        ("CI".to_owned(), "1".to_owned()),
        (
            "npm_config_cache".to_owned(),
            plan.cache
                .join("npm")
                .to_str()
                .ok_or_else(|| failed("the npm cache path is not UTF-8"))?
                .to_owned(),
        ),
    ];
    if let Some(home) = std::env::var_os("HOME").and_then(|value| value.into_string().ok()) {
        env.push(("HOME".to_owned(), home));
    }
    env.sort();
    let mut command = Command::new(&tools.npm);
    command.args(args).current_dir(root).env_clear();
    for (name, value) in &env {
        command.env(name, value);
    }
    commands.push(RecordedCommand {
        program: tools.npm.display().to_string(),
        args: args.to_vec(),
        cwd: root.display().to_string(),
        env: env.clone(),
    });
    let output = run_bounded(&mut command, "release build npm", NPM_LIMITS)?;
    if output.status.success() {
        return Ok(());
    }
    Err(CoordError::new(
        "RELEASE_PORTAL_BUNDLE_INVALID",
        format!(
            "npm {} exited {:?}: {}",
            args.join(" "),
            output.status.code(),
            super::cargo::tail(&[output.stderr.as_slice(), output.stdout.as_slice()].concat())
        ),
    ))
}
