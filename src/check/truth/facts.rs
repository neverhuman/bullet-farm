//! Machine facts consumed by the release-truth projection. Every value is read
//! from a file or read-only Git query; none of it is release authority and no
//! wall clock is consulted.

use std::{
    fs,
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

use serde_json::Value;

use crate::{
    check::{executor, model::CHECK_REPORT_SCHEMA_VERSION},
    doctor::{self, LockSummary},
};

pub(super) const REPOSITORIES: &[&str] = &[
    "bullet-farm",
    "bullet-kernel",
    "bullet-git",
    "bullet-portal",
];
pub(super) const RELEASE_INDEX: &str = "docs/release.md";
pub(super) const MECHANICAL_TIERS: &[(&str, &str, &str)] = &[
    ("fast", "FAST", ".bullet-family/check-fast.json"),
    ("required", "REQUIRED", ".bullet-family/check-required.json"),
];
const MAX_INPUT_BYTES: u64 = 1024 * 1024;
const PORTABLE_NOTE: &str = "not read (portable variant)";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::check) enum Variant {
    /// Machine-local subjects, absolute paths, and mtimes are included.
    Live,
    /// Reproducible from a fresh hub-only checkout; machine-local inputs are excluded.
    Portable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Facts {
    pub variant: Variant,
    pub hub_root: Option<String>,
    pub subjects: Vec<SubjectFact>,
    pub hub_head_committed_at: Option<String>,
    pub lock: Result<LockSummary, String>,
    pub release_index: ReleaseIndexFact,
    pub mechanical: Vec<MechanicalFact>,
    pub inputs: Vec<InputFact>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SubjectFact {
    pub name: &'static str,
    pub commit: Option<String>,
    pub tree: Option<String>,
    pub checkout: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct ReleaseIndexFact {
    pub status: Option<String>,
    pub last_reviewed: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct MechanicalFact {
    pub tier: &'static str,
    pub path: &'static str,
    pub outcome: Mechanical,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Mechanical {
    NotRead,
    Absent,
    Unreadable(String),
    Stale {
        status: String,
        gates: usize,
        mismatch: String,
    },
    Fresh {
        status: String,
        gates: usize,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct InputFact {
    pub label: &'static str,
    pub path: &'static str,
    pub identity: String,
    pub mtime: Option<String>,
}

pub(super) fn gather(hub: &Path, variant: Variant) -> Facts {
    let live = variant == Variant::Live;
    let subjects = REPOSITORIES
        .iter()
        .map(|&name| {
            if live {
                subject(name, &repository_path(hub, name))
            } else {
                SubjectFact {
                    name,
                    commit: None,
                    tree: None,
                    checkout: PORTABLE_NOTE.to_owned(),
                }
            }
        })
        .collect::<Vec<_>>();
    let mechanical = MECHANICAL_TIERS
        .iter()
        .map(|&(tier, expected, path)| MechanicalFact {
            tier,
            path,
            outcome: if live {
                mechanical(&hub.join(path), expected, &subjects)
            } else {
                Mechanical::NotRead
            },
        })
        .collect();
    let release_index = release_index(&hub.join(RELEASE_INDEX));
    let mut inputs = vec![
        input("family lock", "family.lock", hub, live),
        input("hub manifest", "repos.manifest.toml", hub, live),
        // Only the `Last reviewed:` line is consumed, so only that line is bound;
        // prose edits elsewhere in the release index never move this page.
        InputFact {
            identity: release_index.last_reviewed.as_deref().map_or_else(
                || "no `Last reviewed:` line".to_owned(),
                |date| format!("`Last reviewed: {date}` line only"),
            ),
            ..input("release index", RELEASE_INDEX, hub, live)
        },
    ];
    for &(tier, _, path) in MECHANICAL_TIERS {
        let label = if tier == "fast" {
            "fast check report"
        } else {
            "required check report"
        };
        inputs.push(input(label, path, hub, live));
    }
    Facts {
        variant,
        hub_root: live.then(|| hub.to_string_lossy().into_owned()),
        hub_head_committed_at: live
            .then(|| executor::git(hub, &["log", "-1", "--format=%cI"]).ok())
            .flatten(),
        subjects,
        lock: doctor::lock_summary(hub).map_err(|error| error.to_string()),
        release_index,
        mechanical,
        inputs,
    }
}

fn repository_path(hub: &Path, name: &str) -> PathBuf {
    if name == "bullet-farm" {
        hub.to_path_buf()
    } else {
        hub.parent()
            .map_or_else(|| hub.join(name), |family| family.join(name))
    }
}

fn subject(name: &'static str, repository: &Path) -> SubjectFact {
    if !repository.join(".git").exists() {
        return SubjectFact {
            name,
            commit: None,
            tree: None,
            checkout: "absent".to_owned(),
        };
    }
    let identity =
        executor::git(repository, &["rev-parse", "--show-object-format"]).and_then(|algorithm| {
            let head = executor::git(repository, &["rev-parse", "--verify", "HEAD"])?;
            let tree = executor::git(repository, &["rev-parse", "--verify", "HEAD^{tree}"])?;
            Ok((format!("{algorithm}:{head}"), format!("{algorithm}:{tree}")))
        });
    let (commit, tree) = match identity {
        Ok((commit, tree)) => (Some(commit), Some(tree)),
        Err(error) => {
            return SubjectFact {
                name,
                commit: None,
                tree: None,
                checkout: format!("UNKNOWN ({error})"),
            };
        }
    };
    let checkout = match executor::git(
        repository,
        &["status", "--porcelain=v2", "--untracked-files=all"],
    ) {
        Ok(status) if status.is_empty() => "clean".to_owned(),
        Ok(status) => format!("dirty ({} entries)", status.lines().count()),
        Err(error) => format!("UNKNOWN ({error})"),
    };
    SubjectFact {
        name,
        commit,
        tree,
        checkout,
    }
}

fn release_index(path: &Path) -> ReleaseIndexFact {
    let Some(text) = read_bounded(path).and_then(|bytes| String::from_utf8(bytes).ok()) else {
        return ReleaseIndexFact::default();
    };
    let field = |prefix: &str| {
        text.lines()
            .find_map(|line| line.strip_prefix(prefix))
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
    };
    ReleaseIndexFact {
        status: field("Status:"),
        last_reviewed: field("Last reviewed:"),
    }
}

fn mechanical(path: &Path, expected_tier: &str, subjects: &[SubjectFact]) -> Mechanical {
    if fs::symlink_metadata(path).is_err() {
        return Mechanical::Absent;
    }
    let Some(bytes) = read_bounded(path) else {
        return Mechanical::Unreadable("not a regular file within 1 MiB".to_owned());
    };
    let report: Value = match serde_json::from_slice(&bytes) {
        Ok(report) => report,
        Err(error) => return Mechanical::Unreadable(error.to_string()),
    };
    if report["schema_version"] != CHECK_REPORT_SCHEMA_VERSION
        || report["command"] != "check"
        || report["tier"] != expected_tier
    {
        return Mechanical::Unreadable(format!(
            "not a schema {CHECK_REPORT_SCHEMA_VERSION} {expected_tier} check report"
        ));
    }
    let (Some(status), Some(gates)) = (report["status"].as_str(), report["gates"].as_array())
    else {
        return Mechanical::Unreadable("status or gates are missing".to_owned());
    };
    let mismatch = gates.iter().find_map(|gate| {
        gate["subjects"].as_array()?.iter().find_map(|subject| {
            let name = subject["repository"].as_str()?;
            let recorded = subject["commit_oid"].as_str()?;
            let live = subjects
                .iter()
                .find(|subject| subject.name == name)
                .and_then(|subject| subject.commit.as_deref());
            (live != Some(recorded)).then(|| {
                format!(
                    "{name} recorded {recorded}, HEAD {}",
                    live.unwrap_or("UNKNOWN")
                )
            })
        })
    });
    match mismatch {
        Some(mismatch) => Mechanical::Stale {
            status: status.to_owned(),
            gates: gates.len(),
            mismatch,
        },
        None => Mechanical::Fresh {
            status: status.to_owned(),
            gates: gates.len(),
        },
    }
}

fn input(label: &'static str, relative: &'static str, hub: &Path, live: bool) -> InputFact {
    let path = hub.join(relative);
    let Some(bytes) = read_bounded(&path) else {
        return InputFact {
            label,
            path: relative,
            identity: "absent".to_owned(),
            mtime: None,
        };
    };
    let mtime = live
        .then(|| {
            fs::metadata(&path)
                .ok()?
                .modified()
                .ok()?
                .duration_since(UNIX_EPOCH)
                .ok()
                .map(|since| since.as_secs().to_string())
        })
        .flatten();
    InputFact {
        label,
        path: relative,
        identity: format!("blake3:{}", blake3::hash(&bytes).to_hex()),
        mtime,
    }
}

fn read_bounded(path: &Path) -> Option<Vec<u8>> {
    let metadata = fs::symlink_metadata(path).ok()?;
    if !metadata.file_type().is_file() || metadata.len() > MAX_INPUT_BYTES {
        return None;
    }
    fs::read(path).ok()
}
