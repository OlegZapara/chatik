mod chats;
mod messages;
mod users;

use crate::AppConfig;
use axum::Router;

pub fn routes() -> Router<AppConfig> {
    Router::new()
        .nest("/users", users::routes())
        .nest("/chats", chats::routes())
        .nest("/messages", messages::routes())
}
