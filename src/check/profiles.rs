//! Explicit, composable release profiles. Every profile remains fail-closed.

#[path = "profiles/graph.rs"]
mod graph;

use std::{collections::BTreeSet, fs, path::Path};

use super::model::{CheckModelError, GateClass, GateResult};

pub(super) use graph::ReleaseProfile;

pub(super) fn select(
    profile: ReleaseProfile,
    gates: Vec<GateResult>,
    registry: &Path,
) -> Result<Vec<GateResult>, CheckModelError> {
    let closure = dependency_closure(profile);
    let requested = closure
        .iter()
        .flat_map(|item| item.catalog_gate_ids().iter().copied())
        .collect::<BTreeSet<_>>();
    let mut found = BTreeSet::new();
    let mut selected = Vec::new();
    for gate in gates {
        if requested.contains(gate.id()) {
            found.insert(gate.id().to_owned());
            selected.push(gate);
        }
    }
    let missing = requested
        .iter()
        .filter(|id| !found.contains(**id))
        .copied()
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(CheckModelError::new(
            "PROFILE_GATE_MISSING",
            format!(
                "{} references catalog gates that are absent: {}",
                profile.as_str(),
                missing.join(", ")
            ),
        ));
    }

    if profile != ReleaseProfile::LegacyV1_26 {
        replace_receipt_registry_gate(&mut selected, registry)?;
    }
    for item in closure {
        if item.has_condition_gate() {
            selected.push(profile_condition_gate(item)?);
        }
    }
    if profile == ReleaseProfile::LinuxPreview {
        replace_linux_preview_details(&mut selected)?;
        selected.extend(linux_preview_specific_gates()?);
    }
    Ok(selected)
}

fn dependency_closure(profile: ReleaseProfile) -> Vec<ReleaseProfile> {
    fn visit(
        profile: ReleaseProfile,
        visited: &mut BTreeSet<ReleaseProfile>,
        ordered: &mut Vec<ReleaseProfile>,
    ) {
        for dependency in profile.dependencies() {
            visit(*dependency, visited, ordered);
        }
        if visited.insert(profile) {
            ordered.push(profile);
        }
    }

    let mut visited = BTreeSet::new();
    let mut ordered = Vec::new();
    visit(profile, &mut visited, &mut ordered);
    ordered
}

fn profile_condition_gate(profile: ReleaseProfile) -> Result<GateResult, CheckModelError> {
    blocked(
        &format!("release.profile.{}", profile.as_str()),
        GateClass::Release,
        &format!(
            "{} has no current kind-specific semantic receipt proving {}",
            profile.as_str(),
            profile.required_closure()
        ),
        "register exact current-family evidence only after signer lifecycle, dependency closure, schema-3 family, policy/toolchain/environment fingerprints, trusted time, replay state, and the profile-specific semantic verifier all pass",
    )
}

fn replace_receipt_registry_gate(
    gates: &mut [GateResult],
    registry: &Path,
) -> Result<(), CheckModelError> {
    let index = gates
        .iter()
        .position(|gate| gate.id() == "release.receipt-contracts")
        .ok_or_else(|| {
            CheckModelError::new(
                "PROFILE_RECEIPT_GATE_MISSING",
                "release profile closure omits the receipt-contracts gate",
            )
        })?;
    gates[index] = match fs::symlink_metadata(registry) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => blocked(
            "release.receipt-contracts",
            GateClass::Release,
            "the selected receipt registry is absent; zero profile gates were cleared from it",
            "provision an absolute, non-symlink registry only after its kind-specific semantic verifiers and external trust policy exist",
        )?,
        Err(error) => GateResult::fail(
            "release.receipt-contracts",
            GateClass::Release,
            format!("the selected receipt registry could not be inspected: {error}"),
            "preserve the registry and repair its operating-system admission before retrying",
        )?,
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            GateResult::fail(
                "release.receipt-contracts",
                GateClass::Release,
                "the selected receipt registry is not a real non-symlink directory",
                "supply an absolute non-symlink directory; never redirect release evidence through a link or regular file",
            )?
        }
        Ok(_) => blocked(
            "release.receipt-contracts",
            GateClass::Release,
            "the selected registry is present, but no generic receipt can clear a gate without a kind-specific semantic verifier and admitted trust/time roots",
            "add the gate-specific semantic verifier and externally admitted signer/trusted-time policy, then register exact current-family evidence",
        )?,
    };
    Ok(())
}

fn replace_linux_preview_details(gates: &mut [GateResult]) -> Result<(), CheckModelError> {
    for (id, detail, repair) in [
        (
            "release.platform-containment",
            "Ubuntu 24.04 x86_64 S1 rootless-crun and policy-required S2 Firecracker containment receipts are absent",
            "certify the Linux namespace/cgroup/seccomp/network boundary and the pinned Firecracker guest path; S2-required work must refuse until then",
        ),
        (
            "release.provenance",
            "signed build provenance for the exact Ubuntu 24.04 x86_64 package is absent",
            "produce and verify provenance bound to the exact hub tag, schema-3 lock, toolchains, Portal bundle, and Linux archive",
        ),
        (
            "release.sbom",
            "CycloneDX and SPDX bills of materials for the exact Ubuntu 24.04 x86_64 package are absent",
            "generate and semantically validate both SBOM formats against the exact Linux archive",
        ),
        (
            "release.signatures",
            "verified Ed25519 receipt and Sigstore artifact signatures for the exact Ubuntu 24.04 x86_64 package are absent",
            "sign and verify the Linux archive, checksums, SBOMs, provenance, and final non-circular manifest with admitted release keys",
        ),
    ] {
        let index = gates
            .iter()
            .position(|gate| gate.id() == id)
            .ok_or_else(|| {
                CheckModelError::new(
                    "PROFILE_GATE_MISSING",
                    format!("linux-preview is missing {id}"),
                )
            })?;
        gates[index] = blocked(id, GateClass::Release, detail, repair)?;
    }
    Ok(())
}

fn linux_preview_specific_gates() -> Result<Vec<GateResult>, CheckModelError> {
    [
        (
            "release.evolution-v1",
            "post-V1 evolutionary study and canary evidence is absent, as expected while evolutionary authority remains disabled for V1",
            "retain this preview-only diagnostic without treating it as a canonical GA gate; schedule the frozen external study and bounded canary only after V1",
        ),
        (
            "release.operations-v1",
            "production health/readiness/metrics, freeze, incident, audit-anchor, backup, restore, rollback, and disaster workflows lack one exact operations receipt",
            "exercise the packaged single-host operations runbook and register its signed exact-subject receipt",
        ),
        (
            "release.package-linux-x86_64",
            "the signed Ubuntu 24.04 x86_64 package with embedded Portal, services, migrations, sandbox assets, and guest image is absent",
            "build and semantically verify the exact tagged x86_64-unknown-linux-gnu package and its supply-chain subjects",
        ),
        (
            "release.systemd-v1",
            "the native systemd install, upgrade, activation, rollback, uninstall, and non-destructive retention receipt is absent",
            "run two clean Ubuntu 24.04 installs plus lifecycle and disaster drills from the signed package bytes",
        ),
    ]
    .into_iter()
    .map(|(id, detail, repair)| blocked(id, GateClass::Release, detail, repair))
    .collect()
}

fn blocked(
    id: &str,
    class: GateClass,
    detail: &str,
    repair: &str,
) -> Result<GateResult, CheckModelError> {
    GateResult::blocked(id, class, detail, repair)
}
