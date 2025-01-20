use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    database::util::user_db_map_error,
    util::error::{generic_error, ApiError, ApiErrorCode},
};

/// Contains user data
#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct UserData {
    pub username: String,
    #[sqlx(rename = "displayname")]
    pub display_name: String,
    pub email: String,
    pub password: String,
}

impl UserData {
    /// Inserts a new user into the database
    /// ## Note
    /// **The password of any user that is to be inserted into the database has to be hashed**
    pub async fn insert(&self, pool: &PgPool) -> Result<(), ApiError> {
        let query =
        "INSERT INTO users (username, displayUsername, displayName, email, password, verified) VALUES ($1, $2, $3, $4, $5, $6)";
        let _ = sqlx::query(query)
            .bind(&self.username.to_lowercase())
            .bind(&self.username)
            .bind(&self.display_name)
            .bind(&self.email.to_lowercase())
            .bind(&self.password)
            .bind(2)
            .execute(pool)
            .await
            .map_err(user_db_map_error)?;

        Ok(())
    }

    /// Selects **one** user from the database querying by the username
    pub async fn select(username: String, pool: &PgPool) -> Result<Self, ApiError> {
        let query = "SELECT * FROM users WHERE username=$1";
        let data = sqlx::query_as::<_, Self>(query)
            .bind(&username)
            .fetch_one(pool)
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalSqlxError))?;

        Ok(data)
    }

    /// Changes the database field `verified` from `3` to `2`
    pub async fn verify(&self, pool: &PgPool) -> Result<(), ApiError> {
        let query = "UPDATE users SET verified=2 WHERE username=$1";
        let _ = sqlx::query(query)
            .bind(&self.username)
            .execute(pool)
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalSqlxError))?;

        Ok(())
    }
}
