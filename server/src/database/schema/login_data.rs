use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::util::error::{ApiError, ApiErrorCode};

#[derive(Serialize, sqlx::FromRow, Deserialize, Clone)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

impl LoginData {
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
}
