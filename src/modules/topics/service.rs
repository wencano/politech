use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use uuid::Uuid;

use super::dal;

/// HEXACO dimension keys (environment-adjacent / standard for topic–location alignment).
/// Both topics and locations use these; traits are normalized 0–1.
pub const HEXACO_KEYS: [&str; 6] = [
    "honesty_humility",
    "emotionality",
    "extraversion",
    "agreeableness",
    "conscientiousness",
    "openness",
];

fn title_seed(title: &str) -> u64 {
    title.bytes().fold(0u64, |s, b| s.wrapping_add(b as u64).wrapping_mul(31))
}

/// Derive topic traits from title: seeded RNG so same title → same traits; each topic gets
/// a random high/low/mid profile so alignments vary across regions (not uniform).
pub fn derive_traits_from_title(title: &str) -> Value {
    let seed = title_seed(title);
    let mut rng = StdRng::seed_from_u64(seed);

    let mut keys: Vec<&str> = HEXACO_KEYS.iter().copied().collect();
    keys.shuffle(&mut rng);

    let mut obj = BTreeMap::new();
    for (i, key) in keys.iter().enumerate() {
        let v = match i {
            0 | 1 => 0.75 + rng.r#gen::<f64>() * 0.23,
            2 | 3 => 0.02 + rng.r#gen::<f64>() * 0.23,
            _ => 0.35 + rng.r#gen::<f64>() * 0.30,
        };
        obj.insert((*key).to_string(), json!(v.clamp(0.0, 1.0)));
    }

    let title_lower = title.to_lowercase();
    if title_lower.contains("infrastructure") || title_lower.contains("road") {
        obj.insert("conscientiousness".into(), json!((0.7 + rng.r#gen::<f64>() * 0.25).min(1.0)));
        obj.insert("openness".into(), json!((0.65 + rng.r#gen::<f64>() * 0.3).min(1.0)));
    }
    if title_lower.contains("education") || title_lower.contains("school") {
        obj.insert("openness".into(), json!((0.72 + rng.r#gen::<f64>() * 0.26).min(1.0)));
        obj.insert("conscientiousness".into(), json!((0.6 + rng.r#gen::<f64>() * 0.35).min(1.0)));
    }
    if title_lower.contains("health") || title_lower.contains("hospital") {
        obj.insert("agreeableness".into(), json!((0.68 + rng.r#gen::<f64>() * 0.3).min(1.0)));
        obj.insert("emotionality".into(), json!((0.5 + rng.r#gen::<f64>() * 0.4).min(1.0)));
    }
    if title_lower.contains("agriculture") || title_lower.contains("farmer") {
        obj.insert("honesty_humility".into(), json!((0.7 + rng.r#gen::<f64>() * 0.28).min(1.0)));
        obj.insert("conscientiousness".into(), json!((0.55 + rng.r#gen::<f64>() * 0.4).min(1.0)));
    }
    if title_lower.contains("environment") || title_lower.contains("climate") {
        obj.insert("openness".into(), json!((0.7 + rng.r#gen::<f64>() * 0.28).min(1.0)));
        obj.insert("agreeableness".into(), json!((0.6 + rng.r#gen::<f64>() * 0.35).min(1.0)));
    }
    Value::Object(serde_json::Map::from_iter(obj))
}

pub async fn list_topics(db: &sqlx::PgPool) -> Result<Vec<super::models::TopicDto>, sqlx::Error> {
    dal::list_topics(db).await
}

pub async fn get_topic(db: &sqlx::PgPool, id: Uuid) -> Result<Option<super::models::TopicDto>, sqlx::Error> {
    dal::get_topic_by_id(db, id).await
}

pub async fn create_topic(
    db: &sqlx::PgPool,
    id: Uuid,
    title: &str,
) -> Result<super::models::TopicDto, sqlx::Error> {
    let traits = derive_traits_from_title(title);
    dal::insert_topic(db, id, title, &traits).await?;
    let topic = dal::get_topic_by_id(db, id).await?.expect("topic just inserted");
    Ok(topic)
}

pub async fn update_topic(
    db: &sqlx::PgPool,
    id: Uuid,
    title: &str,
) -> Result<Option<super::models::TopicDto>, sqlx::Error> {
    let traits = derive_traits_from_title(title);
    let updated = dal::update_topic(db, id, title, &traits).await?;
    if !updated {
        return Ok(None);
    }
    dal::get_topic_by_id(db, id).await
}

pub async fn delete_topic(db: &sqlx::PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    dal::delete_topic(db, id).await
}
