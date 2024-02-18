use axum::{http::StatusCode, Json};
use axum_extra::{headers::Cookie, TypedHeader};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::Value;

pub async fn base64_cookies(
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<Json<Value>, StatusCode> {
    let data = cookie.get("recipe").ok_or(StatusCode::BAD_REQUEST)?;
    let decoded = STANDARD.decode(data).map_err(|e| {
        eprintln!("ERR: error while decoding recipe from base64 {e}");
        StatusCode::BAD_REQUEST
    })?;
    let recipe = serde_json::from_slice(&decoded).map_err(|e| {
        eprintln!("ERR: error while decoding recipe from base64 {e}");
        StatusCode::BAD_REQUEST
    })?;

    Ok(Json(recipe))
}
