use axum::{
    routing::post,
    extract::Query,
    Router,
    Json,
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Params {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/5", post(slice_list))
}

async fn slice_list(Query(params): Query<Params>, Json(payload): Json<Vec<String>>)
    -> impl IntoResponse {

    let offset: usize = params.offset.unwrap_or(0);
    let limit: usize = params.limit.unwrap_or(payload.len());
    let split: usize = params.split.unwrap_or(usize::MAX);

    let payload = payload
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect::<Vec<_>>();

    if params.split.is_none() {
        Json(payload).into_response()
    }
    else {
        Json(payload.chunks(split).map(|s| s.into()).collect::<Vec<Vec<String>>>())
            .into_response()
    }
}
