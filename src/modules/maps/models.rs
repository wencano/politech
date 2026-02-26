use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RegionAlignmentDto {
    pub code: String,
    pub geojson_key: Option<String>,
    pub name: String,
    /// Normalized alignment 0–1 (for display as %). Demo: min-max scaled so regions span range.
    #[serde(rename = "alignment_01")]
    pub alignment_01: f64,
}
