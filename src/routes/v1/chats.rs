use crate::database::models::chat::{Chat, CreateChatDto, UpdateChatDto};
use crate::database::models::user::User;
use crate::AppConfig;
use axum::extract::{Path, State};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

pub fn routes() -> Router<AppConfig> {
    Router::new()
        .route("/", get(list_chats))
        .route("/:id", get(get_chat_by_id))
        .route("/", post(create_chat))
        .route("/", put(update_chat))
        .route("/:id", delete(delete_chat))
        .route("/:id/users", get(get_chat_users))
        .route("/:chat_id/add-user/:user_id", post(add_user))
        .route("/:chat_id/remove-user/:user_id", delete(remove_user))
}

async fn list_chats(State(pool): State<PgPool>) -> Json<Vec<Chat>> {
    let chats = Chat::list(&pool).await.unwrap();
    Json(chats)
}

async fn get_chat_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Json<Chat> {
    let chat = Chat::get_by_id(id, &pool).await.unwrap();
    Json(chat)
}

async fn create_chat(
    State(pool): State<PgPool>,
    Json(chat_dto): Json<CreateChatDto>,
) -> Json<Chat> {
    let chat = CreateChatDto::insert(&chat_dto, &pool).await.unwrap();
    Json(chat)
}

async fn update_chat(
    State(pool): State<PgPool>,
    Json(chat_dto): Json<UpdateChatDto>,
) -> Json<Chat> {
    let chat = UpdateChatDto::update(&chat_dto, &pool).await.unwrap();
    Json(chat)
}

async fn delete_chat(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Json<String> {
    Chat::delete(id, &pool).await.unwrap();
    Json(String::from("Chat deleted"))
}

async fn get_chat_users(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Json<Vec<User>> {
    let chat = Chat::get_by_id(id, &pool).await.unwrap();
    let users = chat.get_users(&pool).await.unwrap();
    Json(users)
}

async fn add_user(
    State(pool): State<PgPool>,
    Path((chat_id, user_id)): Path<(Uuid, Uuid)>,
) -> Json<String> {
    let existing_chat = Chat::get_by_id(chat_id, &pool).await.unwrap();
    existing_chat.add_user(user_id, &pool).await.unwrap();
    Json(format!(
        "User {} added to chat {}",
        user_id,
        existing_chat.name.unwrap_or(existing_chat.id.to_string())
    ))
}

async fn remove_user(
    State(pool): State<PgPool>,
    Path((chat_id, user_id)): Path<(Uuid, Uuid)>,
) -> Json<String> {
    let existing_chat = Chat::get_by_id(chat_id, &pool).await.unwrap();
    existing_chat.remove_user(user_id, &pool).await.unwrap();
    Json(format!(
        "User {} removed from chat {}",
        user_id,
        existing_chat.name.unwrap_or(existing_chat.id.to_string())
    ))
}
