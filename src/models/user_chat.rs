use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserChat {
    pub user_id: i32,
    pub chat_id: i32,
    pub is_admin: Option<bool>,
}
