use axum::{
    routing::{get, post},
    http::{StatusCode},
    response::{IntoResponse, Response},
    extract::Path,
    Router,
    Json
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Reindeer {
    // name: String,
    strength: i32,
}

#[derive(Deserialize, Debug)]
struct ReindeerData {
    name: String,
    //strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candies_eaten_yesterday: i32,
}

#[derive(Serialize)]
struct ContestResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

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

async fn sum_strength(Json(body): Json<Vec<Reindeer>>) -> String {
    let res = body
        .iter()
        .map(|item| item.strength)
        .sum::<i32>();

    res.to_string()
}

async fn eating_contest(Json(body): Json<Vec<ReindeerData>>) -> Json<ContestResponse> {

    // Find out faster
    let fastest = body
        .iter()
        .max_by(|a, b| a.speed.total_cmp(&b.speed))
        .unwrap();

    // Find out tallest
    let tallest = body
        .iter()
        .max_by(|a, b| a.height.cmp(&b.height))
        .unwrap();

    // Find out magician
    let magician = body
        .iter()
        .max_by(|a, b| a.snow_magic_power.cmp(&b.snow_magic_power))
        .unwrap();

    // Find out consumer
    let consumer = body
        .iter()
        .max_by(|a, b| a.candies_eaten_yesterday.cmp(&b.candies_eaten_yesterday))
        .unwrap();

    // Format contest response.
    let res = ContestResponse {
        fastest: format!("Speeding past the finish line with a strength of {} is {}", fastest.speed, fastest.name),
        tallest: format!("{} is standing tall with his {} cm wide antlers", tallest.name, tallest.antler_width),
        magician: format!("{} could blast you away with a snow magic power of {}", magician.name, magician.snow_magic_power),
        consumer: format!("{} ate lots of candies, but also some {}", consumer.name, consumer.favorite_food),
    };

    Json(res)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(handle_error))
        .route("/1/*nums", get(cube_the_bits))
        .route("/4/strength", post(sum_strength))
        .route("/4/contest", post(eating_contest));

    Ok(router.into())
}
