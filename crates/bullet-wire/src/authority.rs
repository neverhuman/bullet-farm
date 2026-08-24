use serde::{Deserialize, Serialize};

use crate::{
    AcceptanceContractId, AttemptId, Blake3Digest, ContentId, GraphRevisionId, MissionId,
    OrganizationId, PlanRevisionId, RepositoryId, RunnerId, SelectionGroupId, VariantId,
    WorkPackageId, WorkspaceId,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationOperation {
    ApplyPatch,
    Checkpoint,
    PrepareCandidate,
    PreserveWorkspace,
    CleanupWorkspace,
    DispatchEffect,
    ReconcileEffect,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityClaims {
    pub schema_version: u32,
    pub audience: String,
    pub operation: MutationOperation,
    pub request_digest: Blake3Digest,
    pub organization_id: OrganizationId,
    pub repository_id: RepositoryId,
    pub mission_id: MissionId,
    pub acceptance_contract_id: AcceptanceContractId,
    pub plan_revision_id: PlanRevisionId,
    pub graph_revision_id: GraphRevisionId,
    pub graph_sequence: u64,
    pub work_package_id: WorkPackageId,
    pub selection_group_id: SelectionGroupId,
    pub variant_id: VariantId,
    pub attempt_id: AttemptId,
    pub attempt_fence: u64,
    pub runner_id: RunnerId,
    pub runner_epoch: u64,
    pub workspace_id: WorkspaceId,
    pub workspace_nonce: Blake3Digest,
    pub scope_revision: u64,
    pub context_revision: u64,
    pub configuration_snapshot_id: ContentId,
    pub policy_snapshot_id: ContentId,
    pub routing_snapshot_id: ContentId,
    pub authority_epoch: u64,
    pub expires_at_unix_ms: u64,
    pub nonce: Blake3Digest,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "decision", rename_all = "snake_case", deny_unknown_fields)]
pub enum PreservationDecision {
    PreserveRequired {
        reason: String,
    },
    CleanupAuthorized {
        preservation_receipt_digest: Blake3Digest,
        expected_destination_digest: Blake3Digest,
    },
    CleanupDenied {
        reason: String,
    },
}
