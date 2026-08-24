//! Read-only onboarding diagnostics that work from a hub-only clone.

mod checks;
mod discovery;
mod model;

use std::path::Path;

use crate::coord::CoordError;
use checks::{check_family_layout, check_hub_checkout, check_source_metadata, check_tools};
use discovery::{discover_hub as resolve_hub, read_lock};
use model::{CheckStatus, DoctorReport};

const USAGE: &str = "usage: bullet-family [--root PATH] doctor --json";

pub fn run(
    current_dir: &Path,
    explicit_root: Option<&str>,
    args: &[String],
) -> Result<String, CoordError> {
    if args != ["--json"] {
        return Err(CoordError::new("USAGE", USAGE));
    }
    let hub_root = discover_hub(current_dir, explicit_root)?;
    let family_root = hub_root
        .parent()
        .filter(|parent| parent.join("repos.manifest.toml").is_file())
        .map(Path::to_path_buf);
    let lock = read_lock(&hub_root)?;
    let mut checks = vec![check_hub_checkout(&hub_root), check_tools()];
    checks.push(check_source_metadata(&lock));
    checks.extend(check_family_layout(
        &hub_root,
        family_root.as_deref(),
        &lock,
    ));
    let status = if checks
        .iter()
        .any(|check| check.status == CheckStatus::Blocked)
    {
        "BLOCKED"
    } else {
        "READY"
    };
    serde_json::to_string_pretty(&DoctorReport {
        schema_version: 1,
        command: "doctor",
        status,
        hub_root: display_path(&hub_root),
        family_root: family_root.as_deref().map(display_path),
        checks,
    })
    .map_err(CoordError::json)
}

pub(crate) fn discover_hub(
    current_dir: &Path,
    explicit_root: Option<&str>,
) -> Result<std::path::PathBuf, CoordError> {
    resolve_hub(current_dir, explicit_root)
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}
