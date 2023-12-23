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
        .route("/22/rocket", post(find_portal))

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

fn star_dist(a: (i32, i32, i32), b: (i32, i32, i32)) -> f32 {
    let total = (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2);
    (total as f32).sqrt()
}

async fn find_portal(body: String) -> impl IntoResponse {

    let mut lines = body.lines();
    // Line 1: Stars count.
    let stars_count = lines.next().unwrap().parse::<u32>().unwrap();
    // Next stars_count lines
    let mut stars = vec![];
    for _ in 0..stars_count {
        let s = lines.next().unwrap();
        let mut s = s.split_ascii_whitespace().take(3);
        let star = (
            s.next().unwrap().parse().unwrap(),
            s.next().unwrap().parse().unwrap(),
            s.next().unwrap().parse().unwrap(),
        );
        stars.push(star);
    }
    // Next line: portal count
    let portals_count = lines.next().unwrap().parse::<u32>().unwrap();

    let mut portals = multimap::MultiMap::<u32, u32>::new();
    for _ in 0..portals_count {
        let s = lines.next().unwrap();

        //let (s, e) = portal_path(input.next().expect("Line exists"));

        let mut s = s.split_ascii_whitespace().take(3);
        let (s, e) = (
            s.next().unwrap().parse().unwrap(),
            s.next().unwrap().parse().unwrap(),
        );
        portals.insert(s, e);
    }

    let Some(path) = pathfinding::directed::bfs::bfs(
        &0u32,
        |n| portals.get_vec(n).cloned().unwrap_or_default(),
        |&n| n == stars_count - 1,
    ) else {
        return "StatusCode::INTERNAL_SERVER_ERROR".to_string();
    };

    let d = path
        .windows(2)
        .map(|p| star_dist(stars[p[0] as usize], stars[p[1] as usize]))
        .sum::<f32>();

    format!("{} {d:.3}", path.len() - 1)
}