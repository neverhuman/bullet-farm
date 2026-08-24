use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::{
    AuthorityAudience, AuthorityClaims, AuthoritySigningKey, ContractCatalogV1, EnforcementTier,
    InvariantLifecycle, InvariantRegistryV1, MutationOperation, PolicySnapshotV1, PolicyTemplateV1,
    WireError, authority_request_digest, canonical_json,
    contract_bindings::{rust_constants, typescript_constants},
    decode_canonical, hash_canonical, hash_framed_bytes,
};

const CATALOG: &str = "contracts/v1alpha1/contract-catalog.json";
const REGISTRY: &str = "policy/v1alpha1/invariant-registry.json";
const POLICY_TEMPLATE: &str = "policy/v1alpha1/policy-template.json";
const HOSTILE_TEAM: &str = "fixtures/hostile/team-original.bin";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractMode {
    Generate,
    Check,
}

pub fn execute(root: &Path, mode: ContractMode) -> Result<(), WireError> {
    let outputs = render(root)?;
    match mode {
        ContractMode::Generate => write_outputs(root, &outputs),
        ContractMode::Check => check_outputs(root, &outputs),
    }
}

fn render(root: &Path) -> Result<BTreeMap<PathBuf, Vec<u8>>, WireError> {
    let catalog_bytes = read(root, CATALOG)?;
    let catalog = decode_canonical::<ContractCatalogV1>(&catalog_bytes)?;
    catalog.validate()?;
    let schema_bundle = catalog.json_schema_bundle();
    let schema_bundle_bytes = canonical_json(&schema_bundle)?;
    let schema_bundle_hash = hash_canonical("schema.bundle", &schema_bundle)?;

    let registry_bytes = read(root, REGISTRY)?;
    let registry = decode_canonical::<InvariantRegistryV1>(&registry_bytes)?;
    registry.validate()?;
    let registry_hash = hash_canonical("invariant.registry", &registry)?;

    let template_bytes = read(root, POLICY_TEMPLATE)?;
    let template = decode_canonical::<PolicyTemplateV1>(&template_bytes)?;
    let policy = policy_snapshot(template, schema_bundle_hash, registry_hash);
    policy.validate()?;
    let policy_bytes = canonical_json(&policy)?;
    let policy_hash = hash_canonical("policy.snapshot", &policy)?;

    let team = read(root, HOSTILE_TEAM)?;
    let fixture_manifest = fixture_manifest(&team)?;
    let (golden, golden_json, golden_hash) = canonical_golden()?;
    let (authority_golden, authority_golden_hash) = authority_golden()?;
    let rust_binding = rust_constants(
        schema_bundle_hash,
        registry_hash,
        policy_hash,
        &golden_json,
        golden_hash,
        authority_golden_hash,
        &catalog,
    );
    let typescript_binding = typescript_constants(
        schema_bundle_hash,
        registry_hash,
        policy_hash,
        &golden_json,
        golden_hash,
        authority_golden_hash,
        &catalog,
    );
    let generated_clients = json!({
        "rust": hash_framed_bytes("generated.client.rust", rust_binding.as_bytes())?,
        "typescript": hash_framed_bytes(
            "generated.client.typescript",
            typescript_binding.as_bytes()
        )?
    });
    let bundle_manifest = json!({
        "bundle_hash": schema_bundle_hash,
        "authority_golden_hash": authority_golden_hash,
        "catalog_hash": hash_framed_bytes("contract.catalog", &catalog_bytes)?,
        "generated_client_hash": hash_canonical("generated.clients", &generated_clients)?,
        "generated_clients": generated_clients,
        "generator": "bullet-wire-contract-tool-v1alpha1",
        "invariant_registry_hash": registry_hash,
        "policy_snapshot_hash": policy_hash,
        "record_count": catalog.records.len(),
        "schema_version": "v1alpha1"
    });

    let mut outputs = BTreeMap::new();
    outputs.insert(
        "contracts/v1alpha1/schema-bundle.json".into(),
        schema_bundle_bytes,
    );
    outputs.insert(
        "contracts/v1alpha1/bundle-manifest.json".into(),
        canonical_json(&bundle_manifest)?,
    );
    outputs.insert(
        "contracts/generated/rust/schema_bundle.rs".into(),
        rust_binding.into_bytes(),
    );
    outputs.insert(
        "contracts/generated/typescript/schemaBundle.ts".into(),
        typescript_binding.into_bytes(),
    );
    outputs.insert("policy/v1alpha1/policy.json".into(), policy_bytes);
    outputs.insert(
        "fixtures/hostile/fixture-manifest.json".into(),
        canonical_json(&fixture_manifest)?,
    );
    outputs.insert(
        "fixtures/canonical/canonical-golden.json".into(),
        canonical_json(&golden)?,
    );
    outputs.insert(
        "fixtures/canonical/authority-golden.json".into(),
        canonical_json(&authority_golden)?,
    );
    outputs.insert(
        "docs/assurance/invariant-crosswalk.generated.md".into(),
        invariant_crosswalk(&registry).into_bytes(),
    );
    outputs.extend(hostile_cases());
    Ok(outputs)
}

fn invariant_crosswalk(registry: &InvariantRegistryV1) -> String {
    let mut markdown = String::from(
        "<!-- Generated by: bullet-wire-contract-tool v1alpha1 -->\n<!-- Source: policy/v1alpha1/invariant-registry.json -->\n<!-- Command: just contract-generate -->\n<!-- DO NOT EDIT BY HAND. -->\n\n# Invariant crosswalk\n\n| Stable ID | Control | Tier | Lifecycle | First wave | Owner | Enforcement target | Gate |\n| --- | --- | --- | --- | ---: | --- | --- | --- |\n",
    );
    for entry in &registry.entries {
        let controls = if entry.control_ids.is_empty() {
            "—".to_owned()
        } else {
            entry.control_ids.join(", ")
        };
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            entry.id,
            controls,
            tier_label(entry.tier),
            lifecycle_label(entry.lifecycle),
            entry.first_applicable_wave,
            entry.owner,
            if entry.enforcement_target.is_empty() {
                "—"
            } else {
                &entry.enforcement_target
            },
            entry.gate,
        ));
    }
    markdown
}

const fn tier_label(tier: EnforcementTier) -> &'static str {
    match tier {
        EnforcementTier::T1Schema => "T1 schema",
        EnforcementTier::T2Gateway => "T2 gateway",
        EnforcementTier::T3Test => "T3 test",
    }
}

const fn lifecycle_label(lifecycle: InvariantLifecycle) -> &'static str {
    match lifecycle {
        InvariantLifecycle::Planned => "planned",
        InvariantLifecycle::Enforced => "enforced",
        InvariantLifecycle::Retired => "retired",
    }
}

fn hostile_cases() -> BTreeMap<PathBuf, Vec<u8>> {
    let cases: [(&str, &[u8]); 11] = [
        ("bom.json", b"\xef\xbb\xbf{\"x\":1}"),
        ("bidi.json", br#"{"x":"a\u202eb"}"#),
        ("crlf.json", b"{\r\n\"x\":1\r\n}"),
        ("duplicate-key.json", br#"{"x":1,"x":2}"#),
        ("escaped-control.json", br#"{"x":"\u001b"}"#),
        ("invalid-utf8.json", b"{\"x\":\"\xff\"}"),
        ("lf.json", b"{\n\"x\":1\n}"),
        ("non-nfc.json", br#"{"x":"e\u0301"}"#),
        ("nul.json", br#"{"x":"\u0000"}"#),
        ("raw-control.json", b"{\"x\":\"\x1b\"}"),
        ("zero-width.json", br#"{"x":"a\u200bb"}"#),
    ];
    cases
        .into_iter()
        .map(|(name, bytes)| {
            (
                PathBuf::from("fixtures/hostile/cases").join(name),
                bytes.to_vec(),
            )
        })
        .collect()
}

fn policy_snapshot(
    template: PolicyTemplateV1,
    schema_bundle_hash: crate::Blake3Digest,
    invariant_registry_hash: crate::Blake3Digest,
) -> PolicySnapshotV1 {
    PolicySnapshotV1 {
        schema_version: template.schema_version,
        policy_generation: template.policy_generation,
        schema_bundle_hash,
        invariant_registry_hash,
        activation_at_unix_ms: template.activation_at_unix_ms,
        expires_at_unix_ms: template.expires_at_unix_ms,
        issuer_keys: template.issuer_keys,
        risk_policy: template.risk_policy,
        evidence_policy: template.evidence_policy,
        sandbox_policy: template.sandbox_policy,
        budget_policy: template.budget_policy,
        route_policy: template.route_policy,
    }
}

fn fixture_manifest(team: &[u8]) -> Result<Value, WireError> {
    let sha256 = Sha256::digest(team);
    Ok(json!({
        "fixtures": [{
            "blake3": hash_framed_bytes("hostile.fixture", team)?,
            "bytes": team.len(),
            "classification": "hostile_forensic_input",
            "path": HOSTILE_TEAM,
            "runtime_authority": false,
            "sha256": format!("{sha256:x}")
        }],
        "schema_version": "v1alpha1"
    }))
}

fn canonical_golden() -> Result<(Value, String, crate::Blake3Digest), WireError> {
    let value = json!({"a": "é", "array": [true, null, 17], "z": "last"});
    let bytes = canonical_json(&value)?;
    let text = String::from_utf8(bytes.clone())
        .map_err(|error| WireError::new("GOLDEN_ENCODING_FAILED", error.to_string()))?;
    let hash = hash_framed_bytes("golden.cross-language", &bytes)?;
    Ok((
        json!({
            "canonical_json_utf8": text,
            "domain": "golden.cross-language",
            "framed_blake3": hash,
            "schema_version": "v1alpha1"
        }),
        text,
        hash,
    ))
}

fn authority_golden() -> Result<(Value, crate::Blake3Digest), WireError> {
    const SECRET_KEY: [u8; 64] = [
        180, 203, 251, 67, 223, 76, 226, 16, 114, 125, 149, 62, 74, 113, 51, 7, 250, 25, 187, 125,
        159, 133, 4, 20, 56, 217, 225, 27, 148, 42, 55, 116, 30, 185, 219, 187, 188, 4, 124, 3,
        253, 112, 96, 78, 0, 113, 240, 152, 126, 22, 178, 139, 117, 114, 37, 193, 31, 0, 65, 93,
        14, 32, 177, 162,
    ];
    const PUBLIC_KEY_HEX: &str = "1eb9dbbbbc047c03fd70604e0071f0987e16b28b757225c11f00415d0e20b1a2";
    let request = crate::v1alpha1::ApplyPatchRequestV1 {
        schema_version: "v1alpha1".to_owned(),
        mutation_id: format!("mut_{}", "1".repeat(64)),
        repository_id: format!("rep_{}", "4".repeat(64)),
        workspace_id: format!("wsp_{}", "f".repeat(64)),
        workspace_generation: 7,
        proposal: crate::v1alpha1::PatchProposalV1 {
            schema_version: "v1alpha1".to_owned(),
            proposal_id: format!("cnt_{}", "a".repeat(64)),
            producing_attempt_id: format!("atm_{}", "d".repeat(64)),
            base_checkpoint_id: format!("ckp_{}", "5".repeat(64)),
            base_checkpoint_digest: "6".repeat(64),
            operations: vec![crate::v1alpha1::PatchOperationV1 {
                schema_version: "v1alpha1".to_owned(),
                path: "src/lib.rs".to_owned(),
                preimage_kind: crate::v1alpha1::PatchPreimageKindV1::Digest,
                preimage_digest: Some("7".repeat(64)),
                mutation_kind: crate::v1alpha1::PatchMutationKindV1::Write,
                content_utf8: Some("pub fn golden() {}\n".to_owned()),
            }],
            gate_ids: vec![format!("gat_{}", "8".repeat(64))],
        },
    };
    let request_digest = authority_request_digest(&request)?;
    let claims = AuthorityClaims {
        schema_version: "v1alpha1".to_owned(),
        issuer: "bullet-kernel-local".to_owned(),
        audience: AuthorityAudience::BulletGitd,
        operation: MutationOperation::ApplyPatch,
        request_digest,
        mutation_id: parse_id("mut_", '1')?,
        subject_principal: parse_id("pri_", '2')?,
        organization_id: parse_id("org_", '3')?,
        repository_id: parse_id("rep_", '4')?,
        mission_id: parse_id("mis_", '5')?,
        acceptance_contract_id: parse_id("acc_", '6')?,
        plan_revision_id: parse_id("pln_", '7')?,
        graph_revision_id: parse_id("grf_", '8')?,
        graph_sequence: 9,
        work_package_id: parse_id("wpk_", 'a')?,
        selection_group_id: parse_id("sel_", 'b')?,
        variant_id: parse_id("var_", 'c')?,
        attempt_id: parse_id("atm_", 'd')?,
        attempt_fence: 10,
        runner_id: parse_id("run_", 'e')?,
        runner_epoch: 11,
        workspace_id: parse_id("wsp_", 'f')?,
        workspace_generation: request.workspace_generation,
        workspace_nonce: crate::Blake3Digest::from_bytes([12; 32]),
        scope_grant_digest: crate::Blake3Digest::from_bytes([13; 32]),
        scope_revision: 14,
        context_revision: 15,
        configuration_snapshot_id: parse_id("cnt_", '1')?,
        configuration_generation: 16,
        policy_snapshot_id: parse_id("cnt_", '2')?,
        policy_generation: 17,
        routing_snapshot_id: parse_id("cnt_", '3')?,
        routing_generation: 18,
        provider: "claude".to_owned(),
        model: "claude-test".to_owned(),
        adapter: "claude-stream-json-v1".to_owned(),
        provider_profile_id: parse_id("prf_", '4')?,
        credential_generation: 19,
        authority_epoch: 20,
        freeze_generation: 0,
        issued_at_unix_ms: 1_800_000_000_000,
        not_before_unix_ms: 1_800_000_000_000,
        expires_at_unix_ms: 1_800_000_015_000,
        token_nonce: crate::Blake3Digest::from_bytes([21; 32]),
    };
    let signer =
        AuthoritySigningKey::from_bytes("bullet-kernel-local", "authority-test-1", &SECRET_KEY)?;
    let envelope = signer.sign_for_request(&claims, &request)?;
    let envelope_digest = envelope.digest()?;
    let reservation_id = parse_id("rsv_", '9')?;
    let permit_claims = crate::MutationPermitClaims {
        schema_version: "v1alpha1".to_owned(),
        issuer: "bullet-kernel-local".to_owned(),
        audience: AuthorityAudience::BulletGitd,
        operation: MutationOperation::ApplyPatch,
        authority_envelope_digest: envelope_digest,
        authority_token_nonce: claims.token_nonce,
        mutation_id: claims.mutation_id.clone(),
        reservation_id,
        request_digest,
        repository_id: claims.repository_id.clone(),
        workspace_id: claims.workspace_id.clone(),
        workspace_generation: request.workspace_generation,
        attempt_id: claims.attempt_id.clone(),
        attempt_fence: claims.attempt_fence,
        authority_epoch: claims.authority_epoch,
        freeze_generation: claims.freeze_generation,
        issued_at_unix_ms: 1_800_000_000_100,
        not_before_unix_ms: 1_800_000_000_100,
        expires_at_unix_ms: 1_800_000_001_100,
        permit_nonce: crate::Blake3Digest::from_bytes([22; 32]),
    };
    let permit = signer.sign_mutation_permit(&permit_claims)?;
    let permit_digest = permit.digest()?;
    let decision = crate::FinalAuthorityDecision {
        schema_version: "v1alpha1".to_owned(),
        decision: crate::AuthorityDecisionKind::Authorized,
        replay: crate::ReplayDisposition::Fresh,
        mutation_id: claims.mutation_id.clone(),
        operation: MutationOperation::ApplyPatch,
        request_digest,
        reservation_id: Some(permit_claims.reservation_id.clone()),
        permit: Some(permit.clone()),
        replay_result: None,
        reason_code: None,
    };
    decision.validate_shape()?;
    let settlement_request = crate::MutationSettlementRequest {
        schema_version: "v1alpha1".to_owned(),
        reservation_id: permit_claims.reservation_id.clone(),
        mutation_id: claims.mutation_id.clone(),
        operation: MutationOperation::ApplyPatch,
        request_digest,
        permit: permit.clone(),
        permit_digest,
        outcome: crate::MutationOutcome::Committed,
        result_digest: crate::Blake3Digest::from_bytes([23; 32]),
        completed_at_unix_ms: 1_800_000_000_900,
    };
    settlement_request.validate_shape()?;
    let settlement_result = crate::MutationSettlementResult {
        schema_version: "v1alpha1".to_owned(),
        status: crate::SettlementStatus::Accepted,
        replay: crate::ReplayDisposition::Fresh,
        mutation_id: claims.mutation_id.clone(),
        reservation_id: permit_claims.reservation_id.clone(),
        result_digest: Some(settlement_request.result_digest),
        reason_code: None,
    };
    settlement_result.validate()?;
    let value = json!({
        "claims_canonical_json": String::from_utf8(canonical_json(&claims)?)
            .map_err(|error| WireError::new("GOLDEN_ENCODING_FAILED", error.to_string()))?,
        "claims_digest": claims.digest()?,
        "envelope": envelope,
        "envelope_digest": envelope_digest,
        "implicit_assertion_utf8": "bullet-farm.authority.v1alpha1",
        "mutation_decision": decision,
        "mutation_permit": permit,
        "mutation_permit_claims_canonical_json": String::from_utf8(canonical_json(&permit_claims)?)
            .map_err(|error| WireError::new("GOLDEN_ENCODING_FAILED", error.to_string()))?,
        "mutation_permit_digest": permit_digest,
        "mutation_permit_implicit_assertion_utf8": "bullet-farm.mutation-permit.v1alpha1",
        "public_key_hex": PUBLIC_KEY_HEX,
        "request": request,
        "request_digest": request_digest,
        "request_domain": MutationOperation::ApplyPatch.request_domain(),
        "schema_version": "v1alpha1",
        "settlement_request": settlement_request,
        "settlement_result": settlement_result,
    });
    let digest = hash_canonical("authority.golden.v1alpha1", &value)?;
    Ok((value, digest))
}

fn parse_id<T>(prefix: &str, fill: char) -> Result<T, WireError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    format!("{prefix}{}", fill.to_string().repeat(64))
        .parse::<T>()
        .map_err(|error| WireError::new("GOLDEN_ID_FAILED", error.to_string()))
}

fn read(root: &Path, relative: &str) -> Result<Vec<u8>, WireError> {
    fs::read(root.join(relative)).map_err(|error| {
        WireError::new(
            "CONTRACT_INPUT_READ_FAILED",
            format!("cannot read {relative}: {error}"),
        )
    })
}

fn write_outputs(root: &Path, outputs: &BTreeMap<PathBuf, Vec<u8>>) -> Result<(), WireError> {
    for (relative, bytes) in outputs {
        let destination = root.join(relative);
        let parent = destination.parent().ok_or_else(|| {
            WireError::new("CONTRACT_OUTPUT_PATH", "generated output has no parent")
        })?;
        fs::create_dir_all(parent).map_err(io_error("create output directory", relative))?;
        let staged_output = destination.with_extension("bullet-contract.tmp");
        let mut file = File::create(&staged_output).map_err(io_error("create output", relative))?;
        file.write_all(bytes)
            .map_err(io_error("write output", relative))?;
        file.sync_all().map_err(io_error("sync output", relative))?;
        fs::rename(&staged_output, &destination).map_err(io_error("replace output", relative))?;
    }
    Ok(())
}

fn check_outputs(root: &Path, outputs: &BTreeMap<PathBuf, Vec<u8>>) -> Result<(), WireError> {
    for (relative, expected) in outputs {
        let actual = fs::read(root.join(relative)).map_err(io_error("read output", relative))?;
        if &actual != expected {
            return Err(WireError::new(
                "CONTRACT_DRIFT",
                format!("{} differs from generated bytes", relative.display()),
            ));
        }
    }
    Ok(())
}

fn io_error<'a>(action: &'a str, path: &'a Path) -> impl Fn(std::io::Error) -> WireError + 'a {
    move |error| {
        WireError::new(
            "CONTRACT_IO_FAILED",
            format!("{action} {}: {error}", path.display()),
        )
    }
}
