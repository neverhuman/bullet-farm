use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{WireError, policy::POLICY_SCHEMA_VERSION};

mod constraints;
mod launch;
mod records;
mod schema;
use records::required_records;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldTypeV1 {
    String,
    SchemaVersion,
    PolicySchemaVersion,
    Identifier,
    Digest,
    OrganizationId,
    RepositoryId,
    MissionId,
    AcceptanceContractId,
    PlanRevisionId,
    GraphRevisionId,
    WorkPackageId,
    SelectionGroupId,
    VariantId,
    AttemptId,
    RunnerId,
    WorkspaceId,
    PrincipalId,
    ProviderProfileId,
    ContentId,
    MutationId,
    MutationReservationId,
    ScopeGrantId,
    SourceDescriptorId,
    ChangeId,
    CheckpointId,
    CandidateId,
    GateReceiptId,
    ReleaseRegistryId,
    GateId,
    EffectIntentId,
    CandidateProofRoot,
    IntegrationProofRoot,
    GitOid,
    TaggedBlake3Digest,
    ReleaseGateId,
    ReleaseNativeSubjectId,
    ReleaseProfileId,
    ReleaseTag,
    SigningIdentity,
    SshEd25519PublicKey,
    RepoPath,
    AuthorityAudience,
    MutationOperation,
    AuthorityDecision,
    ReplayDisposition,
    MutationResultState,
    MutationOutcome,
    SettlementStatus,
    PatchPreimageKind,
    PatchMutationKind,
    ReleaseReceiptKind,
    ReleaseEvidenceKind,
    ReleaseRegistryObjectKind,
    ReleaseSignerRole,
    ReleaseRepositoryName,
    KeyId,
    KeyPurpose,
    KeyAlgorithm,
    PasetoV4Public,
    SafeU64,
    U64,
    Timestamp,
    OptionalTimestamp,
    OptionalDigest,
    OptionalString,
    OptionalMutationReservationId,
    Boolean,
    Object,
    StringArray,
    ObjectArray,
    AuthorityAudienceArray,
    IssuerKeyArray,
    RiskPolicy,
    EvidencePolicy,
    SandboxPolicy,
    BudgetPolicy,
    RoutePolicy,
    SignedAuthorityEnvelope,
    SignedMutationPermit,
    OptionalSignedMutationPermit,
    MutationReplayResult,
    OptionalMutationReplayResult,
    ScopeGrant,
    PatchProposal,
    PatchOperationArray,
    CandidateIdArray,
    OrderedCandidateIdArray,
    GateIdArray,
    ReleaseGateIdArray,
    ReleaseProfileIdArray,
    ReleaseEvidenceKindArray,
    RepoPathArray,
    CleanupAuthorization,
    ReleaseFamilySubject,
    ReleaseRepositorySubjectArray,
    ReleaseEvidenceSubjectArray,
    ReleaseProfileNodeArray,
    ReleaseSignerKeyArray,
    ReleaseRegistryEntryArray,
    ReleaseRegistryObjectArray,
    ReleaseReplayBindingArray,
    ExecutionToolArray,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractFieldV1 {
    pub name: String,
    pub field_type: FieldTypeV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractRecordV1 {
    pub name: String,
    pub security_class: String,
    pub unknown_fields: String,
    pub fields: Vec<ContractFieldV1>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractCatalogV1 {
    pub schema_version: String,
    pub catalog_version: String,
    pub records: Vec<ContractRecordV1>,
}

impl ContractCatalogV1 {
    pub fn validate(&self) -> Result<(), WireError> {
        if self.schema_version != POLICY_SCHEMA_VERSION || self.catalog_version.is_empty() {
            return Err(WireError::new(
                "INVALID_CONTRACT_CATALOG",
                "catalog requires v1alpha1 schema and a version",
            ));
        }
        let mut names = BTreeSet::new();
        for record in &self.records {
            validate_record(record)?;
            if !names.insert(record.name.as_str()) {
                return Err(WireError::new(
                    "DUPLICATE_CONTRACT_RECORD",
                    format!("duplicate contract record {}", record.name),
                ));
            }
        }
        let expected = required_records();
        if names != expected {
            let missing = expected.difference(&names).copied().collect::<Vec<_>>();
            let extra = names.difference(&expected).copied().collect::<Vec<_>>();
            return Err(WireError::new(
                "CONTRACT_CATALOG_COVERAGE",
                format!("missing {missing:?}; extra {extra:?}"),
            ));
        }
        Ok(())
    }

    pub fn json_schema_bundle(&self) -> Value {
        schema::json_schema_bundle(self)
    }
}

fn validate_record(record: &ContractRecordV1) -> Result<(), WireError> {
    if !valid_record_name(&record.name)
        || record.security_class.is_empty()
        || record.unknown_fields != "reject"
        || record.fields.is_empty()
    {
        return Err(WireError::new(
            "INVALID_CONTRACT_RECORD",
            format!("{} lacks strict record metadata", record.name),
        ));
    }
    let mut fields = BTreeSet::new();
    for field in &record.fields {
        if !valid_field_name(&field.name) || !fields.insert(field.name.as_str()) {
            return Err(WireError::new(
                "INVALID_CONTRACT_FIELD",
                format!(
                    "{} has invalid or duplicate field {}",
                    record.name, field.name
                ),
            ));
        }
    }
    if !fields.contains("schema_version") {
        return Err(WireError::new(
            "MISSING_SCHEMA_VERSION",
            format!("{} does not bind schema_version", record.name),
        ));
    }
    Ok(())
}

fn valid_field_name(value: &str) -> bool {
    let mut bytes = value.bytes();
    value.len() <= 80
        && bytes.next().is_some_and(|byte| byte.is_ascii_lowercase())
        && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        && !RESERVED_FIELDS.contains(&value)
}

fn valid_record_name(value: &str) -> bool {
    let mut bytes = value.bytes();
    value.len() <= 80
        && bytes.next().is_some_and(|byte| byte.is_ascii_uppercase())
        && bytes.all(|byte| byte.is_ascii_alphanumeric())
}

const RESERVED_FIELDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while",
];
