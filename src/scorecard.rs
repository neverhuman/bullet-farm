//! Generated D2 scorecard. Rows without admitted evidence stay at the
//! implemented floor. This module does not make a release gate green.

use crate::coord::CoordError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

const EXPECTED_BLEND: (f64, f64, f64) = (0.2, 0.6, 0.2);
const EXPECTED_BASELINES: (f64, f64, f64, f64) = (94.5, 94.5, 100.0, 3.0);
const EXPECTED_WEIGHT_SUM: u16 = 100;
const EXPECTED_DIMENSIONS: [(u8, &str, u8, u8, u8); 12] = [
    (1, "Concurrency and authority kernel", 14, 95, 64),
    (2, "Isolation and repository safety", 11, 96, 78),
    (3, "Evidence and verification integrity", 12, 94, 28),
    (4, "Integration and delivery authority", 11, 92, 26),
    (5, "Identity, quota and cost governance", 8, 90, 8),
    (6, "Multi-agent collaboration and roles", 8, 88, 6),
    (7, "Evolutionary optimization", 5, 86, 2),
    (8, "Operator truth and UX", 7, 90, 48),
    (9, "Installability and release engineering", 8, 94, 16),
    (10, "Security posture", 7, 92, 58),
    (11, "Test and assurance depth", 6, 90, 57),
    (12, "Documentation honesty", 3, 94, 86),
];
const EXPECTED_ROWS: [(&str, u8, EvidenceKind, &str); 15] = [
    (
        "d1.nonce-ledger",
        1,
        EvidenceKind::CiTest,
        "Durable nonce issue/consume separated",
    ),
    (
        "d1.signed-transport",
        1,
        EvidenceKind::Receipt,
        "Signed lease transport mounted internally",
    ),
    (
        "d2.egress-ci",
        2,
        EvidenceKind::CiTest,
        "Three egress proofs run every push",
    ),
    (
        "d3.proof-root-eight",
        3,
        EvidenceKind::CiTest,
        "ProofRoot over eight inputs with tamper tests",
    ),
    (
        "d4.attestor",
        4,
        EvidenceKind::Receipt,
        "Attestor binary posts exact-SHA checks",
    ),
    (
        "d4.jeryu-live",
        4,
        EvidenceKind::Gate,
        "release.forge.jeryu admitted",
    ),
    (
        "d5.budgets",
        5,
        EvidenceKind::CiTest,
        "Atomic dual-tree reservation/settlement",
    ),
    (
        "d6.two-providers",
        6,
        EvidenceKind::Receipt,
        "Two providers dispatch through the router",
    ),
    (
        "d7.evolution-off",
        7,
        EvidenceKind::CiTest,
        "evolutionary_authority remains false until OD-H",
    ),
    (
        "d8.fifteen-surfaces",
        8,
        EvidenceKind::CiTest,
        "Fifteen portal surfaces render durable subjects",
    ),
    (
        "d9.schema-3",
        9,
        EvidenceKind::Gate,
        "release.installable-lock admitted",
    ),
    (
        "d10.jankurai-90",
        10,
        EvidenceKind::Gate,
        "release.jankurai-90 admitted",
    ),
    (
        "d11.invariants-51",
        11,
        EvidenceKind::CiTest,
        "51/51 invariants enforced",
    ),
    (
        "d12.signed-jeryu-tags",
        12,
        EvidenceKind::SourceReceipt,
        "Jeryu tags are annotated and signed",
    ),
    (
        "g2.transaction-proof",
        1,
        EvidenceKind::Gate,
        "release.transaction-demo admitted",
    ),
];

/// Frozen rubric plus one evidence row per exit criterion.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScorecardSpec {
    /// Document identity.
    pub schema_version: String,
    /// Named rubric (`d2-v1`).
    pub rubric: String,
    /// This schema is an explicitly frozen baseline, not score authority.
    pub status: ScorecardStatus,
    /// False until every normative criterion has a typed point allocation.
    pub criterion_inventory_complete: bool,
    /// 20/60/20 blend.
    pub blend: Blend,
    /// Twelve product dimensions.
    pub dimensions: Vec<DimensionSpec>,
    /// Architecture design score.
    pub architecture_design: f64,
    /// Architecture implemented floor until evidence lands.
    pub architecture_implemented_floor: f64,
    /// Stranger-usable design score.
    pub stranger_design: f64,
    /// Stranger-usable implemented floor.
    pub stranger_implemented_floor: f64,
    /// Exit-criterion rows.
    pub rows: Vec<CriterionRow>,
}

/// Blend weights. Must sum to 1.0.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ScorecardStatus {
    /// Manual baseline held constant until semantic evidence admission exists.
    FrozenBaseline,
}

/// Blend weights. Must sum to 1.0.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Blend {
    /// Architecture share.
    pub architecture: f64,
    /// Implemented-product share.
    pub implemented: f64,
    /// Stranger-usable share.
    pub stranger: f64,
}

/// One D2 dimension.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionSpec {
    /// Dimension number 1..=12.
    pub id: u8,
    /// Display name.
    pub name: String,
    /// D2 weight.
    pub weight: u8,
    /// Design-as-specified score.
    pub design: u8,
    /// Implemented floor when no row has evidence.
    pub implemented_floor: u8,
}

/// One exit criterion.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceKind {
    /// Signed current-family CI observation.
    CiTest,
    /// Signed component or transaction receipt.
    Receipt,
    /// Semantically verified release-profile gate.
    Gate,
    /// Signed source/tag/immutability receipt.
    SourceReceipt,
    /// Independently signed external review or stranger trial.
    ExternalReview,
}

/// Closed evidence reference. This module does not yet semantically verify
/// any variant, so presence never admits a row.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "source", rename_all = "kebab-case", deny_unknown_fields)]
pub enum EvidenceReference {
    /// Signed proof-lane observation.
    CiObservation { subject_id: String },
    /// Verified release gate under one explicit profile.
    ReleaseGate {
        gate_id: String,
        profile_id: String,
        receipt_id: String,
    },
    /// Signed typed receipt.
    SignedReceipt { receipt_id: String },
    /// Signed source/tag/immutability receipt.
    SourceReceipt { receipt_id: String },
    /// Signed independent external report.
    ExternalReview { receipt_id: String },
}

/// One exit criterion.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CriterionRow {
    /// Stable row id.
    pub id: String,
    /// Owning dimension, or 0 for blend-only.
    pub dimension: u8,
    /// Evidence kind.
    pub kind: EvidenceKind,
    /// Claim text.
    pub claim: String,
    /// Typed reference for a future semantic verifier. `null` admits nothing.
    pub evidence: Option<EvidenceReference>,
}

/// Scored instrument. Never a release receipt.
#[derive(Clone, Debug, Serialize)]
pub struct ScorecardReport {
    /// Rubric id.
    pub rubric: String,
    /// Always false for `scorecard-v1`.
    pub authoritative: bool,
    /// Exact classification of the published number.
    pub status: ScorecardStatus,
    /// Weighted design score.
    pub design: f64,
    /// Weighted implemented score (floors + admitted rows).
    pub implemented: f64,
    /// Stranger-usable score.
    pub stranger: f64,
    /// Architecture score.
    pub architecture: f64,
    /// 20/60/20 blend, rounded to one decimal.
    pub blended: f64,
    /// Per-dimension implemented scores.
    pub dimensions: Vec<DimensionScore>,
    /// Rows with whether evidence was admitted.
    pub rows: Vec<RowScore>,
}

/// One dimension after scoring.
#[derive(Clone, Debug, Serialize)]
pub struct DimensionScore {
    /// Dimension number.
    pub id: u8,
    /// Name.
    pub name: String,
    /// Design score.
    pub design: u8,
    /// Implemented score used in the blend.
    pub implemented: u8,
}

/// One row after scoring.
#[derive(Clone, Debug, Serialize)]
pub struct RowScore {
    /// Row id.
    pub id: String,
    /// Whether an evidence subject was present and readable.
    pub admitted: bool,
    /// Stable reason the row contributes no evidence.
    pub refusal_reason: String,
    /// Claim.
    pub claim: String,
}

/// Load and score the rubric. Missing evidence keeps the floor.
///
/// # Errors
///
/// Rubric missing, invalid JSON, or blend weights that do not sum to 1.
pub fn evaluate(hub: &Path) -> Result<ScorecardReport, CoordError> {
    let path = hub.join("policy/scorecard-v1.json");
    let bytes = std::fs::read(&path).map_err(CoordError::io)?;
    let spec: ScorecardSpec = serde_json::from_slice(&bytes).map_err(CoordError::json)?;
    if spec.schema_version != "scorecard-v1" {
        return Err(CoordError::new(
            "SCORECARD_SCHEMA",
            format!("unsupported scorecard schema {}", spec.schema_version),
        ));
    }
    validate_spec(&spec)?;
    let blend_sum = spec.blend.architecture + spec.blend.implemented + spec.blend.stranger;
    if (blend_sum - 1.0).abs() > 0.001 {
        return Err(CoordError::new(
            "SCORECARD_BLEND",
            format!("blend weights sum to {blend_sum}, not 1.0"),
        ));
    }
    let rows: Vec<RowScore> = spec
        .rows
        .iter()
        .map(|row| RowScore {
            id: row.id.clone(),
            admitted: false,
            refusal_reason: if row.evidence.is_some() {
                "SEMANTIC_VERIFIER_UNAVAILABLE".into()
            } else {
                "NO_EVIDENCE_REFERENCE".into()
            },
            claim: row.claim.clone(),
        })
        .collect();
    let dimensions: Vec<DimensionScore> = spec
        .dimensions
        .iter()
        .map(|dim| DimensionScore {
            id: dim.id,
            name: dim.name.clone(),
            design: dim.design,
            // Implemented scores stay at the floor until a receipt, CI test,
            // or gate subject is admitted by `check release`. A readable file
            // is not enough to raise a dimension.
            implemented: dim.implemented_floor,
        })
        .collect();
    let weight_sum: f64 = spec.dimensions.iter().map(|d| f64::from(d.weight)).sum();
    let design = spec
        .dimensions
        .iter()
        .map(|d| f64::from(d.design) * f64::from(d.weight))
        .sum::<f64>()
        / weight_sum;
    let implemented = dimensions
        .iter()
        .zip(&spec.dimensions)
        .map(|(scored, dim)| f64::from(scored.implemented) * f64::from(dim.weight))
        .sum::<f64>()
        / weight_sum;
    let architecture = spec.architecture_implemented_floor;
    let stranger = spec.stranger_implemented_floor;
    let blended = spec.blend.architecture * architecture
        + spec.blend.implemented * implemented
        + spec.blend.stranger * stranger;
    Ok(ScorecardReport {
        rubric: spec.rubric,
        authoritative: false,
        status: spec.status,
        design: round1(design),
        implemented: round1(implemented),
        stranger,
        architecture,
        blended: round1(blended),
        dimensions,
        rows,
    })
}

/// Render the portable markdown page.
#[must_use]
pub fn render_markdown(report: &ScorecardReport) -> String {
    let mut out = String::from("# Scorecard (generated)\n\n");
    out.push_str(
        "Status: **frozen baseline estimate; instrument only; not release authority.**\n\n",
    );
    out.push_str(&format!(
        "Rubric `{}`. Blended **{}** (architecture {}, implemented {}, stranger {}).\n\n",
        report.rubric, report.blended, report.architecture, report.implemented, report.stranger
    ));
    out.push_str("| # | Dimension | Design | Implemented |\n| --- | --- | ---: | ---: |\n");
    for dim in &report.dimensions {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            dim.id, dim.name, dim.design, dim.implemented
        ));
    }
    out.push_str("\n| Row | Admitted | Refusal | Claim |\n| --- | --- | --- | --- |\n");
    for row in &report.rows {
        out.push_str(&format!(
            "| `{}` | {} | `{}` | {} |\n",
            row.id,
            if row.admitted { "yes" } else { "no" },
            row.refusal_reason,
            row.claim
        ));
    }
    out.push_str("\nRows are diagnostic only in `scorecard-v1`: no semantic evidence verifier is connected, so no row can move the frozen baseline.\n");
    out
}

fn validate_spec(spec: &ScorecardSpec) -> Result<(), CoordError> {
    if spec.rubric != "d2-v1"
        || spec.status != ScorecardStatus::FrozenBaseline
        || spec.criterion_inventory_complete
    {
        return Err(scorecard_error(
            "scorecard-v1 must remain an explicitly incomplete frozen baseline",
        ));
    }
    if (
        spec.blend.architecture.to_bits(),
        spec.blend.implemented.to_bits(),
        spec.blend.stranger.to_bits(),
    ) != (
        EXPECTED_BLEND.0.to_bits(),
        EXPECTED_BLEND.1.to_bits(),
        EXPECTED_BLEND.2.to_bits(),
    ) || (
        spec.architecture_design.to_bits(),
        spec.architecture_implemented_floor.to_bits(),
        spec.stranger_design.to_bits(),
        spec.stranger_implemented_floor.to_bits(),
    ) != (
        EXPECTED_BASELINES.0.to_bits(),
        EXPECTED_BASELINES.1.to_bits(),
        EXPECTED_BASELINES.2.to_bits(),
        EXPECTED_BASELINES.3.to_bits(),
    ) {
        return Err(scorecard_error(
            "scorecard-v1 blend and baseline values differ from the frozen instrument",
        ));
    }
    for (name, value) in [
        ("architecture", spec.blend.architecture),
        ("implemented", spec.blend.implemented),
        ("stranger", spec.blend.stranger),
    ] {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(scorecard_error(format!(
                "blend component {name} is outside 0..=1"
            )));
        }
    }
    for (name, value) in [
        ("architecture_design", spec.architecture_design),
        (
            "architecture_implemented_floor",
            spec.architecture_implemented_floor,
        ),
        ("stranger_design", spec.stranger_design),
        (
            "stranger_implemented_floor",
            spec.stranger_implemented_floor,
        ),
    ] {
        if !value.is_finite() || !(0.0..=100.0).contains(&value) {
            return Err(scorecard_error(format!(
                "baseline score {name} is outside 0..=100"
            )));
        }
    }
    if spec.dimensions.len() != EXPECTED_DIMENSIONS.len() {
        return Err(scorecard_error(
            "scorecard-v1 requires exactly 12 dimensions",
        ));
    }
    let mut dimension_ids = BTreeSet::new();
    let mut weight_sum = 0_u16;
    for (dimension, expected) in spec.dimensions.iter().zip(EXPECTED_DIMENSIONS) {
        if !(1..=12).contains(&dimension.id)
            || !dimension_ids.insert(dimension.id)
            || dimension.name.trim().is_empty()
            || dimension.design > 100
            || dimension.implemented_floor > 100
            || (
                dimension.id,
                dimension.name.as_str(),
                dimension.weight,
                dimension.design,
                dimension.implemented_floor,
            ) != expected
        {
            return Err(scorecard_error(format!(
                "dimension {} has an invalid or duplicate identity, name, or score",
                dimension.id
            )));
        }
        weight_sum = weight_sum
            .checked_add(u16::from(dimension.weight))
            .ok_or_else(|| scorecard_error("dimension weight sum overflowed"))?;
    }
    if weight_sum != EXPECTED_WEIGHT_SUM {
        return Err(scorecard_error(format!(
            "dimension weights sum to {weight_sum}, not {EXPECTED_WEIGHT_SUM}"
        )));
    }

    let mut row_ids = BTreeSet::new();
    if spec.rows.len() != EXPECTED_ROWS.len() {
        return Err(scorecard_error(
            "scorecard-v1 row inventory differs from the frozen diagnostic inventory",
        ));
    }
    for (row, expected) in spec.rows.iter().zip(EXPECTED_ROWS) {
        if !dimension_ids.contains(&row.dimension)
            || row.claim.trim().is_empty()
            || !row_ids.insert(row.id.as_str())
            || (row.id.as_str(), row.dimension, row.kind, row.claim.as_str()) != expected
        {
            return Err(scorecard_error(format!(
                "row {} has an invalid dimension, claim, or duplicate identity",
                row.id
            )));
        }
        if let Some(reference) = &row.evidence {
            validate_reference(row.kind, reference)?;
        }
    }
    Ok(())
}

fn validate_reference(kind: EvidenceKind, reference: &EvidenceReference) -> Result<(), CoordError> {
    let compatible = matches!(
        (kind, reference),
        (
            EvidenceKind::CiTest,
            EvidenceReference::CiObservation { .. }
        ) | (EvidenceKind::Gate, EvidenceReference::ReleaseGate { .. })
            | (
                EvidenceKind::Receipt,
                EvidenceReference::SignedReceipt { .. }
            )
            | (
                EvidenceKind::SourceReceipt,
                EvidenceReference::SourceReceipt { .. }
            )
            | (
                EvidenceKind::ExternalReview,
                EvidenceReference::ExternalReview { .. }
            )
    );
    if !compatible {
        return Err(scorecard_error(
            "scorecard evidence reference does not match its row kind",
        ));
    }
    let fields: Vec<&str> = match reference {
        EvidenceReference::CiObservation { subject_id } => vec![subject_id],
        EvidenceReference::ReleaseGate {
            gate_id,
            profile_id,
            receipt_id,
        } => vec![gate_id, profile_id, receipt_id],
        EvidenceReference::SignedReceipt { receipt_id }
        | EvidenceReference::SourceReceipt { receipt_id }
        | EvidenceReference::ExternalReview { receipt_id } => vec![receipt_id],
    };
    if fields.iter().any(|value| !safe_subject(value)) {
        return Err(scorecard_error(
            "scorecard evidence identifiers must be bounded non-path ASCII subjects",
        ));
    }
    Ok(())
}

fn safe_subject(value: &str) -> bool {
    (3..=160).contains(&value.len())
        && !value.starts_with('.')
        && !value.contains("..")
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'-' | b'_' | b'.' | b':')
        })
}

fn scorecard_error(message: impl Into<String>) -> CoordError {
    CoordError::new("SCORECARD_SCHEMA", message.into())
}

fn round1(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}
