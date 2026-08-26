mod authority;
mod canonical;
mod catalog;
mod contract_bindings;
mod contract_tool;
mod digest;
mod error;
mod event;
mod forge_profile;
mod ids;
mod ipc;
mod manifest;
mod outcome;
mod policy;
mod proposal;
mod release;

pub use authority::{
    AUTHORITY_IMPLICIT_ASSERTION, AUTHORITY_SCHEMA_VERSION, AuthorityAudience, AuthorityClaims,
    AuthorityDecisionKind, AuthorityExpectation, AuthorityRequest, AuthorityRequestBinding,
    AuthoritySigningKey, AuthorityVerificationKey, FinalAuthorityCheckRequest,
    FinalAuthorityDecision, LAUNCH_GRANT_CLAIMS_DOMAIN, LAUNCH_GRANT_ENVELOPE_DOMAIN,
    LAUNCH_GRANT_ENVIRONMENT_DOMAIN, LAUNCH_GRANT_IMPLICIT_ASSERTION, LAUNCH_GRANT_POLICY_DOMAIN,
    LAUNCH_GRANT_SIGNING_PURPOSE, LAUNCH_GRANT_WORKSPACE_NONCE_DOMAIN, LaunchGrantClaims,
    LaunchGrantExpectation, LaunchLeaseSubject, LaunchOperation, LaunchProvider,
    LaunchProviderSubject, MAX_AUTHORITY_TTL_MS, MAX_LAUNCH_GRANT_GATE_IDS,
    MAX_LAUNCH_GRANT_TTL_MS, MAX_MUTATION_PERMIT_TTL_MS, MUTATION_PERMIT_IMPLICIT_ASSERTION,
    MutationOperation, MutationOutcome, MutationPermitClaims, MutationPermitExpectation,
    MutationPermitSubject, MutationReplayResult, MutationResultState, MutationSettlementRequest,
    MutationSettlementResult, PreservationDecision, ReplayDisposition, SettlementStatus,
    SignedAuthorityEnvelope, SignedLaunchGrant, SignedMutationPermit, authority_request_digest,
    environment_digest, policy_snapshot_digest, workspace_nonce_digest,
};
pub use canonical::{
    MAX_CANONICAL_DOCUMENT_BYTES, MAX_UNIQUE_DOCUMENT_BYTES, decode_canonical,
    decode_canonical_value, decode_unique_value, decode_unique_value_bounded,
};
pub use catalog::*;
pub use contract_tool::{ContractMode, execute as execute_contract_tool};
pub use digest::{Blake3Digest, canonical_json, hash_canonical, hash_framed_bytes};
pub use error::WireError;
pub use event::{CommandEnvelope, CommandState, EventEnvelope, Snapshot};
pub use forge_profile::{
    FORGE_PROFILE_SCHEMA_VERSION, ForgeCapability, ForgeKind, ForgeProfileId, ForgeProfileRegistry,
    IntegrationSubjectBinding, MAX_FORGE_BASE_URL_BYTES, MAX_REPLICATION_REFS,
    PRIMARY_FORGE_PROFILE_DIGEST_DOMAIN, PrimaryForgeProfileV1, REPLICATION_INTENT_DIGEST_DOMAIN,
    ReplicationIntentKind, ReplicationIntentV1, decode_primary_forge_profile,
    decode_replication_intent,
};
pub use ids::*;
pub use ipc::*;
pub use manifest::{
    CandidateManifest as ComponentCandidateManifest,
    CandidateProofManifest as ComponentCandidateProofManifest,
    CheckpointManifest as ComponentCheckpointManifest,
    IntegrationProofManifest as ComponentIntegrationProofManifest,
    PreservationReceiptManifest as ComponentPreservationReceiptManifest,
};
pub use outcome::*;
pub use policy::*;
pub use proposal::*;
pub use release::{
    RELEASE_BUNDLE_MANIFEST_V2_DIGEST_DOMAIN, RELEASE_BUNDLE_MANIFEST_V2_NATIVE_SUBJECT_PREFIX,
    RELEASE_GATE_RECEIPT_DIGEST_DOMAIN, RELEASE_GATE_RECEIPT_SIGNATURE_DOMAIN,
    RELEASE_GATE_SPEC_DIGEST_DOMAIN, RELEASE_PROFILE_GRAPH_DIGEST_DOMAIN,
    RELEASE_REGISTRY_MANIFEST_DIGEST_DOMAIN, RELEASE_REGISTRY_MANIFEST_SIGNATURE_DOMAIN,
    RELEASE_REGISTRY_OBJECT_DIGEST_DOMAIN, RELEASE_SIGNER_POLICY_DIGEST_DOMAIN,
    RELEASE_SIGNER_POLICY_SIGNATURE_DOMAIN, RELEASE_SOURCE_SUBJECT_DIGEST_DOMAIN,
    RELEASE_TRUSTED_TIME_DIGEST_DOMAIN, RELEASE_TRUSTED_TIME_SIGNATURE_DOMAIN,
    RELEASE_VERIFICATION_REQUEST_DIGEST_DOMAIN, ReleaseWireRecord, decode_release_record,
    release_bundle_manifest_v2_digest, validate_release_bindings,
    validate_release_bundle_manifest_v2_binding,
};

/// Normative generated wire records. Security-sensitive consumers must decode this namespace.
pub mod v1alpha1 {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../contracts/generated/rust/schema_bundle.rs"
    ));
}

pub const SCHEMA_VERSION: u32 = 1;
