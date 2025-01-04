use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::{
    fs::{remove_file, File},
    io::AsyncWriteExt,
};

use crate::{
    routing::routes::structs::ApiResult,
    util::error::{ApiError, ApiErrorCode},
};

#[derive(Serialize, sqlx::FromRow, Deserialize, Clone, Default)]
pub struct Post {
    pub creator: String,
    #[sqlx(default)]
    pub content: String,
    pub description: String,
    pub title: String,
    pub creation: chrono::DateTime<chrono::Utc>,
    pub likes: i32,
}

#[derive(Serialize, sqlx::FromRow, Deserialize, Clone, Default)]
pub struct Comment {
    creator: String,
    post: u32,
    parent: Option<u32>,
    date: DateTime<Utc>,
    content: String,
    likes: u32,
}

#[derive(Debug, Serialize, sqlx::FromRow, Deserialize, Clone, Default)]
pub struct IdWrapper {
    pub id: i32,
}

impl Post {
    pub async fn create(self, pool: &PgPool) -> ApiResult {
        let error = ApiError {
            message: "An internal error has occurred, please contact support".into(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: ApiErrorCode::InternalErrorContactSupport,
        };
        let query = "INSERT INTO posts (creator, description, title, creation, likes) VALUES ($1, $2, $3, $4, 0) RETURNING id";
        let result: IdWrapper = sqlx::query_as(query)
            .bind(&self.creator)
            .bind(&self.description)
            .bind(&self.title)
            .bind(Utc::now())
            .fetch_one(pool)
            .await
            .map_err(|_| error.clone())?;
        let html = markdown::to_html(&self.content);
        let file_path = format!("posts/{}.html", result.id);
        let mut file = File::create(file_path).await.map_err(|_| error.clone())?;
        file.write_all(html.as_bytes()).await.map_err(|_| error)?;
        Ok(())
    }

    pub async fn get(id: i32, pool: &PgPool) -> Result<Post, ApiError> {
        let query = "SELECT * FROM posts WHERE id = $1";
        tracing::debug!("id: {}", id);
        let mut result = sqlx::query_as::<_, Post>(query)
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::debug!("{:?}", e);
                ApiError {
                    message: "The requested post couldn't be found.".into(),
                    status_code: StatusCode::NOT_FOUND,
                    error_code: ApiErrorCode::InternalNotFound,
                }
            })?;
        let file_path = format!("posts/{}.html", id);
        let content_res = tokio::fs::read_to_string(file_path).await;
        let content = match content_res {
            Ok(x) => x,
            Err(_) => {
                Post::delete(id, &pool).await?;
                return Err(ApiError {
                    message: "The file for the post couldn't be found.".into(),
                    status_code: StatusCode::NOT_FOUND,
                    error_code: ApiErrorCode::InternalNotFound,
                });
            }
        };
        result.content = content;
        Ok(result)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> ApiResult {
        let query = "DELETE FROM posts WHERE id = $1;";
        let _ = sqlx::query(query)
            .bind(id)
            .execute(pool)
            .await
            .map_err(|_| ApiError {
                message: "An internal error has occurred, please contact support".into(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error_code: ApiErrorCode::InternalErrorContactSupport,
            })?;
        let file_path = format!("posts/{}.html", id);
        let file = File::open(file_path.clone()).await;
        match file {
            Err(_) => return Ok(()),
            Ok(mut x) => {
                let _ = x.shutdown();
                let _ = remove_file(file_path).await;
            }
        }
        Ok(())
    }
}
