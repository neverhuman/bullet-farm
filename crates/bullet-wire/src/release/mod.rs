//! Strict, canonical release-registry records.
//!
//! These records are wire components only. Decoding and structural validation
//! do not perform signature verification, semantic gate adjudication, or clear
//! a release gate.

mod fields;
mod validate;

pub use validate::{ReleaseWireRecord, decode_release_record};
