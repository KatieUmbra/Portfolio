use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    database::util::user_db_map_error,
    util::error::{ApiError, ApiErrorCode},
};

#[derive(Serialize, sqlx::FromRow, Deserialize, Clone, Default)]
pub struct EmailRequest {
    pub username: String,
    pub secret: String,
    pub operation: i32,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub id: i32,
}

impl EmailRequest {
    pub async fn insert(&self, pool: &PgPool) -> Result<(), ApiError> {
        let query = "INSERT INTO email_requests (username, secret, operation, expiration) VALUES ($1, $2, $3, $4);";
        let _ = sqlx::query(query)
            .bind(&self.username.to_lowercase())
            .bind(&self.secret)
            .bind(&self.operation)
            .bind(&self.expiration)
            .execute(pool)
            .await
            .map_err(user_db_map_error)?;

        Ok(())
    }

    pub async fn select(&self, pool: &PgPool) -> Result<EmailRequest, ApiError> {
        let _ = &self.verify_requests(&pool);
        let query = "SELECT * FROM email_requests WHERE username=$1;";
        let data = sqlx::query_as::<_, EmailRequest>(query)
            .bind(&self.username)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("{:?}", e);
                ApiError {
                    message: "Internal Server Error (Email Requests Select)".into(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_code: ApiErrorCode::None,
                }
            })?;
        Ok(data)
    }

    pub async fn verify_requests(&self, pool: &PgPool) {
        let now = chrono::Utc::now();
        let query = "DELETE FROM email_requests\n WHERE username=$1 AND date<$2";
        let _ = sqlx::query(query)
            .bind(&self.username)
            .bind(now)
            .fetch(pool);
    }

    pub async fn delete(&self, pool: &PgPool) {
        let query = "DELETE FROM email_requests\n WHERE id=$1";
        let _ = sqlx::query(query).bind(&self.id).execute(pool).await;
    }
}
