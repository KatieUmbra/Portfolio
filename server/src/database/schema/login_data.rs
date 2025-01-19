use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::util::error::{ApiError, ApiErrorCode};

/// Struct that contains login data from users that call this route
#[derive(Serialize, sqlx::FromRow, Deserialize, Clone, Default, Debug)]
pub struct LoginData {
    pub username: String,
    pub password: String,
    #[serde(skip)]
    pub verified: i32,
}

impl LoginData {
    /// Selects **one** user from the database
    pub async fn select(&self, pool: &PgPool) -> Result<LoginData, ApiError> {
        let query = "SELECT * FROM users WHERE username=$1";
        let data = sqlx::query_as::<_, LoginData>(query)
            .bind(&self.username)
            .fetch_one(pool)
            .await
            .map_err(|_| ApiError {
                message: "This account does not exist, please register a new account to proceed"
                    .into(),
                status_code: StatusCode::NOT_FOUND,
                error_code: ApiErrorCode::LoginUsernameNotFound,
            })?;

        Ok(data)
    }

    pub async fn select_with_username(
        username: String,
        pool: &PgPool,
    ) -> Result<LoginData, ApiError> {
        let query = "SELECT * FROM users WHERE username=$1";
        let data = sqlx::query_as::<_, LoginData>(query)
            .bind(username)
            .fetch_one(pool)
            .await
            .map_err(|_| ApiError {
                message: "This account does not exist, please register a new account to proceed"
                    .into(),
                status_code: StatusCode::NOT_FOUND,
                error_code: ApiErrorCode::LoginUsernameNotFound,
            })?;

        Ok(data)
    }
}
