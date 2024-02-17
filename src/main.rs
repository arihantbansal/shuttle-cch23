use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Router,
};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn nested_post(Path(first): Path<u32>) -> StatusCode {
    StatusCode::CREATED
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .route("/1/:first/:second", post(nested_post));

    Ok(router.into())
}
