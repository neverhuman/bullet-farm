use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use fs2::FileExt;

use super::{
    ClaimInput, CommitReceiptGroupInput, CommitReceiptInput, CoordError, HandoffInput,
    HeartbeatInput, ReceiptCorrectionInput,
    git::verify_commit_paths,
    model::{ClaimState, ClaimSummary, GroupReceipt, Record, SCHEMA_VERSION, Status},
    state::{
        claim_id, expiry, normalized_paths, reject_outside_claim, reject_overlap, require_active,
        summaries,
    },
    validate_commit_oid, validate_field, validate_repo_name, validate_ttl,
};

pub struct CoordStore {
    root: PathBuf,
    log_path: PathBuf,
}

impl CoordStore {
    pub fn new(root: PathBuf) -> Self {
        let log_path = root.join(".bullet-family/coord/events.jsonl");
        Self { root, log_path }
    }

    pub fn claim(&self, input: &ClaimInput, now: u64) -> Result<ClaimSummary, CoordError> {
        validate_field("agent", &input.agent)?;
        validate_field("lane", &input.lane)?;
        validate_repo_name(&input.repo)?;
        validate_ttl(input.ttl_seconds)?;
        let paths = normalized_paths(&input.paths)?;
        let expires = expiry(now, input.ttl_seconds)?;
        self.mutate(|records| {
            let claims = summaries(records, now)?;
            reject_overlap(&claims, &input.repo, &paths)?;
            let claim_id = claim_id(input, &paths, now);
            let record = Record::Claim {
                schema_version: SCHEMA_VERSION,
                at_unix_ms: now,
                claim_id: claim_id.clone(),
                agent: input.agent.clone(),
                lane: input.lane.clone(),
                repo: input.repo.clone(),
                paths: paths.clone(),
                expires_unix_ms: expires,
            };
            let summary = ClaimSummary {
                claim_id,
                agent: input.agent.clone(),
                lane: input.lane.clone(),
                repo: input.repo.clone(),
                paths,
                claimed_at_unix_ms: now,
                last_event_unix_ms: now,
                expires_unix_ms: expires,
                state: ClaimState::Active,
                proof_command: None,
                changed_paths: Vec::new(),
                commit_oid: None,
                commit_orchestrator: None,
                commit_recorded_at_unix_ms: None,
            };
            Ok((record, summary))
        })
    }

    pub fn heartbeat(&self, input: &HeartbeatInput, now: u64) -> Result<ClaimSummary, CoordError> {
        validate_field("agent", &input.agent)?;
        validate_field("claim_id", &input.claim_id)?;
        validate_ttl(input.ttl_seconds)?;
        if let Some(note) = &input.note {
            validate_field("note", note)?;
        }
        let expires = expiry(now, input.ttl_seconds)?;
        self.mutate(|records| {
            let mut claim = require_active(records, &input.claim_id, &input.agent, now)?;
            claim.expires_unix_ms = expires;
            claim.last_event_unix_ms = now;
            let record = Record::Heartbeat {
                schema_version: SCHEMA_VERSION,
                at_unix_ms: now,
                claim_id: input.claim_id.clone(),
                agent: input.agent.clone(),
                expires_unix_ms: expires,
                note: input.note.clone(),
            };
            Ok((record, claim))
        })
    }

    pub fn handoff(&self, input: &HandoffInput, now: u64) -> Result<ClaimSummary, CoordError> {
        validate_field("agent", &input.agent)?;
        validate_field("claim_id", &input.claim_id)?;
        validate_field("proof_command", &input.proof_command)?;
        if input.proof_exit_code != 0 {
            return Err(CoordError::new(
                "PROOF_FAILED",
                "handoff requires a proof command with exit code 0",
            ));
        }
        if input.commit_oid.is_some() {
            return Err(CoordError::new(
                "COMMIT_REQUIRES_RECEIPT",
                "only an orchestrator commit receipt may attach a commit OID",
            ));
        }
        let changed_paths = normalized_paths(&input.changed_paths)?;
        self.mutate(|records| {
            let mut claim = require_active(records, &input.claim_id, &input.agent, now)?;
            reject_outside_claim(&claim.paths, &changed_paths)?;
            claim.last_event_unix_ms = now;
            claim.state = ClaimState::HandedOff;
            claim.proof_command = Some(input.proof_command.clone());
            claim.changed_paths.clone_from(&changed_paths);
            claim.commit_oid = None;
            let record = Record::Handoff {
                schema_version: SCHEMA_VERSION,
                at_unix_ms: now,
                claim_id: input.claim_id.clone(),
                agent: input.agent.clone(),
                proof_command: input.proof_command.clone(),
                proof_exit_code: input.proof_exit_code,
                changed_paths,
                commit_oid: input.commit_oid.clone(),
            };
            Ok((record, claim))
        })
    }

    pub fn status(&self, now: u64) -> Result<Status, CoordError> {
        let file = self.open_log()?;
        FileExt::lock_shared(&file).map_err(CoordError::io)?;
        let records = read_records(&file)?;
        let claims = summaries(&records, now)?.into_values().collect();
        Ok(Status {
            schema_version: SCHEMA_VERSION,
            source: self.log_path.display().to_string(),
            as_of_unix_ms: now,
            claims,
        })
    }

    pub fn receipt(
        &self,
        input: &CommitReceiptInput,
        now: u64,
    ) -> Result<ClaimSummary, CoordError> {
        validate_field("claim_id", &input.claim_id)?;
        validate_field("orchestrator", &input.orchestrator)?;
        validate_commit_oid(&input.commit_oid)?;
        let committed_paths = normalized_paths(&input.committed_paths)?;
        self.mutate(|records| {
            let mut claim = summaries(records, now)?
                .remove(&input.claim_id)
                .ok_or_else(|| {
                    CoordError::new("CLAIM_NOT_FOUND", format!("no claim {}", input.claim_id))
                })?;
            if claim.state != ClaimState::HandedOff || claim.commit_oid.is_some() {
                return Err(CoordError::new(
                    "CLAIM_NOT_RECEIPTABLE",
                    "receipt requires a handed-off claim without an existing commit",
                ));
            }
            if claim.changed_paths != committed_paths {
                return Err(CoordError::new(
                    "COMMITTED_PATH_MISMATCH",
                    "receipt paths must exactly equal the handed-off changed paths",
                ));
            }
            verify_commit_paths(&self.root, &claim.repo, &input.commit_oid, &committed_paths)?;
            claim.last_event_unix_ms = now;
            claim.commit_oid = Some(input.commit_oid.clone());
            claim.commit_orchestrator = Some(input.orchestrator.clone());
            claim.commit_recorded_at_unix_ms = Some(now);
            let record = Record::CommitReceipt {
                schema_version: SCHEMA_VERSION,
                at_unix_ms: now,
                claim_id: input.claim_id.clone(),
                orchestrator: input.orchestrator.clone(),
                commit_oid: input.commit_oid.clone(),
                committed_paths,
            };
            Ok((record, claim))
        })
    }

    pub fn correct_receipt(
        &self,
        input: &ReceiptCorrectionInput,
        now: u64,
    ) -> Result<ClaimSummary, CoordError> {
        validate_field("claim_id", &input.claim_id)?;
        validate_field("orchestrator", &input.orchestrator)?;
        validate_field("reason", &input.reason)?;
        validate_commit_oid(&input.previous_commit_oid)?;
        validate_commit_oid(&input.commit_oid)?;
        let committed_paths = normalized_paths(&input.committed_paths)?;
        self.mutate(|records| {
            let mut claim = summaries(records, now)?
                .remove(&input.claim_id)
                .ok_or_else(|| {
                    CoordError::new("CLAIM_NOT_FOUND", format!("no claim {}", input.claim_id))
                })?;
            if claim.commit_oid.as_deref() != Some(input.previous_commit_oid.as_str()) {
                return Err(CoordError::new(
                    "RECEIPT_CORRECTION_MISMATCH",
                    "correction must bind the currently recorded commit OID",
                ));
            }
            if claim.changed_paths != committed_paths {
                return Err(CoordError::new(
                    "COMMITTED_PATH_MISMATCH",
                    "correction paths must exactly equal the handed-off changed paths",
                ));
            }
            verify_commit_paths(&self.root, &claim.repo, &input.commit_oid, &committed_paths)?;
            claim.last_event_unix_ms = now;
            claim.commit_oid = Some(input.commit_oid.clone());
            claim.commit_orchestrator = Some(input.orchestrator.clone());
            claim.commit_recorded_at_unix_ms = Some(now);
            let record = Record::CommitReceiptCorrection {
                schema_version: SCHEMA_VERSION,
                at_unix_ms: now,
                claim_id: input.claim_id.clone(),
                orchestrator: input.orchestrator.clone(),
                previous_commit_oid: input.previous_commit_oid.clone(),
                commit_oid: input.commit_oid.clone(),
                committed_paths,
                reason: input.reason.clone(),
            };
            Ok((record, claim))
        })
    }

    pub fn receipt_group(
        &self,
        input: &CommitReceiptGroupInput,
        now: u64,
    ) -> Result<Vec<ClaimSummary>, CoordError> {
        validate_field("orchestrator", &input.orchestrator)?;
        validate_commit_oid(&input.commit_oid)?;
        let mut claim_ids = input.claim_ids.clone();
        for claim_id in &claim_ids {
            validate_field("claim_id", claim_id)?;
        }
        claim_ids.sort();
        claim_ids.dedup();
        if claim_ids.len() < 2 {
            return Err(CoordError::new(
                "RECEIPT_GROUP_REQUIRED",
                "a grouped receipt requires at least two distinct claims",
            ));
        }
        self.mutate(|records| {
            let claims = summaries(records, now)?;
            let mut selected = Vec::with_capacity(claim_ids.len());
            let mut repo = None;
            let mut union = Vec::new();
            for claim_id in &claim_ids {
                let claim = claims.get(claim_id).ok_or_else(|| {
                    CoordError::new("CLAIM_NOT_FOUND", format!("no claim {claim_id}"))
                })?;
                if claim.state != ClaimState::HandedOff || claim.commit_oid.is_some() {
                    return Err(CoordError::new(
                        "CLAIM_NOT_RECEIPTABLE",
                        format!("claim {claim_id} is not an unreceipted handoff"),
                    ));
                }
                if repo.as_ref().is_some_and(|value| value != &claim.repo) {
                    return Err(CoordError::new(
                        "RECEIPT_REPO_MISMATCH",
                        "all grouped claims must belong to one repository",
                    ));
                }
                repo = Some(claim.repo.clone());
                union.extend(claim.changed_paths.iter().cloned());
                selected.push(claim.clone());
            }
            union.sort();
            union.dedup();
            let repo = repo.ok_or_else(|| {
                CoordError::new("RECEIPT_GROUP_REQUIRED", "group has no repository")
            })?;
            verify_commit_paths(&self.root, &repo, &input.commit_oid, &union)?;
            let receipts = selected
                .iter()
                .map(|claim| GroupReceipt {
                    claim_id: claim.claim_id.clone(),
                    committed_paths: claim.changed_paths.clone(),
                })
                .collect();
            for claim in &mut selected {
                claim.last_event_unix_ms = now;
                claim.commit_oid = Some(input.commit_oid.clone());
                claim.commit_orchestrator = Some(input.orchestrator.clone());
                claim.commit_recorded_at_unix_ms = Some(now);
            }
            let record = Record::CommitReceiptGroup {
                schema_version: SCHEMA_VERSION,
                at_unix_ms: now,
                orchestrator: input.orchestrator.clone(),
                commit_oid: input.commit_oid.clone(),
                receipts,
            };
            Ok((record, selected))
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn mutate<T>(
        &self,
        decide: impl FnOnce(&[Record]) -> Result<(Record, T), CoordError>,
    ) -> Result<T, CoordError> {
        let mut file = self.open_log()?;
        FileExt::lock_exclusive(&file).map_err(CoordError::io)?;
        let records = read_records(&file)?;
        let (record, output) = decide(&records)?;
        append_record(&mut file, &record)?;
        Ok(output)
    }

    fn open_log(&self) -> Result<File, CoordError> {
        let parent = self
            .log_path
            .parent()
            .ok_or_else(|| CoordError::new("INVALID_ROOT", "coord log has no parent"))?;
        fs::create_dir_all(parent).map_err(CoordError::io)?;
        let mut options = OpenOptions::new();
        options.create(true).read(true).append(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        options.open(&self.log_path).map_err(CoordError::io)
    }
}

fn read_records(file: &File) -> Result<Vec<Record>, CoordError> {
    let mut reader = file.try_clone().map_err(CoordError::io)?;
    reader.seek(SeekFrom::Start(0)).map_err(CoordError::io)?;
    let mut text = String::new();
    reader.read_to_string(&mut text).map_err(CoordError::io)?;
    text.lines()
        .enumerate()
        .map(|(index, line)| {
            let record: Record = serde_json::from_str(line).map_err(|error| {
                CoordError::new(
                    "CORRUPT_COORD_LOG",
                    format!("line {} is invalid JSON: {error}", index + 1),
                )
            })?;
            if record.schema_version() != SCHEMA_VERSION {
                return Err(CoordError::new(
                    "UNSUPPORTED_SCHEMA",
                    format!("line {} uses an unsupported schema", index + 1),
                ));
            }
            Ok(record)
        })
        .collect()
}

fn append_record(file: &mut File, record: &Record) -> Result<(), CoordError> {
    let mut encoded = serde_json::to_vec(record).map_err(CoordError::json)?;
    encoded.push(b'\n');
    let written = file.write(&encoded).map_err(CoordError::io)?;
    if written != encoded.len() {
        return Err(CoordError::new(
            "PARTIAL_COORD_WRITE",
            format!("wrote {written} of {} bytes", encoded.len()),
        ));
    }
    file.sync_data().map_err(CoordError::io)
}
