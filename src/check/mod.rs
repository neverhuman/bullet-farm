//! Rust-owned, exact-subject local check catalogs.

mod catalog;
mod executor;
mod prerequisites;
mod subject;

pub mod model;

use crate::coord::CoordError;
use model::{CheckReport, CheckTier};
use std::path::Path;

const USAGE: &str = "usage: bullet-family [--root PATH] check <fast|required|release> [--json]";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckExecution {
    report: CheckReport,
    json: bool,
}

impl CheckExecution {
    pub const fn exit_code(&self) -> u8 {
        self.report.exit_code()
    }

    pub fn output(&self) -> Result<String, CoordError> {
        if self.json {
            self.report.stable_json().map_err(CoordError::json)
        } else {
            Ok(self.report.human())
        }
    }

    pub const fn report(&self) -> &CheckReport {
        &self.report
    }
}

pub fn run(hub: &Path, args: &[String]) -> Result<CheckExecution, CoordError> {
    let (tier, json) = parse(args)?;
    let report = executor::report(hub, tier)?;
    Ok(CheckExecution { report, json })
}

fn parse(args: &[String]) -> Result<(CheckTier, bool), CoordError> {
    let (tier, rest) = args
        .split_first()
        .ok_or_else(|| CoordError::new("USAGE", USAGE))?;
    let tier = match tier.as_str() {
        "fast" => CheckTier::Fast,
        "required" => CheckTier::Required,
        "release" => CheckTier::Release,
        _ => return Err(CoordError::new("USAGE", USAGE)),
    };
    let json = match rest {
        [] => false,
        [flag] if flag == "--json" => true,
        _ => return Err(CoordError::new("USAGE", USAGE)),
    };
    Ok((tier, json))
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
        for invalid in [
            vec![],
            vec!["unknown".into()],
            vec!["fast".into(), "--yaml".into()],
            vec!["fast".into(), "--json".into(), "--json".into()],
            vec!["--json".into(), "fast".into()],
        ] {
            assert_eq!(parse(&invalid).unwrap_err().code(), "USAGE");
        }
    }
}
