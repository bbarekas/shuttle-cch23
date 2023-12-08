use std::{cmp, collections::HashMap};
use axum::{routing::get,
           response::IntoResponse,
           Router,
           Json};
use axum_extra::TypedHeader;
use base64::{engine::general_purpose, Engine as _};
use headers::Cookie;

use serde::{Deserialize, Serialize};

pub fn get_routes() -> Router {
    Router::new()
        .route("/7", axum::routing::get(axum::http::StatusCode::OK))
        .route("/7/decode", get(decode_cookie))
        .route("/7/bake", get(bake_cookie))
        .route("/7/bake_simple", get(bake_cookie_simple))

}


#[derive(Debug, Serialize, Deserialize)]
struct DecodeResponse {
    flour: usize,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: usize,
}

async fn decode_cookie(TypedHeader(cookie): TypedHeader<Cookie>) -> impl IntoResponse {

    let cookie = cookie.get("recipe").expect("Could not find recipe");
    let decoded_bytes = general_purpose::STANDARD.decode(cookie).unwrap();
    let recipe_str = String::from_utf8(decoded_bytes).unwrap();

    let recipe: DecodeResponse = serde_json::from_str(&recipe_str).unwrap();

    Json(recipe)
}

#[derive(Debug, Serialize, Deserialize)]
struct Ingredients {
    flour: usize,
    sugar: usize,
    butter: usize,
    #[serde(rename = "baking powder")]
    baking_powder: usize,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct BakeSimpleRequest {
    recipe: Ingredients,
    pantry: Ingredients,
}

#[derive(Debug, Serialize, Deserialize)]
struct BakeSimpleResponse {
    cookies: usize,
    pantry: Ingredients,
}

async fn bake_cookie_simple(TypedHeader(cookie): TypedHeader<Cookie>) -> impl IntoResponse {

    let cookie = cookie.get("recipe").expect("Could not find recipe");
    let decoded_bytes = general_purpose::STANDARD.decode(cookie).unwrap();
    let recipe_str = String::from_utf8(decoded_bytes).unwrap();

    let request: BakeSimpleRequest = serde_json::from_str(&recipe_str).unwrap();
    let recipe = request.recipe;
    let mut pantry = request.pantry;

    let cookies =
        cmp::min(
            cmp::min(
                cmp::min(
                    cmp::min(pantry.flour / recipe.flour, pantry.sugar / recipe.sugar),
                    pantry.butter / recipe.butter,
                ),
                pantry.baking_powder / recipe.baking_powder,
            ),
            pantry.chocolate_chips / recipe.chocolate_chips,
        );

    pantry.flour = pantry.flour - cookies * recipe.flour;
    pantry.sugar = pantry.sugar - cookies * recipe.sugar;
    pantry.butter = pantry.butter - cookies * recipe.butter;
    pantry.baking_powder = pantry.baking_powder - cookies * recipe.baking_powder;
    pantry.chocolate_chips = pantry.chocolate_chips - cookies * recipe.chocolate_chips;

    Json(BakeSimpleResponse {
        cookies,
        pantry,
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct BakeRequest {
    recipe: HashMap<String, usize>,
    pantry: HashMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BakeResponse {
    cookies: usize,
    pantry: HashMap<String, usize>,
}

async fn bake_cookie(TypedHeader(cookie): TypedHeader<Cookie>) -> impl IntoResponse {

    let cookie = cookie.get("recipe").expect("Could not find recipe");
    let decoded_bytes = general_purpose::STANDARD.decode(cookie).unwrap();
    let recipe_str = String::from_utf8(decoded_bytes).unwrap();

    let request: BakeRequest = serde_json::from_str(&recipe_str).unwrap();
    let recipe = request.recipe;
    let mut pantry = request.pantry;

    let cookies = recipe
        .iter()
        .fold(usize::MAX, |cookies, (ingredient, needed)| {
            let available = pantry.get(ingredient).unwrap_or(&0);
            cookies.min(available / needed)
        });

    for (key, pantry_value) in &mut pantry {
        *pantry_value -= cookies * recipe.get(key).unwrap_or(&0);
    }

    Json(BakeResponse {
        cookies,
        pantry,
    })
}
