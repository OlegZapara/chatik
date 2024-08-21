mod chats;
mod messages;
mod users;
mod websocket;

use crate::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/users", users::routes())
        .nest("/chats", chats::routes())
        .nest("/messages", messages::routes())
        .nest("/ws", websocket::routes())
}
