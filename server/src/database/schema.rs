use std::collections::HashMap;

use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono, PgPool};

use crate::util::error::{ApiError, ApiErrorCode};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct UserData {
    pub username: String,
    #[sqlx(rename = "displayname")]
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, sqlx::FromRow, Deserialize, Clone)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, sqlx::FromRow, Deserialize, Clone, Default)]
pub struct EmailRequest {
    pub username: String,
    pub secret: String,
    pub operation: i32,
    pub expiration: chrono::DateTime<chrono::Utc>,
}

pub fn user_db_map_error(error: sqlx::Error) -> ApiError {
    let err1 = ApiError {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: ApiErrorCode::InternalError,
        message: "Something went wrong, try again later".to_string(),
    };

    let db_constraint_error_map: HashMap<&str, ApiError> = HashMap::from([
        (
            "users_pkey",
            ApiError {
                message: "This username already exists!".to_string(),
                error_code: ApiErrorCode::RegisterUsernameExists,
                status_code: StatusCode::CONFLICT,
            },
        ),
        (
            "users_email_key",
            ApiError {
                message: "This email is already in use!".to_string(),
                error_code: ApiErrorCode::RegisterEmailExists,
                status_code: StatusCode::CONFLICT,
            },
        ),
    ]);

    let error = error.into_database_error();
    if let Some(res) = error {
        println!("{:?}", &res);
        if let Some(constraint) = res.constraint() {
            println!("constraint: {:?}", res);
            db_constraint_error_map.get(constraint).unwrap().clone()
        } else {
            err1
        }
    } else {
        err1
    }
}

impl UserData {
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

    pub async fn select(username: String, pool: &PgPool) -> Result<Self, ApiError> {
        let query = "SELECT * FROM users WHERE username=$1";
        let data = sqlx::query_as::<_, Self>(query)
            .bind(&username)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("{:?}", e);

                ApiError {
                    message: "Internal error idk".into(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_code: ApiErrorCode::None,
                }
            })?;

        Ok(data)
    }

    pub async fn verify(&self, pool: &PgPool) -> Result<(), ApiError> {
        let query = "UPDATE users SET verified=1 WHERE username=$1";
        let _ = sqlx::query(query)
            .bind(&self.username)
            .execute(pool)
            .await
            .map_err(|_| ApiError {
                message: "Internal error".into(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error_code: ApiErrorCode::InternalErrorContactSupport,
            })?;

        Ok(())
    }
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
        let query = "DELETE FROM email_requests WHERE username=$1 AND date<$2";
        let _ = sqlx::query(query)
            .bind(&self.username)
            .bind(now)
            .fetch(pool);
    }

    pub async fn delete(&self, pool: &PgPool) {
        let query = "DELETE FROM email_requests WHERE username=$1 AND secret=$2";
        let _ = sqlx::query(query)
            .bind(&self.username)
            .bind(&self.secret)
            .execute(pool);
        tracing::debug!("Delete query result: {:?}", query);
    }
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
