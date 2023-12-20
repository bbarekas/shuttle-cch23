use std::{collections::HashMap, sync::Arc};
use futures::{
    stream::SplitSink,
    SinkExt, StreamExt,
};
use axum::{
    Router,
    routing::{get, post},
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::{State, Path},
    http::StatusCode,
};
use axum::response::IntoResponse;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

pub fn get_routes() -> Router {

    let state = BirdAppState::new();

    Router::new()
        .route("/19", get(StatusCode::OK))
        .route("/19/ws/ping", get(ws_ping))
        .route("/19/reset", post(reset_views))
        .route("/19/views", get(get_views))
        .route("/19/ws/room/:room/user/:user", get(ws_room))
        .with_state(state)
}

pub async fn ws_ping(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut started  = false;

    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        //println!("{:?}", msg);
        if let Message::Text(request) = msg {
            if request == "serve" {
                started = true;
            }
            if started && request == "ping" {
                let response = Message::Text("pong".to_string());

                if socket.send(response).await.is_err() {
                    // client disconnected
                    return;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct BirdAppState {
    views: Arc<RwLock<u32>>,
    rooms: Arc<RwLock<HashMap<u32, Vec<SplitSink<WebSocket, Message>>>>>
}

impl BirdAppState {
    fn new() -> Self {
        Self {
            views: Arc::new(RwLock::new(0)),
            rooms: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Tweet {
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
    message: String,
}

async fn reset_views(State(state): State<BirdAppState>) {
    *state.views.write().await = 0;
}

async fn get_views(State(state): State<BirdAppState>) -> impl IntoResponse {
    state.views.read().await.to_string()
}

async fn ws_room(
    ws: WebSocketUpgrade,
    State(state): State<BirdAppState>,
    Path((room, user)): Path<(u32, String)>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_room(socket, state, room, user))
}

async fn handle_room(ws: WebSocket, state: BirdAppState, room: u32, user: String) {

    let (sender, mut receiver) = ws.split();

    let mut rooms = state.rooms.write().await;
    if let Some(senders) = rooms.get_mut(&room) {
        senders.push(sender);
    } else {
        rooms.insert(room, vec![sender]);
    }
    drop(rooms);

    while let Some(msg) = receiver.next().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };
        //println!("RECV MSG: {:?}", msg);
        match msg {
            Message::Text(tweet) => {
                if let Ok(mut tweet) = serde_json::from_str::<Tweet>(&tweet) {
                    if tweet.user.is_none() {
                        if tweet.message.len() <= 128 {
                            tweet.user = Some(user.clone());

                            let message = Message::Text(serde_json::to_string(&tweet).unwrap());

                            let mut rooms = state.rooms.write().await;
                            if let Some(senders) = rooms.get_mut(&room) {
                                for sender in senders {
                                    if sender.send(message.clone()).await.is_ok() {
                                        // println!("SEND MSG: {:?}", message);
                                        *state.views.write().await += 1;
                                        // if error, client disconnected
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Message::Close(_) => {
                // client disconnected
                return;
            }
            _ => (),
        };
    }
}