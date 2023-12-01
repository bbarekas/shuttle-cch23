use axum::{
    routing::get,
    http::{StatusCode},
    response::{IntoResponse, Response},
    extract::Path,
    Router
};


async fn hello_world() -> Response {
    String::from("Hello, world!").into_response()
}

async fn handle_error() -> (StatusCode, String) {
    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong.")
    );
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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(handle_error))
        .route("/1/*nums", get(cube_the_bits));


    Ok(router.into())
}
