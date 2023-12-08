use axum::{
    extract::Path,
    extract::State,
    routing::get,
    Router,
};
use axum::response::IntoResponse;
use reqwest::Client;
use serde::Deserialize;

pub fn get_routes() -> Router {
    let client = reqwest::Client::new();
    Router::new()
        .route("/8/weight/:id", get(pokemon_weight))
        .route("/8/drop/:id", get(pokemon_drop))
        .with_state(client)
}

#[derive(Debug, Deserialize)]
struct PokemonData {
    weight: f64,
}

async fn get_weight(client: Client, user_id: u64) -> Result<f64, String>  {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", user_id);

    // Send the request and parse the response.
    match client.get(url).send().await {
        Ok(resp) => {
            if resp.status() == 200 {
                let pokemon = resp.json::<PokemonData>().await.unwrap();
                //println!("{:?}", pokemon.weight / 10.0);
                Ok(pokemon.weight / 10.0)
            }
            else {
                Err(format!("Reqwest Error: Response status = {}", resp.status()))
            }
        }
        Err(err) => {
            Err(format!("Reqwest Error: {}", err))
        }
    }

}

async fn pokemon_weight(State(client): State<Client>, Path(user_id): Path<u64>) -> impl IntoResponse  {
    match get_weight(client, user_id).await {
        Ok(weight) => {
            weight.to_string()
        }
        Err(err) => {
            err
        }
    }
}

const V_2_G_H: f64 = 2.0 * 9.825 * 10.0;
async fn pokemon_drop(State(client): State<Client>, Path(user_id): Path<u64>) -> impl IntoResponse  {
    match get_weight(client, user_id).await {
        Ok(weight) => {
            (weight*V_2_G_H.sqrt()).to_string()
        }
        Err(err) => {
            err
        }
    }
}
