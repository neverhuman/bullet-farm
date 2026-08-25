//! Fallible dependency and generated-contract checks over an unpublished candidate family.

use std::{fs, path::Path};

use super::{
    command::{SetupEnvironment, Toolchain},
    transaction::{CandidateFamily, CandidateValidator},
};
use crate::coord::CoordError;

const MAX_CONTRACT_BYTES: u64 = 16 * 1024 * 1024;
const CONTRACT_LINKS: &[(&str, &str, &str, &str)] = &[
    (
        "bullet-farm",
        "contracts/generated/rust/schema_bundle.rs",
        "bullet-kernel",
        "crates/domain/src/schema_bundle.rs",
    ),
    (
        "bullet-farm",
        "contracts/generated/rust/schema_bundle.rs",
        "bullet-git",
        "crates/bullet-git-types/src/schema_bundle.rs",
    ),
    (
        "bullet-farm",
        "contracts/generated/typescript/schemaBundle.ts",
        "bullet-portal",
        "src/generated/schemaBundle.ts",
    ),
    (
        "bullet-kernel",
        "contracts/generated/api.ts",
        "bullet-portal",
        "src/generated/api.ts",
    ),
    (
        "bullet-farm",
        "formal/traces/effect-check-ambiguity.json",
        "bullet-kernel",
        "crates/adapters/tests/fixtures/formal/effect-check-ambiguity.json",
    ),
    (
        "bullet-farm",
        "formal/traces/effect-third-party.json",
        "bullet-kernel",
        "crates/adapters/tests/fixtures/formal/effect-third-party.json",
    ),
    (
        "bullet-farm",
        "formal/traces/lease-fence-reclaim.json",
        "bullet-kernel",
        "crates/adapters/tests/fixtures/formal/lease-fence-reclaim.json",
    ),
];

pub(super) struct SetupValidator<'a> {
    offline: bool,
    toolchain: &'a Toolchain,
    environment: &'a SetupEnvironment,
}

impl<'a> SetupValidator<'a> {
    pub(super) const fn new(
        offline: bool,
        toolchain: &'a Toolchain,
        environment: &'a SetupEnvironment,
    ) -> Self {
        Self {
            offline,
            toolchain,
            environment,
        }
    }
}

impl CandidateValidator for SetupValidator<'_> {
    fn validate(&self, candidate: &CandidateFamily<'_>) -> Result<(), CoordError> {
        for name in ["bullet-farm", "bullet-kernel", "bullet-git"] {
            let mut args = vec!["fetch", "--locked"];
            if self.offline {
                args.push("--offline");
            }
            self.toolchain
                .run_cargo(candidate.path(name)?, &args, self.environment)?;
        }
        let mut npm_args = vec!["ci", "--ignore-scripts", "--no-audit", "--no-fund"];
        if self.offline {
            npm_args.push("--offline");
        }
        self.toolchain.run_npm(
            candidate.path("bullet-portal")?,
            &npm_args,
            self.environment,
        )?;
        self.toolchain.run_cargo(
            candidate.path("bullet-farm")?,
            &[
                "run",
                "--locked",
                "--quiet",
                "-p",
                "bullet-wire",
                "--bin",
                "bullet-contract",
                "--",
                "check",
                "--root",
                ".",
            ],
            self.environment,
        )?;
        self.toolchain.run_cargo(
            candidate.path("bullet-kernel")?,
            &[
                "run",
                "--locked",
                "--quiet",
                "-p",
                "bullet",
                "--",
                "contracts",
                "check",
            ],
            self.environment,
        )?;
        self.toolchain.run_bash(
            candidate.path("bullet-farm")?,
            &["-n", "scripts/sync-family-contracts.sh"],
            self.environment,
        )?;
        validate_contract_links(candidate)
    }
}

fn validate_contract_links(candidate: &CandidateFamily<'_>) -> Result<(), CoordError> {
    for &(source_member, source, destination_member, destination) in CONTRACT_LINKS {
        let expected = read_bounded(&candidate.path(source_member)?.join(source))?;
        let actual = read_bounded(&candidate.path(destination_member)?.join(destination))?;
        if actual != expected {
            return Err(CoordError::new(
                "GENERATED_CONTRACT_DRIFT",
                format!("{destination_member}/{destination} differs from {source_member}/{source}"),
            ));
        }
    }
    Ok(())
}

fn read_bounded(path: &Path) -> Result<Vec<u8>, CoordError> {
    let metadata = fs::symlink_metadata(path).map_err(CoordError::io)?;
    if metadata.file_type().is_symlink()
        || !metadata.file_type().is_file()
        || metadata.len() > MAX_CONTRACT_BYTES
    {
        return Err(CoordError::new(
            "INVALID_GENERATED_CONTRACT",
            format!(
                "{} is not an admitted regular contract file",
                path.display()
            ),
        ));
    }
    fs::read(path).map_err(CoordError::io)
}
