use crate::AppConfig;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};

pub fn routes() -> Router<AppConfig> {
    Router::new().route("/", get(ws_handler))
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppConfig>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppConfig) {
    increment_connections(&state);
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                if socket.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
            Message::Binary(bin) => {
                if socket.send(Message::Binary(bin)).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => (),
        }
    }
    decrement_connections(&state);
}

pub fn increment_connections(state: &AppConfig) {
    let mut counter = state.active_connections.lock().unwrap();
    *counter += 1;
    log::info!(target: "chatik.ws", "CLIENT CONNECTED. Active: {}", *counter);
}

pub fn decrement_connections(state: &AppConfig) {
    let mut counter = state.active_connections.lock().unwrap();
    if *counter > 0 {
        *counter -= 1;
    }
    log::info!(target: "chatik.ws", "CLIENT DISCONNECTED. Active: {}", *counter);
}
