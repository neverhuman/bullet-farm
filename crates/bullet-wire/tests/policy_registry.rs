use std::{fs, path::PathBuf};

use bullet_wire::{
    ContractCatalogV1, ContractMode, InvariantRegistryV1, PolicySnapshotV1, canonical_json,
    decode_canonical, execute_contract_tool,
};
use sha2::{Digest, Sha256};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .unwrap()
        .to_path_buf()
}

#[test]
fn registry_is_complete_tiered_and_phase_honest() {
    let bytes = fs::read(root().join("policy/v1alpha1/invariant-registry.json")).unwrap();
    let registry = decode_canonical::<InvariantRegistryV1>(&bytes).unwrap();
    registry.validate().unwrap();
    assert!(registry.entries.iter().any(|entry| {
        entry.lifecycle == bullet_wire::InvariantLifecycle::Planned
            && entry.first_applicable_wave == 11
    }));
    assert!(registry.entries.iter().all(|entry| {
        entry.first_applicable_wave > 1
            || entry.lifecycle == bullet_wire::InvariantLifecycle::Enforced
    }));
}

#[test]
fn duplicate_alias_and_unknown_registry_field_fail_closed() {
    let bytes = fs::read(root().join("policy/v1alpha1/invariant-registry.json")).unwrap();
    let mut registry = decode_canonical::<InvariantRegistryV1>(&bytes).unwrap();
    let alias = registry.entries[0].legacy_aliases[0].clone();
    registry.entries[1].legacy_aliases.push(alias);
    assert_eq!(
        registry.validate().unwrap_err().code(),
        "DUPLICATE_INVARIANT_ID"
    );

    let mut value = serde_json::from_slice::<serde_json::Value>(&bytes).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("authority".to_owned(), serde_json::json!(true));
    let changed = canonical_json(&value).unwrap();
    assert_eq!(
        decode_canonical::<InvariantRegistryV1>(&changed)
            .unwrap_err()
            .code(),
        "DOCUMENT_SCHEMA_INVALID"
    );
}

#[test]
fn catalog_names_cannot_inject_generated_languages() {
    let bytes = fs::read(root().join("contracts/v1alpha1/contract-catalog.json")).unwrap();
    let mut catalog = decode_canonical::<ContractCatalogV1>(&bytes).unwrap();
    catalog.records[0].name = "Injected{Code".to_owned();
    assert_eq!(
        catalog.validate().unwrap_err().code(),
        "INVALID_CONTRACT_RECORD"
    );

    let mut catalog = decode_canonical::<ContractCatalogV1>(&bytes).unwrap();
    catalog.records[0].fields[0].name = "type".to_owned();
    assert_eq!(
        catalog.validate().unwrap_err().code(),
        "INVALID_CONTRACT_FIELD"
    );
}

#[test]
fn policy_and_catalog_are_strict_complete_and_offline() {
    let policy = decode_canonical::<PolicySnapshotV1>(
        &fs::read(root().join("policy/v1alpha1/policy.json")).unwrap(),
    )
    .unwrap();
    policy.validate().unwrap();
    assert!(!policy.sandbox_policy.live_admission_enabled);

    let catalog = decode_canonical::<ContractCatalogV1>(
        &fs::read(root().join("contracts/v1alpha1/contract-catalog.json")).unwrap(),
    )
    .unwrap();
    catalog.validate().unwrap();
    let bundle = catalog.json_schema_bundle();
    assert_eq!(
        bundle["schemas"]["SignedAuthorityEnvelopeV1"]["additionalProperties"],
        false
    );
    assert_eq!(
        bundle["schemas"]["RouteDecision"]["additionalProperties"],
        false
    );
}

#[test]
fn hostile_team_fixture_preserves_the_audited_bytes() {
    let bytes = fs::read(root().join("fixtures/hostile/team-original.bin")).unwrap();
    assert_eq!(bytes.len(), 31_835);
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        "013f19032017ea5e27f69717f5e294e46aa56fbdabe31619ecfa0f77ac4007bf"
    );
}

#[test]
fn committed_generated_contracts_have_zero_byte_drift() {
    execute_contract_tool(&root(), ContractMode::Check).unwrap();
}

#[test]
fn generated_pin_accepts_only_exact_canonical_contract_bytes() {
    use bullet_wire::v1alpha1::{PinnedContract, verify_pinned_contract};

    let fixtures = [
        (
            PinnedContract::SchemaBundle,
            "contracts/v1alpha1/schema-bundle.json",
        ),
        (
            PinnedContract::InvariantRegistry,
            "policy/v1alpha1/invariant-registry.json",
        ),
        (
            PinnedContract::PolicySnapshot,
            "policy/v1alpha1/policy.json",
        ),
    ];
    for (contract, path) in fixtures {
        let bytes = fs::read(root().join(path)).unwrap();
        verify_pinned_contract(contract, &bytes).unwrap();
        let mut changed = bytes;
        changed.push(b' ');
        let error = verify_pinned_contract(contract, &changed).unwrap_err();
        assert_eq!(error.reason_code(), "UNPINNED_CONTRACT");
    }

    verify_pinned_contract(
        PinnedContract::CanonicalGolden,
        bullet_wire::v1alpha1::CANONICAL_GOLDEN_JSON.as_bytes(),
    )
    .unwrap();
    assert!(
        verify_pinned_contract(
            PinnedContract::SchemaBundle,
            bullet_wire::v1alpha1::CANONICAL_GOLDEN_JSON.as_bytes(),
        )
        .is_err()
    );
}

#[test]
fn generated_records_are_strict_runtime_types() {
    let value = serde_json::json!({
        "schema_version": "v1alpha1",
        "failure_class_id": format!("failure_{}", "0".repeat(64)),
        "taxonomy_version": "v1",
        "class_name": "boundary",
        "definition": "defined",
        "surprise": true
    });
    assert!(serde_json::from_value::<bullet_wire::v1alpha1::FailureClass>(value).is_err());
}
