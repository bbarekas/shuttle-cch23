use axum::{
    routing::post,
    Router,
    extract::Multipart,
};
use axum::response::IntoResponse;
use serde::{Serialize};
use tower_http::services::ServeDir;

use image::{
    io::Reader,
    GenericImageView,
};
use std::io::Cursor;

#[derive(Serialize)]
struct CountResponse {
    elf: i32,
    #[serde(rename(serialize = "elf on a shelf"))]
    elf_on_a_shelf: i32,
    #[serde(rename(serialize = "shelf with no elf on it"))]
    shelf_with_no_elf_on_it: i32,
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/11/red_pixels", post(red_pixels))
        .nest_service("/11/assets", ServeDir::new("assets"))

}

async fn red_pixels(mut multipart: Multipart) -> impl IntoResponse {
    let mut red_pixels = 0u32;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        let reader = Reader::new(Cursor::new(&data))
            .with_guessed_format()
            .expect("This will never fail using Cursor");
        let img = reader.decode().expect("Failed to read image");

        red_pixels = img.pixels()
            .filter(|x| {
                let [r, g, b, _] = x.2 .0;
                r as u16 > (g as u16 + b as u16)
            })
            .count() as u32;
    }

    red_pixels.to_string()
}
