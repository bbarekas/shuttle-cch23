mod days;

use axum::{
    routing::{get},
    http::{StatusCode},
    response::{IntoResponse, Response},
    Router,
};
use shuttle_runtime::CustomError;
use shuttle_persist::PersistInstance;
use sqlx::PgPool;

async fn hello_world() -> Response {
    String::from("Hello, world!").into_response()
}

async fn handle_error() -> (StatusCode, String) {
    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong.")
    );
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_persist::Persist] persist: PersistInstance,
    #[shuttle_shared_db::Postgres(local_uri = "postgres://postgres:postgres@localhost:5432/postgres")] pool: PgPool
) -> shuttle_axum::ShuttleAxum {

    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(handle_error))
        .merge(days::d01::get_routes())
        .merge(days::d04::get_routes())
        .merge(days::d06::get_routes())
        .merge(days::d07::get_routes())
        .merge(days::d08::get_routes())
        .merge(days::d11::get_routes())
        .merge(days::d12::get_routes(persist))
        .merge(days::d13::get_routes(pool.clone()))
        .merge(days::d14::get_routes())
        .merge(days::d15::get_routes())
        .merge(days::d18::get_routes(pool))
        .merge(days::d19::get_routes());

    Ok(router.into())
}
