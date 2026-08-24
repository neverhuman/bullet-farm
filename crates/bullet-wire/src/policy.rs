use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{AuthorityAudience, Blake3Digest, WireError};

mod keys;

pub const POLICY_SCHEMA_VERSION: &str = "v1alpha1";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementTier {
    T1Schema,
    T2Gateway,
    T3Test,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvariantLifecycle {
    Planned,
    Enforced,
    Retired,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InvariantEntryV1 {
    pub id: String,
    pub legacy_aliases: Vec<String>,
    pub control_ids: Vec<String>,
    pub statement: String,
    pub tier: EnforcementTier,
    pub lifecycle: InvariantLifecycle,
    pub first_applicable_wave: u8,
    pub trust_plane: String,
    pub owner: String,
    pub enforcement_target: String,
    pub proof_command: String,
    pub gate: String,
    pub threat_class: String,
    pub violation_event: String,
    pub violation_mode: String,
    pub milestone: String,
    pub introduced_in: String,
    pub documentation_anchor: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InvariantRegistryV1 {
    pub schema_version: String,
    pub registry_version: String,
    pub entries: Vec<InvariantEntryV1>,
}

impl InvariantRegistryV1 {
    pub fn validate(&self) -> Result<(), WireError> {
        require_v1alpha1(&self.schema_version, "InvariantRegistryV1")?;
        if self.registry_version.is_empty() || self.entries.is_empty() {
            return Err(WireError::new(
                "INVALID_INVARIANT_REGISTRY",
                "registry version and entries are required",
            ));
        }
        let mut ids = BTreeSet::new();
        let mut aliases = BTreeSet::new();
        let mut controls = BTreeSet::new();
        for entry in &self.entries {
            validate_entry(entry)?;
            if !ids.insert(entry.id.as_str()) {
                return Err(duplicate("invariant ID", &entry.id));
            }
            for alias in &entry.legacy_aliases {
                if !aliases.insert(alias.as_str()) {
                    return Err(duplicate("legacy alias", alias));
                }
            }
            for control in &entry.control_ids {
                if !controls.insert(control.as_str()) {
                    return Err(duplicate("control crosswalk ID", control));
                }
            }
        }
        if ids.iter().any(|id| aliases.contains(*id)) {
            return Err(WireError::new(
                "INVARIANT_ALIAS_COLLISION",
                "an invariant ID is reused as a legacy alias",
            ));
        }
        require_control_crosswalk(&controls)
    }
}

fn validate_entry(entry: &InvariantEntryV1) -> Result<(), WireError> {
    if !valid_id(&entry.id) || entry.control_ids.iter().any(|id| !valid_id(id)) {
        return Err(WireError::new(
            "INVALID_INVARIANT_ID",
            format!("{} contains an invalid stable or control ID", entry.id),
        ));
    }
    let always_required = [
        &entry.statement,
        &entry.trust_plane,
        &entry.owner,
        &entry.gate,
        &entry.threat_class,
        &entry.violation_event,
        &entry.violation_mode,
        &entry.introduced_in,
        &entry.documentation_anchor,
    ];
    if always_required.iter().any(|value| value.is_empty()) {
        return Err(WireError::new(
            "INCOMPLETE_INVARIANT",
            format!("{} omits required traceability", entry.id),
        ));
    }
    match entry.lifecycle {
        InvariantLifecycle::Enforced => {
            if entry.enforcement_target.is_empty() || entry.proof_command.is_empty() {
                return Err(WireError::new(
                    "ENFORCED_INVARIANT_WITHOUT_PROOF",
                    format!("{} lacks an enforcement target or proof command", entry.id),
                ));
            }
        }
        InvariantLifecycle::Planned => {
            if entry.milestone.is_empty() || entry.first_applicable_wave <= 1 {
                return Err(WireError::new(
                    "INVALID_PLANNED_INVARIANT",
                    format!(
                        "{} is planned without a future wave and milestone",
                        entry.id
                    ),
                ));
            }
        }
        InvariantLifecycle::Retired => {
            if entry.milestone.is_empty() {
                return Err(WireError::new(
                    "INVALID_RETIRED_INVARIANT",
                    format!("{} does not name its replacement or retirement", entry.id),
                ));
            }
        }
    }
    Ok(())
}

fn require_control_crosswalk(actual: &BTreeSet<&str>) -> Result<(), WireError> {
    let expected = expected_control_ids();
    let expected_refs = expected.iter().map(String::as_str).collect::<BTreeSet<_>>();
    if actual != &expected_refs {
        let missing = expected_refs
            .difference(actual)
            .copied()
            .collect::<Vec<_>>();
        let extra = actual
            .difference(&expected_refs)
            .copied()
            .collect::<Vec<_>>();
        return Err(WireError::new(
            "INVARIANT_CROSSWALK_MISMATCH",
            format!("missing {missing:?}; extra {extra:?}"),
        ));
    }
    Ok(())
}

fn expected_control_ids() -> Vec<String> {
    let mut ids = (1..=12)
        .map(|index| format!("C{index}"))
        .collect::<Vec<_>>();
    ids.push("C6B".to_owned());
    for index in 1..=29 {
        let base = format!("EV{index:02}");
        match index {
            1 | 5 | 17 => ids.extend([format!("{base}A"), format!("{base}B")]),
            10 | 11 => ids.extend([format!("{base}A"), format!("{base}B"), format!("{base}C")]),
            _ => ids.push(base),
        }
    }
    ids
}

fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'-')
}

fn duplicate(kind: &str, value: &str) -> WireError {
    WireError::new(
        "DUPLICATE_INVARIANT_ID",
        format!("duplicate {kind} {value}"),
    )
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum KeyPurposeV1 {
    AuthoritySigning,
    ReleaseSigning,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum KeyAlgorithmV1 {
    #[serde(rename = "paseto-v4.public")]
    PasetoV4Public,
    #[serde(rename = "ssh-ed25519")]
    SshEd25519,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IssuerKeyV1 {
    pub schema_version: String,
    pub issuer: String,
    pub key_id: String,
    pub key_purpose: KeyPurposeV1,
    pub algorithm: KeyAlgorithmV1,
    pub public_key: String,
    pub audiences: Vec<AuthorityAudience>,
    pub activates_at_unix_ms: u64,
    pub expires_at_unix_ms: u64,
    pub revoked_at_unix_ms: Option<u64>,
    pub retain_until_unix_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RiskPolicyV1 {
    pub schema_version: String,
    pub automatic_integration_max_risk: String,
    pub signed_human_approval_min_risk: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidencePolicyV1 {
    pub schema_version: String,
    pub r2_requires_sealed_product_holdout: bool,
    pub author_evidence_is_independent: bool,
    pub unknown_satisfies_gate: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SandboxPolicyV1 {
    pub schema_version: String,
    pub production_reference: String,
    pub arbitrary_shell_gates: bool,
    pub network_default: String,
    pub live_admission_enabled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BudgetPolicyV1 {
    pub schema_version: String,
    pub maximum_lease_ttl_seconds: u64,
    pub unknown_quota_is_headroom: bool,
    pub maximum_changed_paths: u64,
    pub maximum_attempt_seconds: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoutePolicyV1 {
    pub schema_version: String,
    pub universal_incumbent: String,
    pub deterministic_abstention_target: String,
    pub evolutionary_authority: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyTemplateV1 {
    pub schema_version: String,
    pub policy_generation: u64,
    pub activation_at_unix_ms: u64,
    pub expires_at_unix_ms: u64,
    pub issuer_keys: Vec<IssuerKeyV1>,
    pub risk_policy: RiskPolicyV1,
    pub evidence_policy: EvidencePolicyV1,
    pub sandbox_policy: SandboxPolicyV1,
    pub budget_policy: BudgetPolicyV1,
    pub route_policy: RoutePolicyV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicySnapshotV1 {
    pub schema_version: String,
    pub policy_generation: u64,
    pub schema_bundle_hash: Blake3Digest,
    pub invariant_registry_hash: Blake3Digest,
    pub activation_at_unix_ms: u64,
    pub expires_at_unix_ms: u64,
    pub issuer_keys: Vec<IssuerKeyV1>,
    pub risk_policy: RiskPolicyV1,
    pub evidence_policy: EvidencePolicyV1,
    pub sandbox_policy: SandboxPolicyV1,
    pub budget_policy: BudgetPolicyV1,
    pub route_policy: RoutePolicyV1,
}

impl PolicySnapshotV1 {
    pub fn validate(&self) -> Result<(), WireError> {
        require_v1alpha1(&self.schema_version, "PolicySnapshotV1")?;
        if self.policy_generation == 0
            || self.activation_at_unix_ms >= self.expires_at_unix_ms
            || self.issuer_keys.is_empty()
        {
            return Err(WireError::new(
                "INVALID_POLICY_WINDOW",
                "policy requires a generation, issuer key, and ordered validity window",
            ));
        }
        validate_nested_policy_versions(self)?;
        keys::validate_issuer_keys(&self.issuer_keys)?;
        if self.budget_policy.maximum_lease_ttl_seconds > 15
            || self.budget_policy.unknown_quota_is_headroom
            || self.sandbox_policy.live_admission_enabled
            || self.sandbox_policy.arbitrary_shell_gates
            || self.evidence_policy.author_evidence_is_independent
            || self.evidence_policy.unknown_satisfies_gate
            || !self.evidence_policy.r2_requires_sealed_product_holdout
            || self.route_policy.universal_incumbent != "T0"
            || self.route_policy.evolutionary_authority
        {
            return Err(WireError::new(
                "UNSAFE_POLICY",
                "v1alpha1 Gate 0 policy must remain offline, conservative, and T0-anchored",
            ));
        }
        Ok(())
    }
}

fn validate_nested_policy_versions(policy: &PolicySnapshotV1) -> Result<(), WireError> {
    for (name, version) in [
        ("risk_policy", policy.risk_policy.schema_version.as_str()),
        (
            "evidence_policy",
            policy.evidence_policy.schema_version.as_str(),
        ),
        (
            "sandbox_policy",
            policy.sandbox_policy.schema_version.as_str(),
        ),
        (
            "budget_policy",
            policy.budget_policy.schema_version.as_str(),
        ),
        ("route_policy", policy.route_policy.schema_version.as_str()),
    ] {
        require_v1alpha1(version, name)?;
    }
    Ok(())
}

fn require_v1alpha1(actual: &str, kind: &str) -> Result<(), WireError> {
    if actual != POLICY_SCHEMA_VERSION {
        return Err(WireError::new(
            "UNSUPPORTED_POLICY_SCHEMA",
            format!("{kind} schema {actual} is unsupported"),
        ));
    }
    Ok(())
}
