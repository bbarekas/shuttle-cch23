use axum::{
    routing::post,
    Router,
    Json
};
use serde::{Serialize};

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
        .route("/6", post(count_elfs))
}

async fn count_elfs(body: String) -> Json<CountResponse> {
    let count_elf = body.matches("elf").count();
    let count_eoas = body.matches("elf on a shelf").count();
    let count_shelf = body.matches("shelf").count();

    // Format count response.
    let res = CountResponse {
        elf: count_elf as i32,
        elf_on_a_shelf: count_eoas as i32,
        shelf_with_no_elf_on_it: (count_shelf - count_eoas) as i32,
    };

    Json(res)
}
