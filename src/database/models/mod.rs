use thiserror::Error;

pub mod chat;
pub mod message;
pub mod user;

const BCRYPT_HASH_ROUNDS: u32 = 11;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Error while interacting with the database: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Password hashing error: {0}")]
    PasswordHashingError(#[from] bcrypt::BcryptError),
}
