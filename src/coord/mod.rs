mod git;
mod model;
mod receipt_state;
mod state;
mod store;

use std::{
    ffi::OsString,
    fmt,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub use model::{ClaimState, ClaimSummary, DEFAULT_TTL_SECONDS, Status};
pub use store::CoordStore;

#[derive(Debug)]
pub struct CoordError {
    code: &'static str,
    reason: String,
}

impl CoordError {
    pub fn new(code: &'static str, reason: impl Into<String>) -> Self {
        Self {
            code,
            reason: reason.into(),
        }
    }

    pub fn io(error: std::io::Error) -> Self {
        Self::new("COORD_IO_FAILED", error.to_string())
    }

    pub fn json(error: serde_json::Error) -> Self {
        Self::new("COORD_JSON_FAILED", error.to_string())
    }

    pub const fn code(&self) -> &'static str {
        self.code
    }

    pub fn exit_code(&self) -> u8 {
        match self.code {
            "CLAIM_OVERLAP" | "CLAIM_NOT_ACTIVE" | "CLAIM_OWNER_MISMATCH" => 3,
            "CORRUPT_COORD_LOG" | "UNSUPPORTED_SCHEMA" | "PARTIAL_COORD_WRITE" => 4,
            _ => 2,
        }
    }
}

impl fmt::Display for CoordError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.reason)
    }
}

impl std::error::Error for CoordError {}

#[derive(Debug)]
pub struct ClaimInput {
    pub agent: String,
    pub lane: String,
    pub repo: String,
    pub paths: Vec<String>,
    pub ttl_seconds: u64,
}

#[derive(Debug)]
pub struct HeartbeatInput {
    pub claim_id: String,
    pub agent: String,
    pub ttl_seconds: u64,
    pub note: Option<String>,
}

#[derive(Debug)]
pub struct HandoffInput {
    pub claim_id: String,
    pub agent: String,
    pub proof_command: String,
    pub proof_exit_code: i32,
    pub changed_paths: Vec<String>,
    pub commit_oid: Option<String>,
}

#[derive(Debug)]
pub struct CommitReceiptInput {
    pub claim_id: String,
    pub orchestrator: String,
    pub commit_oid: String,
    pub committed_paths: Vec<String>,
}

#[derive(Debug)]
pub struct ReceiptCorrectionInput {
    pub claim_id: String,
    pub orchestrator: String,
    pub previous_commit_oid: String,
    pub commit_oid: String,
    pub committed_paths: Vec<String>,
    pub reason: String,
}

#[derive(Debug)]
pub struct CommitReceiptGroupInput {
    pub claim_ids: Vec<String>,
    pub orchestrator: String,
    pub commit_oid: String,
}

#[derive(Debug)]
pub struct GroupReceiptCorrectionInput {
    pub claim_ids: Vec<String>,
    pub orchestrator: String,
    pub previous_commit_oid: String,
    pub commit_oid: String,
    pub reason: String,
}

pub fn validate_repo_name(repo: &str) -> Result<(), CoordError> {
    validate_field("repo", repo)?;
    if repo == "."
        || !repo
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(CoordError::new(
            "INVALID_REPO",
            "repository name must contain lowercase ASCII letters, digits, or hyphens",
        ));
    }
    Ok(())
}

pub fn discover_family_root(
    start: &Path,
    explicit: Option<OsString>,
) -> Result<PathBuf, CoordError> {
    if let Some(root) = explicit {
        let root = PathBuf::from(root);
        return verify_root(&root);
    }
    let start = start.canonicalize().map_err(CoordError::io)?;
    let mut discovered = None;
    for ancestor in start.ancestors() {
        if ancestor.join("repos.manifest.toml").is_file() {
            discovered = Some(ancestor.to_path_buf());
        }
    }
    discovered.ok_or_else(|| {
        CoordError::new(
            "FAMILY_ROOT_NOT_FOUND",
            "no ancestor contains repos.manifest.toml; pass --root <path>",
        )
    })
}

pub fn unix_millis() -> Result<u64, CoordError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| {
            CoordError::new(
                "CLOCK_BEFORE_EPOCH",
                format!("system clock failed: {error}"),
            )
        })?;
    u64::try_from(duration.as_millis())
        .map_err(|_| CoordError::new("CLOCK_OVERFLOW", "system time does not fit u64"))
}

pub fn validate_path(raw: &str) -> Result<String, CoordError> {
    validate_field("path", raw)?;
    if raw == "." {
        return Ok(raw.to_owned());
    }
    if raw.starts_with('/') || raw.contains('\\') {
        return Err(CoordError::new(
            "INVALID_PATH",
            format!("path must be repository-relative: {raw}"),
        ));
    }
    if raw
        .split('/')
        .any(|segment| segment.is_empty() || segment == "." || segment == "..")
    {
        return Err(CoordError::new(
            "INVALID_PATH",
            format!("path has an empty or dot segment: {raw}"),
        ));
    }
    Ok(raw.trim_end_matches('/').to_owned())
}

pub fn validate_field(name: &str, value: &str) -> Result<(), CoordError> {
    if value.is_empty() || value.len() > 1_024 || value.chars().any(char::is_control) {
        return Err(CoordError::new(
            "INVALID_FIELD",
            format!("{name} must contain 1..=1024 non-control UTF-8 bytes"),
        ));
    }
    Ok(())
}

pub fn validate_ttl(ttl_seconds: u64) -> Result<(), CoordError> {
    if !(30..=86_400).contains(&ttl_seconds) {
        return Err(CoordError::new(
            "INVALID_TTL",
            "TTL must be between 30 and 86400 seconds",
        ));
    }
    Ok(())
}

pub fn validate_commit_oid(oid: &str) -> Result<(), CoordError> {
    if !matches!(oid.len(), 40 | 64)
        || !oid
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(CoordError::new(
            "INVALID_COMMIT_OID",
            "commit OID must be 40 or 64 lowercase hexadecimal characters",
        ));
    }
    Ok(())
}

fn verify_root(root: &Path) -> Result<PathBuf, CoordError> {
    let root = root.canonicalize().map_err(CoordError::io)?;
    if !root.join("repos.manifest.toml").is_file() {
        return Err(CoordError::new(
            "INVALID_ROOT",
            format!("{} has no repos.manifest.toml", root.display()),
        ));
    }
    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::{validate_commit_oid, validate_path};

    #[test]
    fn validates_repository_relative_paths() {
        assert_eq!(validate_path("src/main.rs").unwrap(), "src/main.rs");
        assert_eq!(validate_path(".").unwrap(), ".");
        for denied in ["/etc/passwd", "../src", "src/../main", "src\\main"] {
            assert!(validate_path(denied).is_err(), "accepted {denied}");
        }
    }

    #[test]
    fn validates_commit_oids() {
        assert!(validate_commit_oid(&"a".repeat(40)).is_ok());
        assert!(validate_commit_oid(&"f".repeat(64)).is_ok());
        assert!(validate_commit_oid(&"A".repeat(40)).is_err());
    }
}
