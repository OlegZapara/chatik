use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub origin_id: Uuid,
    pub from_id: Uuid,
    pub to_id: Uuid,
    pub to_type: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageDto {
    pub origin_id: Uuid,
    pub from_id: Uuid,
    pub to_id: Uuid,
    pub to_type: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessageDto {
    pub id: Uuid,
    pub origin_id: Uuid,
    pub from_id: Uuid,
    pub to_id: Uuid,
    pub to_type: String,
    pub message: String,
}

impl Message {
    pub async fn list<'a, E>(exec: E) -> Result<Vec<Message>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query_as!(
            Message,
            "
            SELECT * FROM messages
            ",
        )
        .fetch_all(exec)
        .await?;

        Ok(result)
    }

    pub async fn get_by_id<'a, E>(
        id: Uuid,
        exec: E,
    ) -> Result<Message, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query_as!(
            Message,
            "
            SELECT * FROM messages WHERE id=$1
            ",
            id,
        )
        .fetch_one(exec)
        .await?;

        Ok(result)
    }

    pub async fn delete<'a, E>(id: Uuid, exec: E) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query!(
            "
            DELETE FROM messages WHERE id=$1
            ",
            id,
        )
        .execute(exec)
        .await?;

        Ok(())
    }
}

impl CreateMessageDto {
    pub async fn insert<'a, E>(&self, exec: E) -> Result<Message, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query_as!(
            Message,
            "
            INSERT INTO messages (origin_id, from_id, to_id, to_type, message)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            ",
            self.origin_id,
            self.from_id,
            self.to_id,
            self.to_type,
            self.message,
        )
        .fetch_one(exec)
        .await?;

        Ok(result)
    }
}

impl UpdateMessageDto {
    pub async fn update<'a, E>(&self, exec: E) -> Result<Message, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query_as!(
            Message,
            "
            UPDATE messages
            SET origin_id=$1, from_id=$2, to_id=$3, to_type=$4, message=$5
            WHERE id=$6
            RETURNING *
            ",
            self.origin_id,
            self.from_id,
            self.to_id,
            self.to_type,
            self.message,
            self.id,
        )
        .fetch_one(exec)
        .await?;

        Ok(result)
    }
}
