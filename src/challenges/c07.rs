use axum::{http::StatusCode, Json};
use axum_extra::{headers::Cookie, TypedHeader};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    recipe: Ingredients,
    pantry: Ingredients,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ingredients {
    flour: u64,
    sugar: u64,
    butter: u64,
    #[serde(rename = "baking powder")]
    baking_powder: u64,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CookiesBaked {
    cookies: u64,
    pantry: Ingredients,
}

pub async fn bake_cookies(
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<Json<CookiesBaked>, StatusCode> {
    let data = cookie.get("recipe").ok_or(StatusCode::BAD_REQUEST)?;
    let decoded = STANDARD.decode(data).map_err(|e| {
        eprintln!("ERR: error while decoding recipe from base64 {e}");
        StatusCode::BAD_REQUEST
    })?;

    let ingredients: Recipe = serde_json::from_slice(&decoded).map_err(|e| {
        eprintln!("ERR: error while decoding recipe from base64 {e}");
        StatusCode::BAD_REQUEST
    })?;

    let recipe = ingredients.recipe;
    let pantry = ingredients.pantry;

    let cookies_baked = how_many(&recipe, &pantry);
    let leftover_pantry = reduce_pantry(&recipe, &pantry, cookies_baked);

    Ok(Json(CookiesBaked {
        cookies: cookies_baked,
        pantry: leftover_pantry,
    }))
}

fn how_many(recipe: &Ingredients, pantry: &Ingredients) -> u64 {
    let mut count = u64::MAX;
    count = count.min(pantry.flour / recipe.flour);
    count = count.min(pantry.sugar / recipe.sugar);
    count = count.min(pantry.butter / recipe.butter);
    count = count.min(pantry.baking_powder / recipe.baking_powder);
    count = count.min(pantry.chocolate_chips / recipe.chocolate_chips);

    count
}

fn reduce_pantry(recipe: &Ingredients, pantry: &Ingredients, cookies_baked: u64) -> Ingredients {
    Ingredients {
        flour: pantry.flour - (recipe.flour * cookies_baked),
        sugar: pantry.sugar - (recipe.sugar * cookies_baked),
        butter: pantry.butter - (recipe.butter * cookies_baked),
        baking_powder: pantry.baking_powder - (recipe.baking_powder * cookies_baked),
        chocolate_chips: pantry.chocolate_chips - (recipe.chocolate_chips * cookies_baked),
    }
}
