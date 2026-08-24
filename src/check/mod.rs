//! Rust-owned check command foundation. Execution catalogs intentionally do not exist yet.

mod prerequisites;

pub mod model;

use crate::coord::CoordError;
use model::{CheckReport, CheckTier};

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

pub fn run(args: &[String]) -> Result<CheckExecution, CoordError> {
    let (tier, json) = parse(args)?;
    let report = prerequisites::report(tier)
        .map_err(|error| CoordError::new(error.code(), error.to_string()))?;
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
            assert!(run(&[tier.into()]).is_ok());
            assert!(run(&[tier.into(), "--json".into()]).is_ok());
        }
        for invalid in [
            vec![],
            vec!["unknown".into()],
            vec!["fast".into(), "--yaml".into()],
            vec!["fast".into(), "--json".into(), "--json".into()],
            vec!["--json".into(), "fast".into()],
        ] {
            assert_eq!(run(&invalid).unwrap_err().code(), "USAGE");
        }
    }
}
