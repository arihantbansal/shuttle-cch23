use axum::{extract::Multipart, http::StatusCode, Json};
use serde_json::Value;

pub async fn red_pixel_count(mut form_data: Multipart) -> Result<Json<Value>, StatusCode> {
    match form_data.next_field().await.unwrap() {
        Some(field) => {
            let name = field.name().unwrap().to_string();
            let img = field.bytes().await.unwrap();

            Ok(Json("hi".into()))
        }
        None => Err(StatusCode::BAD_REQUEST),
    }
}
