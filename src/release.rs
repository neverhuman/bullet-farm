//! Verification and safe extraction components for signed release bundles.

mod archive;
mod schema;
mod verify;

use std::path::PathBuf;

use crate::coord::CoordError;

const VERIFY_USAGE: &str =
    "usage: bullet-family release verify --bundle ABSOLUTE_PATH --allowed-signers ABSOLUTE_PATH";
const EXTRACT_USAGE: &str = "usage: bullet-family release extract --bundle ABSOLUTE_PATH --allowed-signers ABSOLUTE_PATH --target TARGET --destination ABSOLUTE_PATH";

pub use schema::{
    RELEASE_MANIFEST_SCHEMA_VERSION, ReleaseFile, ReleaseManifest, ReleasePackage,
    SignedReleaseFile,
};

pub fn run(args: &[String]) -> Result<String, CoordError> {
    match parse_args(args)? {
        Command::Verify(options) => {
            let receipt = verify::verify(&options.bundle, &options.allowed_signers)?;
            Ok(format!(
                "release bundle verified (read-only component): {} packages at {} by {}",
                receipt.manifest.package.len(),
                receipt.manifest.tag,
                receipt.manifest.release_signing_identity
            ))
        }
        Command::Extract(options) => {
            let receipt = verify::verify(&options.bundle, &options.allowed_signers)?;
            archive::extract(
                &options.bundle,
                &receipt.manifest,
                &options.target,
                &options.destination,
            )?;
            Ok(format!(
                "verified release archive extracted (component only; no install or release authority): {} at {}",
                options.target,
                options.destination.display()
            ))
        }
    }
}

#[derive(Debug)]
enum Command {
    Verify(CommonArgs),
    Extract(ExtractArgs),
}

#[derive(Debug)]
struct CommonArgs {
    bundle: PathBuf,
    allowed_signers: PathBuf,
}

#[derive(Debug)]
struct ExtractArgs {
    bundle: PathBuf,
    allowed_signers: PathBuf,
    target: String,
    destination: PathBuf,
}

fn parse_args(args: &[String]) -> Result<Command, CoordError> {
    let action = args
        .first()
        .ok_or_else(|| CoordError::new("USAGE", VERIFY_USAGE))?;
    let usage = match action.as_str() {
        "verify" => VERIFY_USAGE,
        "extract" => EXTRACT_USAGE,
        _ => return Err(CoordError::new("USAGE", VERIFY_USAGE)),
    };
    let mut bundle = None;
    let mut allowed_signers = None;
    let mut target = None;
    let mut destination = None;
    let mut index = 1;
    while index < args.len() {
        let value = args
            .get(index + 1)
            .ok_or_else(|| CoordError::new("USAGE", usage))?;
        let duplicate = match args[index].as_str() {
            "--bundle" => bundle.replace(PathBuf::from(value)).is_some(),
            "--allowed-signers" => allowed_signers.replace(PathBuf::from(value)).is_some(),
            "--target" if action == "extract" => target.replace(value.clone()).is_some(),
            "--destination" if action == "extract" => {
                destination.replace(PathBuf::from(value)).is_some()
            }
            _ => return Err(CoordError::new("USAGE", usage)),
        };
        if duplicate {
            return Err(CoordError::new("DUPLICATE_OPTION", usage));
        }
        index += 2;
    }
    let common = CommonArgs {
        bundle: bundle.ok_or_else(|| CoordError::new("USAGE", usage))?,
        allowed_signers: allowed_signers.ok_or_else(|| CoordError::new("USAGE", usage))?,
    };
    match action.as_str() {
        "verify" => Ok(Command::Verify(common)),
        "extract" => Ok(Command::Extract(ExtractArgs {
            bundle: common.bundle,
            allowed_signers: common.allowed_signers,
            target: target.ok_or_else(|| CoordError::new("USAGE", usage))?,
            destination: destination.ok_or_else(|| CoordError::new("USAGE", usage))?,
        })),
        _ => unreachable!("action was admitted above"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arguments_are_exact() {
        for args in [
            vec![],
            vec!["verify".into()],
            vec!["install".into()],
            vec!["verify".into(), "--unknown".into(), "x".into()],
            vec!["extract".into(), "--bundle".into(), "x".into()],
        ] {
            assert_eq!(parse_args(&args).unwrap_err().code(), "USAGE");
        }
    }
}
