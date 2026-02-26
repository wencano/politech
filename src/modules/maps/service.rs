use uuid::Uuid;

use crate::modules::topics::service as topic_svc;
use crate::modules::topics::HEXACO_KEYS;
use crate::modules::locations::service as loc_svc;

use super::models::RegionAlignmentDto;

/// Compute alignment 0–1 per region: dot product of topic and location traits (0–1), 1:1 HEXACO.
/// Raw score_01 = dot/n; then min-max normalized to [0.05, 0.95] so demo chart/map show clear variation.
pub async fn alignment_for_topic(
    db: &sqlx::PgPool,
    topic_id: Uuid,
) -> Result<Vec<RegionAlignmentDto>, sqlx::Error> {
    let topic = topic_svc::get_topic(db, topic_id).await?;
    let topic = match topic {
        Some(t) => t,
        None => return Ok(Vec::new()),
    };
    let locations = loc_svc::list_locations(db).await?;
    let empty: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    let topic_traits = topic.traits.as_object().unwrap_or(&empty);

    let scores: Vec<(RegionAlignmentDto, f64)> = locations
        .into_iter()
        .map(|loc| {
            let loc_traits = loc.traits.as_object().unwrap_or(&empty);
            let mut dot = 0.0f64;
            let mut n = 0usize;
            for k in HEXACO_KEYS {
                if let (Some(tv), Some(lv)) = (
                    topic_traits.get(k).and_then(|v| v.as_f64()),
                    loc_traits.get(k).and_then(|v| v.as_f64()),
                ) {
                    let t = tv.clamp(0.0, 1.0);
                    let l = lv.clamp(0.0, 1.0);
                    dot += t * l;
                    n += 1;
                }
            }
            let score_01 = if n > 0 { (dot / n as f64).clamp(0.0, 1.0) } else { 0.5 };
            let dto = RegionAlignmentDto {
                code: loc.code,
                geojson_key: loc.geojson_key,
                name: loc.name.clone(),
                alignment_01: score_01,
            };
            (dto, score_01)
        })
        .collect();

    let raw: Vec<f64> = scores.iter().map(|(_, s)| *s).collect();
    let min_s = raw.iter().copied().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0);
    let max_s = raw.iter().copied().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(1.0);
    let span = (max_s - min_s).max(1e-9);
    let result: Vec<RegionAlignmentDto> = scores
        .into_iter()
        .map(|(mut dto, s)| {
            let normalized = 0.05 + 0.9 * (s - min_s) / span;
            dto.alignment_01 = normalized.clamp(0.0, 1.0);
            dto
        })
        .collect();

    Ok(result)
}
