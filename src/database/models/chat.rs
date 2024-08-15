use crate::database::models::user::User;
use crate::database::models::{DatabaseError, BCRYPT_HASH_ROUNDS};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Chat {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub password_hash: Option<String>,
    pub profile_img: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub password: Option<String>,
    pub profile_img: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatDto {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub password: Option<String>,
    pub profile_img: Option<String>,
}

impl Chat {
    pub async fn list<'a, E>(exec: E) -> Result<Vec<Chat>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query_as!(
            Chat,
            "
            SELECT * FROM chats
            ",
        )
        .fetch_all(exec)
        .await?;

        Ok(result)
    }

    pub async fn get_by_id<'a, E>(
        id: Uuid,
        exec: E,
    ) -> Result<Chat, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query_as!(
            Chat,
            "
            SELECT * FROM chats WHERE id=$1
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
            DELETE FROM chats
            WHERE id = $1
            ",
            id,
        )
        .execute(exec)
        .await?;

        Ok(())
    }

    pub async fn add_user<'a, E>(
        &self,
        user_id: Uuid,
        exec: E,
    ) -> Result<(), DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query!(
            "
            INSERT INTO users_chats (user_id, chat_id)
            VALUES ($1, $2)
            ",
            user_id,
            self.id,
        )
        .execute(exec)
        .await?;

        Ok(())
    }

    pub async fn remove_user<'a, E>(
        &self,
        user_id: Uuid,
        exec: E,
    ) -> Result<(), DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query!(
            "
            DELETE FROM users_chats
            WHERE user_id = $1 AND chat_id = $2
            ",
            user_id,
            self.id,
        )
        .execute(exec)
        .await?;

        Ok(())
    }

    pub async fn get_users<'a, E>(
        &self,
        exec: E,
    ) -> Result<Vec<User>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query_as!(
            User,
            "
            SELECT users.* FROM users
            JOIN users_chats ON users.id = users_chats.user_id
            WHERE users_chats.chat_id = $1
            ",
            self.id,
        )
        .fetch_all(exec)
        .await?;

        Ok(result)
    }
}

impl CreateChatDto {
    pub async fn insert<'a, E>(&self, exec: E) -> Result<Chat, DatabaseError>
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
            Chat,
            "
            INSERT INTO chats (name, description, password_hash, profile_img)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            ",
            self.name,
            self.description,
            password_hash,
            self.profile_img,
        )
        .fetch_one(exec)
        .await?;

        Ok(result)
    }
}

impl UpdateChatDto {
    pub async fn update<'a, E>(&self, exec: E) -> Result<Chat, DatabaseError>
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
            Chat,
            "
            UPDATE chats
            SET name = $1, description = $2, password_hash = $3, profile_img = $4
            WHERE id = $5
            RETURNING *
            ",
            self.name,
            self.description,
            password_hash,
            self.profile_img,
            self.id,
        )
        .fetch_one(exec)
        .await?;

        Ok(result)
    }
}
