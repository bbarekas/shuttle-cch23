use axum::{routing::{get, post}, Router};
use axum::response::IntoResponse;
use axum::extract::{State};
use axum::Json;
use sqlx::{PgPool};
use serde::Deserialize;
use serde_json::json;


#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn get_routes(
    pool: PgPool
) -> Router {

    let state = AppState { pool };

    Router::new()
        .route("/13/sql", get(query_sql))
        .route("/13/reset", post(reset_sql))
        .route("/13/orders", post(post_order))
        .route("/13/orders/total", get(sum_order))
        .route("/13/orders/popular", get(popular_order))
        .with_state(state)
}

async fn query_sql(State(state): State<AppState>) -> impl IntoResponse  {
    let query = sqlx::query!("SELECT 20231213 number")
        .fetch_one(&state.pool)
        .await
        .unwrap()
        .number
        .unwrap();

    query.to_string()
}

async fn reset_sql(State(state): State<AppState>) -> impl IntoResponse  {
    let _ = sqlx::query!("DROP TABLE IF EXISTS orders")
        .execute(&state.pool)
        .await
        .unwrap();

    let _ = sqlx::query!(
    r"
  CREATE TABLE orders (
    id INT PRIMARY KEY,
    region_id INT,
    gift_name VARCHAR(50),
    quantity INT
  )"
  )
        .execute(&state.pool)
        .await.unwrap();

    "".to_string()
}

#[derive(Deserialize, Debug)]
pub struct Order {
    pub id: i32,
    pub region_id: i32,
    pub gift_name: String,
    pub quantity: i32,
}

pub async fn post_order(State(state): State<AppState>, Json(payload): Json<Vec<Order>>) -> impl IntoResponse
{
    for el in payload {
        let _ = sqlx::query!(
            "INSERT INTO orders (id, region_id, gift_name, quantity) VALUES ($1, $2, $3, $4)",
                el.id,
                el.region_id,
                el.gift_name,
                el.quantity
            )
            .execute(&state.pool)
            .await.unwrap();
    }
    "".to_string()
}

async fn sum_order(State(state): State<AppState>)  -> impl IntoResponse {
    let total = sqlx::query!("SELECT SUM(quantity) total from orders")
        .fetch_one(&state.pool)
        .await
        .unwrap()
        .total
        .unwrap();

    let res = json!({
        "total": total,
    });

    Json(res)
}

async fn popular_order(State(state): State<AppState>) -> impl IntoResponse {
    let popular = sqlx::query!("SELECT gift_name, SUM(quantity) total from orders GROUP BY gift_name ORDER BY SUM(quantity) DESC")
        .fetch_optional(&state.pool)
        .await
        .unwrap()
        .map(|item| item.gift_name);

    let res = json!({
      "popular": popular,
    });

    Json(res)
}
