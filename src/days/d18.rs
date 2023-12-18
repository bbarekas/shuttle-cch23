use axum::{routing::{get, post}, Router};
use axum::response::IntoResponse;
use axum::extract::{Path, State};
use axum::Json;
use sqlx::{PgPool, FromRow};
use serde::{Serialize, Deserialize};
use crate::days::d13;

pub fn get_routes(
    pool: PgPool
) -> Router {

    let state = d13::AppState { pool };

    Router::new()
        .route("/18", get(axum::http::StatusCode::OK))
        .route("/18/reset", post(reset_sql))
        .route("/18/orders", post(post_orders))
        .route("/18/regions", post(post_regions))
        .route("/18/regions/total", get(sum_regions))
        .route("/18/regions/top_list/:number", get(top_gifts))
        .with_state(state)
}


async fn reset_sql(State(state): State<d13::AppState>) -> impl IntoResponse  {
    let _ = sqlx::query!("DROP TABLE IF EXISTS regions")
        .execute(&state.pool)
        .await
        .unwrap();

    let _ = sqlx::query!("DROP TABLE IF EXISTS orders")
        .execute(&state.pool)
        .await
        .unwrap();


    let _ = sqlx::query!(
            r"
          CREATE TABLE regions (
            id INT PRIMARY KEY,
            name VARCHAR(50)
          )")
        .execute(&state.pool)
        .await.unwrap();

    let _ = sqlx::query!(
            r"
          CREATE TABLE orders (
            id INT PRIMARY KEY,
            region_id INT,
            gift_name VARCHAR(50),
            quantity INT
          )")
        .execute(&state.pool)
        .await.unwrap();

    "".to_string()
}

pub async fn post_orders(State(state): State<d13::AppState>, Json(payload): Json<Vec<d13::Order>>) -> impl IntoResponse
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

#[derive(Deserialize, Debug)]
pub struct Region {
    pub id: i32,
    pub name: String,
}

pub async fn post_regions(State(state): State<d13::AppState>, Json(payload): Json<Vec<Region>>) -> impl IntoResponse
{
    for el in payload {
        let _ = sqlx::query!(
            "INSERT INTO regions (id, name) VALUES ($1, $2)",
                el.id,
                el.name,
            )
            .execute(&state.pool)
            .await.unwrap();
    }
    "".to_string()
}

#[derive(Serialize, FromRow, Default)]
pub struct SumResponse {
    region: String,
    total: i64,
}

pub async fn sum_regions(State(state): State<d13::AppState>) -> impl IntoResponse
{
    let totals = sqlx::query_as::<_, SumResponse>(
        r#"
            SELECT r.name AS region,
                sum(o.quantity) AS total
            FROM orders o
                JOIN regions r
                    ON o.region_id = r.id
            GROUP BY r.name
            ORDER BY r.name ASC
        "#,
    )
        .fetch_all(&state.pool)
        .await
        .unwrap();

    Json(totals)
}



#[derive(Serialize, FromRow, Default)]
pub struct TopGift {
    region: String,
    top_gifts: Vec<String>,
}

pub async fn top_gifts(State(state): State<d13::AppState>, Path(limit): Path<i32>
) -> impl IntoResponse {

    let top_gifts = sqlx::query_as::<_, TopGift>(
        r#"
            SELECT r.name AS region,
                array_remove(array_agg(o.gift_name), NULL) AS top_gifts
            FROM regions r
            LEFT JOIN LATERAL (
                SELECT o.gift_name,
                    sum(o.quantity) AS total_quantity
                FROM orders o
                WHERE o.region_id = r.id
                GROUP BY o.gift_name
                ORDER BY total_quantity DESC,
                    o.gift_name ASC
                LIMIT $1
                ) o ON TRUE
            GROUP BY r.name
            ORDER BY r.name ASC
        "#,
    )
        .bind(limit)
        .fetch_all(&state.pool)
        .await
        .unwrap();

    Json(top_gifts)
}
