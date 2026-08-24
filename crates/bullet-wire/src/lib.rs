mod authority;
mod digest;
mod error;
mod event;
mod ids;
mod manifest;
mod outcome;
mod proposal;

pub use authority::{AuthorityClaims, MutationOperation, PreservationDecision};
pub use digest::{Blake3Digest, canonical_json, hash_canonical};
pub use error::WireError;
pub use event::{CommandEnvelope, CommandState, EventEnvelope, Snapshot};
pub use ids::*;
pub use manifest::*;
pub use outcome::*;
pub use proposal::*;

pub const SCHEMA_VERSION: u32 = 1;
