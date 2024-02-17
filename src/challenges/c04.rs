use axum::{extract, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReindeerInfo {
    name: String,
    strength: u32,
}

pub async fn calculate_total_strength(
    extract::Json(payload): extract::Json<Vec<ReindeerInfo>>,
) -> impl IntoResponse {
    let total_strength = payload.into_iter().fold(0, |acc, deer| acc + deer.strength);
    total_strength.to_string()
}
