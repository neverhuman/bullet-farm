mod authority;
mod canonical;
mod catalog;
mod contract_bindings;
mod contract_tool;
mod digest;
mod error;
mod event;
mod ids;
mod manifest;
mod outcome;
mod policy;
mod proposal;

pub use authority::{
    AuthorityClaims as ComponentAuthorityClaims, MutationOperation, PreservationDecision,
};
pub use canonical::{MAX_CANONICAL_DOCUMENT_BYTES, decode_canonical, decode_canonical_value};
pub use catalog::*;
pub use contract_tool::{ContractMode, execute as execute_contract_tool};
pub use digest::{Blake3Digest, canonical_json, hash_canonical, hash_framed_bytes};
pub use error::WireError;
pub use event::{CommandEnvelope, CommandState, EventEnvelope, Snapshot};
pub use ids::*;
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

/// Normative generated wire records. Security-sensitive consumers must decode this namespace.
pub mod v1alpha1 {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../contracts/generated/rust/schema_bundle.rs"
    ));
}

pub const SCHEMA_VERSION: u32 = 1;
