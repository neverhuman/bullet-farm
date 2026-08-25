use std::{fs, path::PathBuf};

use bullet_wire::{
    ReleaseWireRecord, canonical_json, decode_release_record,
    v1alpha1::{
        GateReceiptV1, ReleaseEvidenceKindV1, ReleaseEvidenceSubjectV1, ReleaseFamilySubjectV1,
        ReleaseReceiptKindV1, ReleaseRegistryEntryV1, ReleaseRegistryManifestV1,
        ReleaseReplayBindingV1, ReleaseReplayStateV1, ReleaseRepositoryNameV1,
        ReleaseRepositorySubjectV1, ReleaseSignerKeyV1, ReleaseSignerPolicyV1, ReleaseSignerRoleV1,
        TrustedTimeObservationV1,
    },
};
use serde_json::{Value, json};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .unwrap()
        .to_path_buf()
}

fn hex(character: char, length: usize) -> String {
    std::iter::repeat_n(character, length).collect()
}

fn digest(character: char) -> String {
    format!("blake3:{}", hex(character, 64))
}

fn typed_id(prefix: &str, character: char) -> String {
    format!("{prefix}_{}", hex(character, 64))
}

fn repository(repository: ReleaseRepositoryNameV1, character: char) -> ReleaseRepositorySubjectV1 {
    ReleaseRepositorySubjectV1 {
        schema_version: "v1alpha1".to_owned(),
        repository,
        tag: "v1.0.0".to_owned(),
        commit_oid: format!("sha1:{}", hex(character, 40)),
        tree_oid: format!("sha1:{}", hex(character, 40)),
        release_signing_identity: format!(
            "release-{character}@bullet.farm|ed25519|SHA256:{}",
            hex(character.to_ascii_uppercase(), 24)
        ),
        dependency_lock_digest: digest(character),
        artifact_manifest_digest: digest(character),
    }
}

fn evidence(subject_kind: ReleaseEvidenceKindV1, character: char) -> ReleaseEvidenceSubjectV1 {
    ReleaseEvidenceSubjectV1 {
        schema_version: "v1alpha1".to_owned(),
        subject_kind,
        subject_id: typed_id("cnt", character),
        subject_digest: digest(character),
    }
}

fn receipt() -> GateReceiptV1 {
    GateReceiptV1 {
        schema_version: "v1alpha1".to_owned(),
        gate_receipt_id: typed_id("grc", 'a'),
        gate_id: "release.rust-toolchain".to_owned(),
        gate_version: 1,
        receipt_kind: ReleaseReceiptKindV1::RustToolchain,
        profile_ids: vec!["self-hosted-v1".to_owned()],
        evidence_nonce: hex('b', 64),
        request_digest: digest('c'),
        profile_graph_digest: digest('d'),
        gate_policy_digest: digest('e'),
        family_subject: ReleaseFamilySubjectV1 {
            schema_version: "v1alpha1".to_owned(),
            family: "bullet-farm".to_owned(),
            family_lock_digest: digest('f'),
            schema_bundle_digest: digest('a'),
            repositories: vec![
                repository(ReleaseRepositoryNameV1::BulletFarm, '1'),
                repository(ReleaseRepositoryNameV1::BulletGit, '2'),
                repository(ReleaseRepositoryNameV1::BulletKernel, '3'),
                repository(ReleaseRepositoryNameV1::BulletPortal, '4'),
            ],
        },
        evidence_subjects: vec![
            evidence(ReleaseEvidenceKindV1::Environment, '1'),
            evidence(ReleaseEvidenceKindV1::Policy, '2'),
            evidence(ReleaseEvidenceKindV1::Schema, '3'),
            evidence(ReleaseEvidenceKindV1::Toolchain, '4'),
        ],
        attestor_key_id: "gate-attestor".to_owned(),
        started_at_unix_ms: 1_000,
        completed_at_unix_ms: 2_000,
        expires_at_unix_ms: 3_000,
    }
}

fn signer_key(key_id: &str, role: ReleaseSignerRoleV1, character: char) -> ReleaseSignerKeyV1 {
    ReleaseSignerKeyV1 {
        schema_version: "v1alpha1".to_owned(),
        key_id: key_id.to_owned(),
        role,
        signing_identity: format!(
            "{key_id}@bullet.farm|ed25519|SHA256:{}",
            hex(character.to_ascii_uppercase(), 24)
        ),
        public_key: format!("ssh-ed25519 {}", hex(character.to_ascii_uppercase(), 44)),
        activates_at_unix_ms: 1_000,
        expires_at_unix_ms: 3_000,
        revoked_at_unix_ms: None,
        retain_until_unix_ms: 4_000,
    }
}

fn signer_policy() -> ReleaseSignerPolicyV1 {
    ReleaseSignerPolicyV1 {
        schema_version: "v1alpha1".to_owned(),
        family: "bullet-farm".to_owned(),
        policy_generation: 1,
        activates_at_unix_ms: 1_000,
        expires_at_unix_ms: 3_000,
        registry_signer_key_id: "registry".to_owned(),
        trusted_time_key_id: "time".to_owned(),
        signer_keys: vec![
            signer_key("artifact", ReleaseSignerRoleV1::ArtifactRelease, 'a'),
            signer_key("attestor", ReleaseSignerRoleV1::GateAttestor, 'b'),
            signer_key("registry", ReleaseSignerRoleV1::RegistryCurator, 'c'),
            signer_key("source", ReleaseSignerRoleV1::SourceTag, 'd'),
            signer_key("time", ReleaseSignerRoleV1::TrustedTime, 'e'),
        ],
    }
}

fn registry_entry() -> ReleaseRegistryEntryV1 {
    ReleaseRegistryEntryV1 {
        schema_version: "v1alpha1".to_owned(),
        gate_id: "release.rust-toolchain".to_owned(),
        gate_receipt_id: typed_id("grc", 'a'),
        receipt_digest: digest('a'),
        receipt_path: "receipts/rust-toolchain.json".to_owned(),
        receipt_signature_digest: digest('b'),
        receipt_signature_path: "signatures/rust-toolchain.sig".to_owned(),
        trusted_time_digest: digest('c'),
        trusted_time_path: "time/rust-toolchain.json".to_owned(),
        trusted_time_signature_digest: digest('d'),
        trusted_time_signature_path: "time/rust-toolchain.sig".to_owned(),
    }
}

fn registry_manifest() -> ReleaseRegistryManifestV1 {
    ReleaseRegistryManifestV1 {
        schema_version: "v1alpha1".to_owned(),
        registry_id: typed_id("rrg", 'a'),
        generation: 1,
        previous_registry_digest: digest('a'),
        signer_policy_digest: digest('b'),
        profile_graph_digest: digest('c'),
        family_lock_digest: digest('d'),
        created_at_unix_ms: 1_000,
        expires_at_unix_ms: 3_000,
        registry_signer_key_id: "registry".to_owned(),
        entries: vec![registry_entry()],
    }
}

fn replay_state() -> ReleaseReplayStateV1 {
    ReleaseReplayStateV1 {
        schema_version: "v1alpha1".to_owned(),
        registry_id: typed_id("rrg", 'a'),
        generation: 1,
        registry_manifest_digest: digest('a'),
        previous_state_digest: digest('b'),
        restore_epoch: 7,
        trusted_time_floor_unix_ms: 1_000,
        bindings: vec![ReleaseReplayBindingV1 {
            schema_version: "v1alpha1".to_owned(),
            evidence_nonce: hex('a', 64),
            gate_receipt_id: typed_id("grc", 'a'),
            gate_id: "release.rust-toolchain".to_owned(),
            request_digest: digest('c'),
            receipt_digest: digest('d'),
        }],
        registry_signer_key_id: "registry".to_owned(),
    }
}

fn trusted_time() -> TrustedTimeObservationV1 {
    TrustedTimeObservationV1 {
        schema_version: "v1alpha1".to_owned(),
        family: "bullet-farm".to_owned(),
        gate_receipt_id: typed_id("grc", 'a'),
        receipt_digest: digest('a'),
        evidence_nonce: hex('b', 64),
        signer_policy_digest: digest('c'),
        observed_at_unix_ms: 1_000,
        valid_until_unix_ms: 3_000,
        restore_epoch: 7,
        trusted_time_key_id: "time".to_owned(),
    }
}

fn assert_release_error<T: ReleaseWireRecord + std::fmt::Debug>(value: &T) {
    let bytes = canonical_json(value).unwrap();
    assert_eq!(
        decode_release_record::<T>(&bytes).unwrap_err().code(),
        "INVALID_RELEASE_WIRE_RECORD"
    );
}

fn add_unknown(mut value: Value, pointer: &str, field: &str) -> Vec<u8> {
    value
        .pointer_mut(pointer)
        .and_then(Value::as_object_mut)
        .unwrap()
        .insert(field.to_owned(), json!("caller-selected"));
    canonical_json(&value).unwrap()
}

#[test]
fn release_registry_records_are_closed_exact_and_hostile() {
    let receipt = receipt();
    let bytes = canonical_json(&receipt).unwrap();
    assert_eq!(
        decode_release_record::<GateReceiptV1>(&bytes).unwrap(),
        receipt
    );
    signer_policy().validate_release().unwrap();
    registry_manifest().validate_release().unwrap();
    replay_state().validate_release().unwrap();
    trusted_time().validate_release().unwrap();

    let value = serde_json::to_value(&receipt).unwrap();
    for (pointer, field) in [
        ("", "result"),
        ("", "signature"),
        ("/family_subject", "surprise"),
        ("/family_subject/repositories/0", "commit_signature"),
        ("/evidence_subjects/0", "payload"),
    ] {
        let error =
            decode_release_record::<GateReceiptV1>(&add_unknown(value.clone(), pointer, field))
                .unwrap_err();
        assert_eq!(error.code(), "DOCUMENT_SCHEMA_INVALID", "{pointer}/{field}");
    }
    let pretty = serde_json::to_string_pretty(&receipt).unwrap();
    assert_eq!(
        decode_release_record::<GateReceiptV1>(pretty.as_bytes())
            .unwrap_err()
            .code(),
        "NON_CANONICAL_JSON"
    );
    assert_eq!(
        decode_release_record::<GateReceiptV1>(
            br#"{"schema_version":"v1alpha1","schema_version":"v1alpha1"}"#
        )
        .unwrap_err()
        .code(),
        "DUPLICATE_JSON_KEY"
    );
    let unsafe_integer = String::from_utf8(bytes.clone())
        .unwrap()
        .replace("\"gate_version\":1", "\"gate_version\":9007199254740992");
    assert_eq!(
        decode_release_record::<GateReceiptV1>(unsafe_integer.as_bytes())
            .unwrap_err()
            .code(),
        "UNSAFE_JSON_INTEGER"
    );

    let mut unsorted_family = receipt.clone();
    unsorted_family.family_subject.repositories.swap(0, 1);
    assert_release_error(&unsorted_family);
    let mut incomplete_evidence = receipt.clone();
    incomplete_evidence.evidence_subjects.pop();
    assert_release_error(&incomplete_evidence);
    let mut repeated_kind = receipt.clone();
    repeated_kind
        .evidence_subjects
        .insert(1, evidence(ReleaseEvidenceKindV1::Environment, '5'));
    repeated_kind.validate_release().unwrap();

    let mut shared_key = signer_policy();
    shared_key.signer_keys[1].public_key = shared_key.signer_keys[0].public_key.clone();
    assert_release_error(&shared_key);
    let mut revoked_after_expiry = signer_policy();
    revoked_after_expiry.signer_keys[0].revoked_at_unix_ms = Some(3_000);
    assert_release_error(&revoked_after_expiry);
    let mut traversal = registry_manifest();
    traversal.entries[0].receipt_path = "../receipt.json".to_owned();
    assert_release_error(&traversal);
    let mut replay = replay_state();
    replay.bindings.push(replay.bindings[0].clone());
    assert_release_error(&replay);
    let mut pre_restore_replay = replay_state();
    pre_restore_replay.restore_epoch = 0;
    assert_release_error(&pre_restore_replay);
    let mut pre_restore_time = trusted_time();
    pre_restore_time.restore_epoch = 0;
    assert_release_error(&pre_restore_time);

    let generated_rust =
        fs::read_to_string(root().join("contracts/generated/rust/schema_bundle.rs")).unwrap();
    let rust_gate = generated_rust
        .split("pub struct GateReceiptV1")
        .nth(1)
        .unwrap()
        .split("pub struct GraphDeltaV1")
        .next()
        .unwrap();
    assert!(!rust_gate.contains("serde_json::Value"));
    assert!(!rust_gate.contains("pub result:"));
    assert!(!rust_gate.contains("pub signature:"));

    let generated_ts =
        fs::read_to_string(root().join("contracts/generated/typescript/schemaBundle.ts")).unwrap();
    let ts_gate = generated_ts
        .split("export interface GateReceiptV1")
        .nth(1)
        .unwrap()
        .split("export interface GraphDeltaV1")
        .next()
        .unwrap();
    assert!(ts_gate.contains("family_subject: ReleaseFamilySubjectV1;"));
    assert!(ts_gate.contains("evidence_subjects: ReleaseEvidenceSubjectV1[];"));
    assert!(!ts_gate.contains("Record<string, unknown>"));
    assert!(!ts_gate.contains("result:"));
    assert!(!ts_gate.contains("signature:"));

    let schema_bundle: Value = serde_json::from_slice(
        &fs::read(root().join("contracts/v1alpha1/schema-bundle.json")).unwrap(),
    )
    .unwrap();
    let gate_schema = &schema_bundle["schemas"]["GateReceiptV1"];
    assert_eq!(gate_schema["additionalProperties"], false);
    assert_eq!(
        gate_schema["properties"]["family_subject"]["$ref"],
        "#/schemas/ReleaseFamilySubjectV1"
    );
    assert!(gate_schema["properties"].get("result").is_none());
    assert!(gate_schema["properties"].get("signature").is_none());
}
