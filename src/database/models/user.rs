use crate::database::models::chat::Chat;
use crate::database::models::{DatabaseError, BCRYPT_HASH_ROUNDS};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: Option<String>,
    pub profile_img: Option<String>,
    pub about: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub password: Option<String>,
    pub profile_img: Option<String>,
    pub about: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub id: Uuid,
    pub username: String,
    pub password: Option<String>,
    pub profile_img: Option<String>,
    pub about: Option<String>,
}

impl User {
    pub async fn list<'a, E>(exec: E) -> Result<Vec<User>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query_as!(
            User,
            "
            SELECT * FROM users
            ",
        )
        .fetch_all(exec)
        .await?;

        Ok(result)
    }

    pub async fn get_by_id<'a, E>(
        id: Uuid,
        exec: E,
    ) -> Result<User, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query_as!(
            User,
            "
            SELECT * FROM users
            WHERE id = $1
            ",
            id,
        )
        .fetch_one(exec)
        .await?;

        Ok(result)
    }

    pub async fn delete<'a, E>(id: Uuid, exec: E) -> Result<(), DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query!(
            "
            DELETE FROM users
            WHERE id = $1
            ",
            id,
        )
        .execute(exec)
        .await?;

        Ok(())
    }

    pub async fn is_admin<'a, E>(
        &self,
        chat_id: Uuid,
        exec: E,
    ) -> Result<bool, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT is_admin FROM users_chats
            WHERE user_id = $1 AND chat_id = $2
            ",
            self.id,
            chat_id,
        )
        .fetch_one(exec)
        .await?;

        Ok(result.is_admin.unwrap_or(false))
    }

    pub async fn get_chats<'a, E>(
        &self,
        exec: E,
    ) -> Result<Vec<Chat>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query_as!(
            Chat,
            "
            SELECT chats.* FROM chats
            JOIN users_chats ON chats.id = users_chats.chat_id
            WHERE users_chats.user_id = $1
            ",
            self.id,
        )
        .fetch_all(exec)
        .await?;

        Ok(result)
    }
}

impl CreateUserDto {
    pub async fn insert<'a, E>(&self, exec: E) -> Result<User, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let password_hash = match self.password {
            Some(ref password) => {
                Some(bcrypt::hash(password, BCRYPT_HASH_ROUNDS)?)
            }
            None => None,
        };
        let result = sqlx::query_as!(
            User,
            "
            INSERT INTO users (username, password_hash, profile_img, about)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            ",
            self.username,
            password_hash,
            self.profile_img,
            self.about,
        )
        .fetch_one(exec)
        .await?;

        Ok(result)
    }
}

impl UpdateUserDto {
    pub async fn update<'a, E>(&self, exec: E) -> Result<User, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let password_hash = match self.password {
            Some(ref password) => {
                Some(bcrypt::hash(password, BCRYPT_HASH_ROUNDS)?)
            }
            None => None,
        };
        let result = sqlx::query_as!(
            User,
            "
            UPDATE users
            SET username = $2, password_hash = $3, profile_img = $4, about = $5
            WHERE id = $1
            RETURNING *
            ",
            self.id,
            self.username,
            password_hash,
            self.profile_img,
            self.about,
        )
        .fetch_one(exec)
        .await?;

        Ok(result)
    }
}
