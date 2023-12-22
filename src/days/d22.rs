use axum::{
    routing::{get, post},
    Router,
    response::IntoResponse,
    };
use http::StatusCode;

pub fn get_routes() -> Router {

    Router::new()
        .route("/22", get(StatusCode::OK))
        .route("/22/integers", post(find_single_one))
        //.route("/22/rocket", post(find_portal))

}

async fn find_single_one(body: String) -> impl IntoResponse {

    let mut input: Vec<u64> = body.trim().split_whitespace()
        .map(|x| x.parse().expect("parse error"))
        .collect();
    input.sort();

    let mut single = 0;
    for (pos, num) in input.iter().enumerate().step_by(2) {
        if pos == input.len()-1 {
            single = *num as usize;
            break;
        }
        if *num != input[pos+1] {
            single = *num as usize;
            break;
        }
    }

    "🎁".repeat(single).to_string()
}
