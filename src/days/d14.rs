use askama::Template;
use axum::{routing::{get, post}, Router};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};


pub fn get_routes() -> Router {

    Router::new()
        .route("/14", get(axum::http::StatusCode::OK))
        .route("/14/unsafe", post(unsafe_content))
        .route("/14/safe", post(safe_content))
}

#[derive(Deserialize, Serialize, Template)]
#[template(path = "page.html")]
struct Content {
    content: String,
    not_safe: Option<bool>,
}

async fn unsafe_content(Json(data): Json<Content>) -> impl IntoResponse {
    let data = Content {
        not_safe: Some(true),
        ..data
    };
    data.into_response()
}

async fn safe_content(Json(data): Json<Content>) -> impl IntoResponse {
    data.into_response()
}