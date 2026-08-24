use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(super) enum CheckStatus {
    Pass,
    Blocked,
}

#[derive(Debug, Serialize)]
pub(super) struct DoctorCheck {
    pub(super) id: &'static str,
    pub(super) status: CheckStatus,
    pub(super) detail: String,
    pub(super) repair: Option<String>,
}

impl DoctorCheck {
    pub(super) fn pass(id: &'static str, detail: impl Into<String>) -> Self {
        Self {
            id,
            status: CheckStatus::Pass,
            detail: detail.into(),
            repair: None,
        }
    }

    pub(super) fn blocked(
        id: &'static str,
        detail: impl Into<String>,
        repair: impl Into<String>,
    ) -> Self {
        Self {
            id,
            status: CheckStatus::Blocked,
            detail: detail.into(),
            repair: Some(repair.into()),
        }
    }
}

#[derive(Debug, Serialize)]
pub(super) struct DoctorReport {
    pub(super) schema_version: u32,
    pub(super) command: &'static str,
    pub(super) status: &'static str,
    pub(super) hub_root: String,
    pub(super) family_root: Option<String>,
    pub(super) checks: Vec<DoctorCheck>,
}

#[derive(Debug, Deserialize)]
pub(super) struct FamilyLock {
    pub(super) schema_version: String,
    pub(super) member: Vec<LockedMember>,
}

#[derive(Debug, Deserialize)]
pub(super) struct LockedMember {
    pub(super) name: String,
    pub(super) commit_oid: String,
    #[serde(default)]
    pub(super) jeryu_url: Option<String>,
    #[serde(default)]
    pub(super) source_url: Option<String>,
    #[serde(default)]
    pub(super) jeryu_slug: Option<String>,
}
