use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeFile;

mod challenges;
use challenges::{c00, c01, c04, c05, c06, c07, c08, c11, c12};
use tracing::info;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    info!("Starting...");

    let router = Router::new()
        .route("/", get(c00::hello_world))
        .route("/-1/error", get(c00::fake_error))
        // .route("/1/:first/:second", get(c01::nested_xor))
        .route("/1/*path", get(c01::calculate_sled_id))
        .route("/4/strength", post(c04::calculate_total_strength))
        .route("/4/contest", post(c04::cursed_contest))
        .route("/5", post(c05::slice_params))
        .route("/6", post(c06::count_elf))
        .route("/7/decode", get(c07::base64_cookies))
        .route("/7/bake", get(c07::bake_cookies))
        .route("/8/weight/:pokedex_number", get(c08::poke_weight))
        .route("/8/drop/:pokedex_number", get(c08::poke_momentum))
        .route_service(
            "/11/assets/decoration.png",
            ServeFile::new("assets/decoration.png"),
        )
        .route("/11/red_pixels", post(c11::red_pixel_count))
        .route("/12/save/:string", post(c12::store_string))
        .route("/12/load/:string", get(c12::load_string));

    Ok(router.into())
}
