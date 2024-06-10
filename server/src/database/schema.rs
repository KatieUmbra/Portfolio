use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, sqlx::FromRow, Deserialize, Clone)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

impl UserData {
    pub async fn insert(&self, pool: &PgPool) -> Result<(), StatusCode> {
        let query =
        "INSERT INTO users (username, displayUsername, displayName, email, password, verified) VALUES ($1, $2, $3, $4, $5, $6)";
        let _ = sqlx::query(query)
            .bind(&self.username.to_lowercase())
            .bind(&self.username)
            .bind(&self.display_name)
            .bind(&self.email.to_lowercase())
            .bind(&self.password)
            .bind(false)
            .execute(pool)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        Ok(())
    }
}

impl LoginData {
    pub async fn select(&self, pool: &PgPool) -> Result<LoginData, StatusCode> {
        let query = "SELECT * FROM users WHERE username=$1";
        let data = sqlx::query_as::<_, LoginData>(query)
            .bind(&self.username)
            .fetch_one(pool)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        Ok(data)
    }
}
