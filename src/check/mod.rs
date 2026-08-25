//! Rust-owned, exact-subject local check catalogs.

mod catalog;
mod executor;
mod prerequisites;
mod release_evidence;
mod subject;
mod truth;

pub mod model;

use crate::coord::CoordError;
use model::{CheckReport, CheckTier};
use std::path::Path;

const USAGE: &str = "usage: bullet-family [--root PATH] check <fast|required|release> [--json] | check release --report [--portable]";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OutputMode {
    Human,
    Json,
    Report(truth::Variant),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckExecution {
    report: CheckReport,
    mode: OutputMode,
    /// Rendered release truth page; present only for `--report`.
    page: Option<String>,
}

impl CheckExecution {
    pub const fn exit_code(&self) -> u8 {
        self.report.exit_code()
    }

    pub fn output(&self) -> Result<String, CoordError> {
        match (self.mode, &self.page) {
            (OutputMode::Json, _) => self.report.stable_json().map_err(CoordError::json),
            (OutputMode::Report(_), Some(page)) => Ok(page.clone()),
            (OutputMode::Report(_), None) => Err(CoordError::new(
                "RELEASE_TRUTH_UNAVAILABLE",
                "the release truth page was not rendered",
            )),
            (OutputMode::Human, _) => Ok(self.report.human()),
        }
    }

    pub const fn report(&self) -> &CheckReport {
        &self.report
    }
}

pub fn run(hub: &Path, args: &[String]) -> Result<CheckExecution, CoordError> {
    let (tier, mode) = parse(args)?;
    let report = executor::report(hub, tier)?;
    let page = match mode {
        OutputMode::Report(variant) => Some(truth::render(hub, &report, variant)?),
        OutputMode::Human | OutputMode::Json => None,
    };
    Ok(CheckExecution { report, mode, page })
}

fn parse(args: &[String]) -> Result<(CheckTier, OutputMode), CoordError> {
    let (tier, rest) = args
        .split_first()
        .ok_or_else(|| CoordError::new("USAGE", USAGE))?;
    let tier = match tier.as_str() {
        "fast" => CheckTier::Fast,
        "required" => CheckTier::Required,
        "release" => CheckTier::Release,
        _ => return Err(CoordError::new("USAGE", USAGE)),
    };
    let mode = match rest {
        [] => OutputMode::Human,
        [flag] if flag == "--json" => OutputMode::Json,
        [flag] if flag == "--report" && tier == CheckTier::Release => {
            OutputMode::Report(truth::Variant::Live)
        }
        [flag, portable]
            if flag == "--report" && portable == "--portable" && tier == CheckTier::Release =>
        {
            OutputMode::Report(truth::Variant::Portable)
        }
        _ => return Err(CoordError::new("USAGE", USAGE)),
    };
    Ok((tier, mode))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_is_exact() {
        for tier in ["fast", "required", "release"] {
            assert!(parse(&[tier.into()]).is_ok());
            assert!(parse(&[tier.into(), "--json".into()]).is_ok());
        }
        assert_eq!(
            parse(&["release".into(), "--report".into()]).unwrap().1,
            OutputMode::Report(truth::Variant::Live)
        );
        assert_eq!(
            parse(&["release".into(), "--report".into(), "--portable".into()])
                .unwrap()
                .1,
            OutputMode::Report(truth::Variant::Portable)
        );
        for invalid in [
            vec![],
            vec!["unknown".into()],
            vec!["fast".into(), "--yaml".into()],
            vec!["fast".into(), "--json".into(), "--json".into()],
            vec!["--json".into(), "fast".into()],
            vec!["fast".into(), "--report".into()],
            vec!["required".into(), "--report".into(), "--portable".into()],
            vec!["release".into(), "--portable".into()],
            vec!["release".into(), "--portable".into(), "--report".into()],
            vec!["release".into(), "--json".into(), "--report".into()],
        ] {
            assert_eq!(parse(&invalid).unwrap_err().code(), "USAGE");
        }
    }
}
