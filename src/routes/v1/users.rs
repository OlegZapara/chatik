use crate::database::models::chat::Chat;
use crate::database::models::user::{CreateUserDto, UpdateUserDto, User};
use crate::AppConfig;
use axum::extract::{Path, State};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

pub fn routes() -> Router<AppConfig> {
    Router::new()
        .route("/", get(list_users))
        .route("/:id", get(get_user_by_id))
        .route("/", post(create_user))
        .route("/", put(update_user))
        .route("/:id", delete(delete_user))
        .route("/:id/chats", get(get_user_chats))
}

async fn list_users(State(pool): State<PgPool>) -> Json<Vec<User>> {
    let users = User::list(&pool).await.unwrap();
    Json(users)
}

async fn get_user_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Json<User> {
    let user = User::get_by_id(id, &pool).await.unwrap();
    Json(user)
}

async fn create_user(
    State(pool): State<PgPool>,
    Json(user_dto): Json<CreateUserDto>,
) -> Json<User> {
    let user = CreateUserDto::insert(&user_dto, &pool).await.unwrap();
    Json(user)
}

async fn update_user(
    State(pool): State<PgPool>,
    Json(user_dto): Json<UpdateUserDto>,
) -> Json<User> {
    let user = UpdateUserDto::update(&user_dto, &pool).await.unwrap();
    Json(user)
}

async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Json<String> {
    User::delete(id, &pool).await.unwrap();
    Json(String::from("User deleted"))
}

async fn get_user_chats(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Json<Vec<Chat>> {
    let existing_user = User::get_by_id(id, &pool).await.unwrap();
    let chats = existing_user.get_chats(&pool).await.unwrap();
    Json(chats)
}
