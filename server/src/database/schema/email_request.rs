use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    database::util::user_db_map_error,
    util::error::{generic_error, ApiError, ApiErrorCode},
};

/// Struct for the Postgres database table that contains email requests
#[derive(Serialize, sqlx::FromRow, Deserialize, Clone, Default)]
pub struct EmailRequest {
    /// Username of the user who made the email request
    pub username: String,
    /// The token sent to the email
    pub secret: String,
    /// The operation that's suposed to be completed once accessed
    /// - 0: Verify
    /// - 1: Reset password
    pub operation: i32,
    /// Expiration of the request
    pub expiration: chrono::DateTime<chrono::Utc>,
    /// Unique id of the request
    pub id: i32,
}

impl EmailRequest {
    /// Inserts a request into the database
    pub async fn insert(&self, pool: &PgPool) -> Result<(), ApiError> {
        let old_request = self.select(pool).await;
        if let Err(_) = old_request {
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
        } else {
            Err(ApiError {
                message: "There are email requests pending in the database".into(),
                status_code: StatusCode::BAD_REQUEST,
                error_code: ApiErrorCode::InternalUnspecifiedError,
            })
        }
    }

    /// Selects **one** email request from the database using the username
    pub async fn select(&self, pool: &PgPool) -> Result<EmailRequest, ApiError> {
        let _ = &self.verify_requests(&pool);
        let query = "SELECT * FROM email_requests WHERE username=$1 AND operation=$2";
        let data = sqlx::query_as::<_, EmailRequest>(query)
            .bind(&self.username)
            .bind(&self.operation)
            .fetch_one(pool)
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalSqlxError))?;
        Ok(data)
    }

    /// Verifies that there's only one request of a type related to a username at a time
    pub async fn verify_requests(&self, pool: &PgPool) {
        let now = chrono::Utc::now();
        let query = "DELETE FROM email_requests\n WHERE username=$1 AND date<$2";
        let _ = sqlx::query(query)
            .bind(&self.username)
            .bind(now)
            .fetch(pool);
    }

    /// Deletes a request from the database
    pub async fn delete(&self, pool: &PgPool) {
        let query = "DELETE FROM email_requests\n WHERE id=$1";
        let _ = sqlx::query(query).bind(&self.id).execute(pool).await;
    }
}
