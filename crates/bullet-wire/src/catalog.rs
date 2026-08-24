use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use crate::{WireError, policy::POLICY_SCHEMA_VERSION};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldTypeV1 {
    String,
    Identifier,
    Digest,
    U64,
    Timestamp,
    Boolean,
    Object,
    StringArray,
    ObjectArray,
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
        let schemas = self
            .records
            .iter()
            .map(|record| (record.name.clone(), record_schema(record)))
            .collect::<Map<_, _>>();
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "bundle_version": self.catalog_version,
            "schema_version": self.schema_version,
            "schemas": schemas,
        })
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

fn record_schema(record: &ContractRecordV1) -> Value {
    let properties = record
        .fields
        .iter()
        .map(|field| (field.name.clone(), field_schema(field)))
        .collect::<Map<_, _>>();
    let required = record
        .fields
        .iter()
        .map(|field| Value::String(field.name.clone()))
        .collect::<Vec<_>>();
    json!({
        "$id": format!("https://schemas.bullet.farm/v1alpha1/{}.json", record.name),
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "additionalProperties": false,
        "properties": properties,
        "required": required,
        "title": record.name,
        "type": "object",
        "x-bullet-security-class": record.security_class,
        "x-bullet-unknown-fields": record.unknown_fields,
    })
}

fn field_schema(field: &ContractFieldV1) -> Value {
    match field.field_type {
        FieldTypeV1::String => json!({"type": "string", "minLength": 1}),
        FieldTypeV1::Identifier => json!({
            "type": "string",
            "pattern": "^[a-z][a-z0-9-]{1,15}_[0-9a-f]{64}$"
        }),
        FieldTypeV1::Digest => json!({"type": "string", "pattern": "^[0-9a-f]{64}$"}),
        FieldTypeV1::U64 => {
            json!({"type": "integer", "minimum": 0, "maximum": 9007199254740991_u64})
        }
        FieldTypeV1::Timestamp => {
            json!({"type": "integer", "minimum": 0, "maximum": 9007199254740991_u64})
        }
        FieldTypeV1::Boolean => json!({"type": "boolean"}),
        FieldTypeV1::Object => json!({"type": "object"}),
        FieldTypeV1::StringArray => json!({"type": "array", "items": {"type": "string"}}),
        FieldTypeV1::ObjectArray => json!({"type": "array", "items": {"type": "object"}}),
    }
}

fn required_records() -> BTreeSet<&'static str> {
    TRANSACTION_RECORDS
        .iter()
        .chain(RESEARCH_RECORDS.iter())
        .copied()
        .collect()
}

const TRANSACTION_RECORDS: &[&str] = &[
    "AcceptanceContractV1",
    "AuditBatchV1",
    "CandidateManifestV1",
    "CheckIntentV1",
    "DeliveryGrantV1",
    "EffectIntentV1",
    "EvidenceV1",
    "GateReceiptV1",
    "GraphDeltaV1",
    "IntegrationIntentV1",
    "InterventionV1",
    "LaunchGrantV1",
    "ObservationV1",
    "PlanRevisionV1",
    "PolicySnapshotV1",
    "ProofBundleV1",
    "ScopeGrantV1",
    "SignedAuthorityEnvelopeV1",
    "VerificationIntentV1",
];

const RESEARCH_RECORDS: &[&str] = &[
    "ActivationPointer",
    "AdjudicationReceipt",
    "AggregateEvaluationV1",
    "AllocationReceiptV1",
    "ArchivePolicyV1",
    "AttackProposal",
    "BehaviorTraceV1",
    "CertificationKey",
    "CertificationRecord",
    "ContaminationDecision",
    "DecertificationReceipt",
    "DelayedOutcomeLink",
    "DriftSignal",
    "EvaluationVectorV1",
    "ExperimentProtocolV1",
    "ExposureEdge",
    "FailureClass",
    "HoldoutLease",
    "HoldoutSuiteManifest",
    "HumanDecision",
    "OpponentSetSnapshot",
    "OverrideReceipt",
    "PromotionDecision",
    "QueryIntent",
    "QueryReceipt",
    "ReviewReceipt",
    "ReviewerAssignment",
    "RouteDecision",
    "RouteRequest",
    "RouterUpdateBatch",
    "SanitizationReceipt",
    "SentinelResult",
    "SystemFingerprint",
    "TaskCorpusManifestV1",
    "TaskSpecV1",
    "TeamRecipeV1",
];
