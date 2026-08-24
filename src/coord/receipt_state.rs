use std::collections::{BTreeMap, BTreeSet};

use super::{
    CoordError,
    model::{ClaimState, ClaimSummary, Record},
    state::normalized_paths,
    validate_commit_oid, validate_field,
};

pub(super) fn apply(
    record: &Record,
    claims: &mut BTreeMap<String, ClaimSummary>,
) -> Result<(), CoordError> {
    match record {
        Record::CommitReceipt {
            at_unix_ms,
            claim_id,
            orchestrator,
            commit_oid,
            committed_paths,
            ..
        } => apply_receipt(
            claims,
            *at_unix_ms,
            claim_id,
            orchestrator,
            commit_oid,
            committed_paths,
        ),
        Record::CommitReceiptCorrection {
            at_unix_ms,
            claim_id,
            orchestrator,
            previous_commit_oid,
            commit_oid,
            committed_paths,
            reason,
            ..
        } => {
            validate_field("reason", reason).map_err(as_corrupt)?;
            validate_commit_oid(previous_commit_oid).map_err(as_corrupt)?;
            let claim = claims.get(claim_id).ok_or_else(|| {
                corrupt(format!("correction references missing claim {claim_id}"))
            })?;
            if claim.commit_oid.as_deref() != Some(previous_commit_oid) {
                return Err(corrupt(format!(
                    "claim {claim_id} correction does not bind its current commit"
                )));
            }
            apply_existing_receipt(
                claims,
                *at_unix_ms,
                claim_id,
                orchestrator,
                commit_oid,
                committed_paths,
            )
        }
        Record::CommitReceiptGroup {
            at_unix_ms,
            orchestrator,
            commit_oid,
            receipts,
            ..
        } => apply_group(claims, *at_unix_ms, orchestrator, commit_oid, receipts),
        _ => Err(corrupt("receipt replay received a non-receipt record")),
    }
}

fn apply_receipt(
    claims: &mut BTreeMap<String, ClaimSummary>,
    at: u64,
    claim_id: &str,
    orchestrator: &str,
    commit_oid: &str,
    paths: &[String],
) -> Result<(), CoordError> {
    let claim = claims
        .get(claim_id)
        .ok_or_else(|| corrupt(format!("receipt references missing claim {claim_id}")))?;
    if claim.state != ClaimState::HandedOff || claim.commit_oid.is_some() {
        return Err(corrupt(format!(
            "claim {claim_id} has an invalid or duplicate commit receipt"
        )));
    }
    apply_existing_receipt(claims, at, claim_id, orchestrator, commit_oid, paths)
}

fn apply_existing_receipt(
    claims: &mut BTreeMap<String, ClaimSummary>,
    at: u64,
    claim_id: &str,
    orchestrator: &str,
    commit_oid: &str,
    paths: &[String],
) -> Result<(), CoordError> {
    validate_field("orchestrator", orchestrator).map_err(as_corrupt)?;
    validate_commit_oid(commit_oid).map_err(as_corrupt)?;
    let normalized = normalized_paths(paths).map_err(as_corrupt)?;
    if normalized != paths {
        return Err(corrupt(format!(
            "claim {claim_id} has noncanonical committed paths"
        )));
    }
    let claim = claims
        .get_mut(claim_id)
        .ok_or_else(|| corrupt(format!("receipt references missing claim {claim_id}")))?;
    if claim.changed_paths != paths {
        return Err(corrupt(format!(
            "claim {claim_id} receipt paths differ from its handoff"
        )));
    }
    if at < claim.last_event_unix_ms {
        return Err(corrupt(format!("claim {claim_id} time moved backwards")));
    }
    claim.last_event_unix_ms = at;
    claim.commit_oid = Some(commit_oid.to_owned());
    claim.commit_orchestrator = Some(orchestrator.to_owned());
    claim.commit_recorded_at_unix_ms = Some(at);
    Ok(())
}

fn apply_group(
    claims: &mut BTreeMap<String, ClaimSummary>,
    at: u64,
    orchestrator: &str,
    commit_oid: &str,
    receipts: &[super::model::GroupReceipt],
) -> Result<(), CoordError> {
    validate_field("orchestrator", orchestrator).map_err(as_corrupt)?;
    validate_commit_oid(commit_oid).map_err(as_corrupt)?;
    if receipts.is_empty() {
        return Err(corrupt("group receipt has no claims"));
    }
    let mut seen = BTreeSet::new();
    let mut previous = None;
    for receipt in receipts {
        if !seen.insert(receipt.claim_id.as_str())
            || previous.is_some_and(|value| value >= receipt.claim_id.as_str())
        {
            return Err(corrupt("group receipt claim IDs are not unique and sorted"));
        }
        previous = Some(receipt.claim_id.as_str());
        apply_receipt(
            claims,
            at,
            &receipt.claim_id,
            orchestrator,
            commit_oid,
            &receipt.committed_paths,
        )?;
    }
    Ok(())
}

fn as_corrupt(error: CoordError) -> CoordError {
    corrupt(error.to_string())
}

fn corrupt(reason: impl Into<String>) -> CoordError {
    CoordError::new("CORRUPT_COORD_LOG", reason)
}
