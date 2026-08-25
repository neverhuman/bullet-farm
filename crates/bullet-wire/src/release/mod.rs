//! Strict, canonical release-registry records.
//!
//! These records are wire components only. Decoding and structural validation
//! do not perform signature verification, semantic gate adjudication, or clear
//! a release gate.

mod bindings;
mod fields;
mod profile;
mod validate;

pub const RELEASE_GATE_RECEIPT_DIGEST_DOMAIN: &str = "release.gate-receipt.v1alpha1";
pub const RELEASE_GATE_RECEIPT_SIGNATURE_DOMAIN: &str = "release.gate-receipt-signature.v1alpha1";
pub const RELEASE_GATE_SPEC_DIGEST_DOMAIN: &str = "release.gate-spec.v1alpha1";
pub const RELEASE_PROFILE_GRAPH_DIGEST_DOMAIN: &str = "release.profile-graph.v1alpha1";
pub const RELEASE_REGISTRY_MANIFEST_DIGEST_DOMAIN: &str = "release.registry-manifest.v1alpha1";
pub const RELEASE_REGISTRY_MANIFEST_SIGNATURE_DOMAIN: &str =
    "release.registry-manifest-signature.v1alpha1";
pub const RELEASE_REGISTRY_OBJECT_DIGEST_DOMAIN: &str = "release.registry-object.v1alpha1";
pub const RELEASE_SIGNER_POLICY_DIGEST_DOMAIN: &str = "release.signer-policy.v1alpha1";
pub const RELEASE_SIGNER_POLICY_SIGNATURE_DOMAIN: &str = "release.signer-policy-signature.v1alpha1";
pub const RELEASE_SOURCE_SUBJECT_DIGEST_DOMAIN: &str = "release.source-subject.v1alpha1";
pub const RELEASE_TRUSTED_TIME_DIGEST_DOMAIN: &str = "release.trusted-time.v1alpha1";
pub const RELEASE_TRUSTED_TIME_SIGNATURE_DOMAIN: &str = "release.trusted-time-signature.v1alpha1";
pub const RELEASE_VERIFICATION_REQUEST_DIGEST_DOMAIN: &str =
    "release.verification-request.v1alpha1";

pub use bindings::validate_release_bindings;
pub use validate::{ReleaseWireRecord, decode_release_record};
