use crate::database::models::message::{
    CreateMessageDto, Message, UpdateMessageDto,
};
use crate::AppState;
use axum::extract::{Path, State};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_messages))
        .route("/:id", get(get_message_by_id))
        .route("/", post(create_message))
        .route("/", put(update_message))
        .route("/:id", delete(delete_message))
}

async fn list_messages(State(pool): State<PgPool>) -> Json<Vec<Message>> {
    let messages = Message::list(&pool).await.unwrap();
    Json(messages)
}

async fn get_message_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Json<Message> {
    let message = Message::get_by_id(id, &pool).await.unwrap();
    Json(message)
}

async fn create_message(
    State(pool): State<PgPool>,
    Json(message_dto): Json<CreateMessageDto>,
) -> Json<Message> {
    let message = CreateMessageDto::insert(&message_dto, &pool).await.unwrap();
    Json(message)
}

async fn update_message(
    State(pool): State<PgPool>,
    Json(message_dto): Json<UpdateMessageDto>,
) -> Json<Message> {
    let message = UpdateMessageDto::update(&message_dto, &pool).await.unwrap();
    Json(message)
}

async fn delete_message(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Json<String> {
    Message::delete(id, &pool).await.unwrap();
    Json(String::from("Message deleted"))
}
