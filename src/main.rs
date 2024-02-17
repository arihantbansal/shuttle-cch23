use axum::{
    routing::{get, post},
    Router,
};

mod challenges;
use challenges::c00;
use challenges::c01;
use challenges::c04;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(c00::hello_world))
        .route("/-1/error", get(c00::fake_error))
        // .route("/1/:first/:second", get(c01::nested_xor))
        .route("/1/*path", get(c01::calculate_sled_id))
        .route("/4/strength", post(c04::calculate_total_strength));

    Ok(router.into())
}
