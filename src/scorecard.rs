//! Generated D2 scorecard. Rows without admitted evidence stay at the
//! implemented floor. This module does not make a release gate green.

use crate::coord::CoordError;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Frozen rubric plus one evidence row per exit criterion.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScorecardSpec {
    /// Document identity.
    pub schema_version: String,
    /// Named rubric (`d2-v1`).
    pub rubric: String,
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
#[derive(Clone, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CriterionRow {
    /// Stable row id.
    pub id: String,
    /// Owning dimension, or 0 for blend-only.
    pub dimension: u8,
    /// Evidence kind.
    pub kind: String,
    /// Claim text.
    pub claim: String,
    /// Admitted subject path, commit, or gate id. `null` admits nothing.
    pub evidence: Option<String>,
}

/// Scored instrument. Never a release receipt.
#[derive(Clone, Debug, Serialize)]
pub struct ScorecardReport {
    /// Rubric id.
    pub rubric: String,
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
            admitted: row
                .evidence
                .as_ref()
                .is_some_and(|subject| hub.join(subject).is_file()),
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
    out.push_str("Status: **instrument only; not release authority.**\n\n");
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
    out.push_str("\n| Row | Admitted | Claim |\n| --- | --- | --- |\n");
    for row in &report.rows {
        out.push_str(&format!(
            "| `{}` | {} | {} |\n",
            row.id,
            if row.admitted { "yes" } else { "no" },
            row.claim
        ));
    }
    out.push_str(
        "\nA row moves the implemented score only after an admitted evidence subject exists.\n",
    );
    out
}

fn round1(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}
