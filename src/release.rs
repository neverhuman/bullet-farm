//! Read-only verification of signed, non-circular release bundles.

mod schema;
mod verify;

use std::path::PathBuf;

use crate::coord::CoordError;

const USAGE: &str =
    "usage: bullet-family release verify --bundle ABSOLUTE_PATH --allowed-signers ABSOLUTE_PATH";

pub use schema::{
    RELEASE_MANIFEST_SCHEMA_VERSION, ReleaseFile, ReleaseManifest, ReleasePackage,
    SignedReleaseFile,
};

pub fn run(args: &[String]) -> Result<String, CoordError> {
    let options = parse_args(args)?;
    let receipt = verify::verify(&options.bundle, &options.allowed_signers)?;
    Ok(format!(
        "release bundle verified (read-only component): {} packages at {} by {}",
        receipt.package_count, receipt.tag, receipt.signer
    ))
}

#[derive(Debug)]
struct VerifyArgs {
    bundle: PathBuf,
    allowed_signers: PathBuf,
}

fn parse_args(args: &[String]) -> Result<VerifyArgs, CoordError> {
    if args.first().is_none_or(|value| value != "verify") {
        return Err(CoordError::new("USAGE", USAGE));
    }
    let mut bundle = None;
    let mut allowed_signers = None;
    let mut index = 1;
    while index < args.len() {
        let value = args
            .get(index + 1)
            .ok_or_else(|| CoordError::new("USAGE", USAGE))?;
        let slot = match args[index].as_str() {
            "--bundle" => &mut bundle,
            "--allowed-signers" => &mut allowed_signers,
            _ => return Err(CoordError::new("USAGE", USAGE)),
        };
        if slot.replace(PathBuf::from(value)).is_some() {
            return Err(CoordError::new("DUPLICATE_OPTION", USAGE));
        }
        index += 2;
    }
    Ok(VerifyArgs {
        bundle: bundle.ok_or_else(|| CoordError::new("USAGE", USAGE))?,
        allowed_signers: allowed_signers.ok_or_else(|| CoordError::new("USAGE", USAGE))?,
    })
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
        ] {
            assert_eq!(parse_args(&args).unwrap_err().code(), "USAGE");
        }
    }
}
