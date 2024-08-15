mod chats;
mod messages;
mod users;
mod websocket;

use crate::AppConfig;
use axum::Router;

pub fn routes() -> Router<AppConfig> {
    Router::new()
        .nest("/users", users::routes())
        .nest("/chats", chats::routes())
        .nest("/messages", messages::routes())
        .nest("/ws", websocket::routes())
}
