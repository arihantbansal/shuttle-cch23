use axum::{extract::Path, http::StatusCode, routing::get, Router};
use tracing::debug;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn nested_xor(Path((first, second)): Path<(u32, u32)>) -> String {
    (u32::pow(first ^ second, 3)).to_string()
}

async fn calculate_sled_id(Path(path): Path<String>) -> String {
    debug!("Numbers: {:?}", path);
    let numbers = path
        .split('/')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let xored = numbers.into_iter().fold(0, |acc, num| acc ^ num);
    let powed = xored.pow(3);
    powed.to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        // .route("/1/:first/:second", get(nested_xor))
        .route("/1/*path", get(calculate_sled_id));

    Ok(router.into())
}
