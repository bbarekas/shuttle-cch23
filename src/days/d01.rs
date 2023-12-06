use axum::{
    extract::Path,
    routing::get,
    Router
};

pub fn get_routes() -> Router {
    Router::new()
        .route("/1/*nums", get(cube_the_bits))
}

async fn cube_the_bits(Path(path): Path<String>) -> String {
    let res = path
        .split('/')
        .map(|part| part.parse::<i32>().unwrap_or(0))
        .reduce(|acc, e| acc ^ e)
        .expect("Iterator not empty")
        .pow(3);

    res.to_string()
}
