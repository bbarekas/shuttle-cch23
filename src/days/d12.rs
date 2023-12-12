use axum::{routing::{get, post}, Router, Json};
use axum::response::IntoResponse;
use std::sync::Arc;
use axum::extract::{Path, State};
use chrono::{DateTime, Local, Datelike, NaiveTime, Utc};
use ulid::Ulid;
use uuid::Uuid;
use serde_json::{json};

use shuttle_persist::PersistInstance;

struct AppState {
    persist: PersistInstance,
}

pub fn get_routes(
    persist: PersistInstance
) -> Router {

    let state = Arc::new(AppState { persist });

    Router::new()
        .route("/12/save/:key", post(save_key))
        .route("/12/load/:key",  get(load_key))
        .route("/12/ulids", post(convert_ulids))
        .route("/12/ulids/:weekday", post(calculate_ulids))

        .with_state(state)
}

async fn save_key(State(state): State<Arc<AppState>>, Path(key): Path<String>) -> impl IntoResponse  {
    // Save key with current time.
    state
        .persist
        .save::<NaiveTime>(
            &key,  Local::now().time()
        )
        .map_err(|e| (e.to_string()))
    // Return nothing.
}

async fn load_key(State(state): State<Arc<AppState>>, Path(key): Path<String>) -> impl IntoResponse  {
    // Get elapsed time from stored value with key.
    let elapsed = Local::now().time() - state
        .persist
        .load::<NaiveTime>(
            &key
        ).unwrap();

    elapsed.num_seconds().to_string()
}

async fn convert_ulids(Json(body): Json<Vec<String>>) -> impl IntoResponse  {

    let ulids = body.into_iter()
        .map(|el| Ulid::from_string(&el))
        .filter_map(Result::ok);

    let uuids = ulids
        .map(Uuid::from)
        .map(|el| el.to_string())
        .rev()
        .collect::<Vec<_>>();

    Json(uuids)
}

async fn calculate_ulids(
    Path(weekday): Path<u32>,
    Json(body): Json<Vec<String>>
) -> impl IntoResponse  {

    let ulids = body.into_iter()
        .map(|el| Ulid::from_string(&el))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let mut n_christmas = 0;
    let mut n_weekday = 0;
    let mut n_future = 0;
    let mut n_lsb = 0;

    for ulid in ulids {
        let time: DateTime<Utc> = ulid.datetime().into();
        //println!("TIME: {:?}", time);
        if time.month() == 12 && time.day() == 24 {
            n_christmas += 1;
        }
        if time.weekday().num_days_from_monday() == weekday {
            n_weekday += 1;
        }
        if time > Utc::now() {
            n_future += 1;
        }
        if ulid.random() & 1 == 1 {
            n_lsb += 1;
        }
    }

    let res = json!({
        "christmas eve": n_christmas,
        "weekday": n_weekday,
        "in the future": n_future,
        "LSB is 1": n_lsb
    });

    Json(res)
}
