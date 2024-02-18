use std::cmp;

use axum::{extract::Query, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct OffsetParams {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

pub async fn slice_params(
    queries: Query<OffsetParams>,
    Json(payload): Json<Vec<String>>,
) -> impl IntoResponse {
    let offset = queries.offset.unwrap_or(0);
    let limit = queries.limit.unwrap_or(payload.len());

    let sliced = payload[offset..(cmp::min(offset + limit, payload.len()))].to_vec();
    match queries.split {
        Some(split) => Json(json!(sliced
            .chunks(split)
            .map(|s| s.into())
            .collect::<Vec<Vec<String>>>())),
        None => Json(json!(sliced)),
    }
}
