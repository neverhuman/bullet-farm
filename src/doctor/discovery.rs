use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use crate::coord::CoordError;

use super::model::FamilyLock;

pub(super) fn discover_hub(
    current_dir: &Path,
    explicit_root: Option<&str>,
) -> Result<PathBuf, CoordError> {
    let start = explicit_root.map_or_else(|| current_dir.to_path_buf(), PathBuf::from);
    let start = start.canonicalize().map_err(CoordError::io)?;
    for ancestor in start.ancestors() {
        if is_hub(ancestor) {
            return Ok(ancestor.to_path_buf());
        }
        let child = ancestor.join("bullet-farm");
        if is_hub(&child) {
            return child.canonicalize().map_err(CoordError::io);
        }
        if explicit_root.is_some() {
            break;
        }
    }
    Err(CoordError::new(
        "HUB_CHECKOUT_NOT_FOUND",
        format!(
            "{} is not inside a Bullet Farm hub checkout",
            start.display()
        ),
    ))
}

pub(super) fn read_lock(hub_root: &Path) -> Result<FamilyLock, CoordError> {
    let path = hub_root.join("family.lock");
    let text = fs::read_to_string(&path).map_err(CoordError::io)?;
    let lock: FamilyLock = toml::from_str(&text)
        .map_err(|error| CoordError::new("INVALID_FAMILY_LOCK", error.to_string()))?;
    if lock.schema_version.is_empty() || lock.member.is_empty() {
        return Err(CoordError::new(
            "INVALID_FAMILY_LOCK",
            "family.lock needs a schema version and at least one member",
        ));
    }
    let mut names = BTreeSet::new();
    for member in &lock.member {
        if member.name.is_empty() || !names.insert(member.name.as_str()) {
            return Err(CoordError::new(
                "INVALID_FAMILY_LOCK",
                "family.lock member names must be non-empty and unique",
            ));
        }
        if !valid_oid(&member.commit_oid) {
            return Err(CoordError::new(
                "INVALID_FAMILY_LOCK",
                format!("{} has an invalid commit OID", member.name),
            ));
        }
    }
    Ok(lock)
}

fn is_hub(path: &Path) -> bool {
    path.join("Cargo.toml").is_file()
        && path.join("family.lock").is_file()
        && path.join("scripts/setup.sh").is_file()
}

fn valid_oid(oid: &str) -> bool {
    matches!(oid.len(), 40 | 64)
        && oid
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
