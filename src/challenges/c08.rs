use axum::{extract::Path, http::StatusCode, Json};
use serde_json::Value;

pub async fn poke_weight(Path(pokedex_number): Path<u64>) -> Result<Json<f64>, StatusCode> {
    let poke_details = reqwest::get(format!(
        "https://pokeapi.co/api/v2/pokemon/{}",
        pokedex_number
    ))
    .await
    .map_err(|e| {
        eprintln!("ERR: error while awaiting request {e}");
        StatusCode::BAD_REQUEST
    })?
    .json::<Value>()
    .await
    .map_err(|e| {
        eprintln!("ERR: error while awaiting json response {e}");
        StatusCode::BAD_REQUEST
    })?;

    let poke_weight = poke_details["weight"]
        .as_number()
        .unwrap()
        .as_f64()
        .unwrap()
        / 10.0f64;

    Ok(Json(poke_weight))
}
