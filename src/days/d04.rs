use axum::{
    routing::post,
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
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candies_eaten_yesterday: i32,
}

#[derive(Default, Serialize)]
struct ContestResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/4/strength", post(sum_strength))
        .route("/4/contest", post(eating_contest))
}

async fn sum_strength(Json(body): Json<Vec<Reindeer>>) -> String {
    let res = body
        .iter()
        .map(|item| item.strength)
        .sum::<i32>();

    res.to_string()
}

#[allow(clippy::unused_async, dead_code)]
async fn eating_contest_old(Json(body): Json<Vec<ReindeerData>>) -> Json<ContestResponse> {

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


async fn eating_contest(Json(body): Json<Vec<ReindeerData>>) -> Json<ContestResponse> {

    let Some(first) = body.first() else {
        return Json(ContestResponse::default());
    };

    let (mut fastest, mut tallest, mut magician, mut consumer) = (first, first, first, first);
    for x in &body {
        if fastest.speed < x.speed {
            fastest = x;
        }
        if tallest.height < x.height {
            tallest = x;
        }
        if magician.snow_magic_power < x.snow_magic_power {
            magician = x;
        }
        if consumer.candies_eaten_yesterday < x.candies_eaten_yesterday {
            consumer = x;
        }
    }
    Json(ContestResponse {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name,
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width,
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power,
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food,
        ),
    })
}
