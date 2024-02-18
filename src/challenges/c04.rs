use axum::{extract, response::IntoResponse, Json};
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

#[derive(Deserialize)]
pub struct ReindeerDetails {
    name: String,
    strength: u32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(alias = "cAnD13s_3ATeN-yesT3rdAy")]
    candies: u32,
}

pub async fn cursed_contest(Json(payload): Json<Vec<ReindeerDetails>>) -> impl IntoResponse {}
