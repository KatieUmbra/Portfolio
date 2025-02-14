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
    util::{
        error::{generic_error, ApiError, ApiErrorCode},
        markdown::markdown_to_html,
    },
};

#[derive(Serialize, sqlx::FromRow, Deserialize, Clone, Default)]
pub struct Post {
    pub id: i32,
    pub creator: String,
    #[sqlx(default)]
    pub content: String,
    pub description: String,
    pub title: String,
    pub creation: chrono::DateTime<chrono::Utc>,
    pub likes: i32,
}

#[derive(Serialize, sqlx::FromRow, Deserialize, Clone, Default)]
pub struct PostData {
    pub title: String,
    pub description: String,
    pub content: String,
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
pub struct VecWrapper<T> {
    pub vec: Vec<T>,
}

#[derive(Debug, Serialize, sqlx::FromRow, Deserialize, Clone, Default)]
pub struct IdWrapper {
    pub id: i32,
}

#[derive(Debug, Serialize, sqlx::FromRow, Deserialize, Clone, Default)]
pub struct I32Wrapper {
    pub amount: i32,
}

impl Post {
    pub async fn create(self, pool: &PgPool) -> ApiResult {
        let query = "INSERT INTO posts (creator, description, title, creation, likes) VALUES ($1, $2, $3, $4, 0) RETURNING id";
        let result: IdWrapper = sqlx::query_as(query)
            .bind(&self.creator)
            .bind(&self.description)
            .bind(&self.title)
            .bind(Utc::now())
            .fetch_one(pool)
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalSqlxError))?;

        // Parsing to html
        let html = markdown_to_html(&self.content);
        let file_path = format!("posts/{}.html", result.id);
        let mut file = File::create(file_path)
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalFsError))?;
        file.write_all(html.as_bytes())
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalFsError))?;

        // Keeping markdown copy for post editing
        let file_path = format!("posts/{}.md", result.id);
        let mut file = File::create(file_path)
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalFsError))?;
        file.write_all(&self.content.as_bytes())
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalFsError))?;
        Ok(())
    }

    pub async fn update(self, id: i32, pool: &PgPool) -> ApiResult {
        let query_username = "SELECT * FROM posts WHERE id=$1";
        let creator = sqlx::query_as::<_, Self>(query_username)
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalSqlxError))?;

        if self.creator != creator.creator {
            return Err(generic_error(ApiErrorCode::BlogUnauthorized));
        }

        let query = "UPDATE posts SET title=$1, description=$2 WHERE id=$3";
        let _ = sqlx::query(query)
            .bind(&self.title)
            .bind(&self.description)
            .bind(id)
            .execute(pool)
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalSqlxError))?;

        let file_path = format!("posts/{}.html", id);
        let file_path_new = format!("posts/{}.html.old", id);
        let _ = std::fs::rename(file_path, file_path_new);

        let file_path = format!("posts/{}.md", id);
        let file_path_new = format!("posts/{}.md.old", id);
        let _ = std::fs::rename(file_path, file_path_new);

        // Parsing to html
        let html = markdown_to_html(&self.content);
        let file_path = format!("posts/{}.html", id);
        let mut file = File::create(file_path)
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalFsError))?;
        file.write_all(html.as_bytes())
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalFsError))?;

        // Keeping markdown copy for post editing
        let file_path = format!("posts/{}.md", id);
        let mut file = File::create(file_path)
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalFsError))?;
        file.write_all(&self.content.as_bytes())
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalFsError))?;
        Ok(())
    }

    pub async fn get(id: i32, pool: &PgPool) -> Result<Post, ApiError> {
        let query = "SELECT * FROM posts WHERE id = $1";
        let mut result = sqlx::query_as::<_, Post>(query)
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|_| ApiError {
                message: "The requested post couldn't be found.".into(),
                status_code: StatusCode::NOT_FOUND,
                error_code: ApiErrorCode::BlogNotFound,
            })?;
        let file_path = format!("posts/{}.html", id);
        let content_res = tokio::fs::read_to_string(file_path).await;
        let content = match content_res {
            Ok(x) => x,
            Err(_) => {
                Post::delete(id, &pool, None).await?;
                return Err(ApiError {
                    message: "The file for the post couldn't be found.".into(),
                    status_code: StatusCode::NOT_FOUND,
                    error_code: ApiErrorCode::BlogNotFound,
                });
            }
        };
        result.content = content;
        Ok(result)
    }

    pub async fn get_md(id: i32, pool: &PgPool) -> Result<Post, ApiError> {
        let query = "SELECT * FROM posts WHERE id = $1";
        let mut result = sqlx::query_as::<_, Post>(query)
            .bind(id)
            .fetch_one(pool)
            .await
            .map_err(|_| ApiError {
                message: "The requested post couldn't be found.".into(),
                status_code: StatusCode::NOT_FOUND,
                error_code: ApiErrorCode::BlogNotFound,
            })?;
        let file_path = format!("posts/{}.md", id);
        let content_res = tokio::fs::read_to_string(file_path).await;
        let content = match content_res {
            Ok(x) => x,
            Err(_) => {
                Post::delete(id, &pool, None).await?;
                return Err(ApiError {
                    message: "The file for the post couldn't be found.".into(),
                    status_code: StatusCode::NOT_FOUND,
                    error_code: ApiErrorCode::BlogNotFound,
                });
            }
        };
        result.content = content;
        Ok(result)
    }

    pub async fn get_latest(amount: i32, pool: &PgPool) -> Result<Vec<Post>, ApiError> {
        let query = "SELECT * FROM posts ORDER BY creation DESC FETCH FIRST $1 ROWS ONLY";
        let data = sqlx::query_as::<_, Post>(query)
            .bind(amount)
            .fetch_all(pool)
            .await
            .map_err(|_| generic_error(ApiErrorCode::InternalSqlxError))?;
        Ok(data)
    }

    pub async fn delete(id: i32, pool: &PgPool, constraint: Option<String>) -> ApiResult {
        match constraint {
            Some(usr) => {
                let query = "DELETE FROM posts WHERE id = $1 AND creator = $2;";
                let _ = sqlx::query(query)
                    .bind(id)
                    .bind(usr)
                    .execute(pool)
                    .await
                    .map_err(|_| ApiError {
                        message: "You are not the creator of this post!".into(),
                        status_code: StatusCode::UNAUTHORIZED,
                        error_code: ApiErrorCode::BlogUnauthorized,
                    })?;
            }
            None => {
                let query = "DELETE FROM posts WHERE id = $1;";
                let _ = sqlx::query(query)
                    .bind(id)
                    .execute(pool)
                    .await
                    .map_err(|_| generic_error(ApiErrorCode::InternalSqlxError))?;
            }
        }
        let file_path = format!("posts/{}.html", id);
        let file = File::open(file_path.clone()).await;
        match file {
            Err(_) => return Ok(()),
            Ok(mut x) => {
                let _ = x.shutdown();
                let _ = remove_file(file_path).await;
            }
        }
        let file_path = format!("posts/{}.md", id);
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

impl From<PostData> for Post {
    fn from(it: PostData) -> Post {
        Post {
            id: 0,
            title: it.title,
            description: it.description,
            content: it.content,
            creator: "".into(),
            likes: 0,
            creation: Utc::now(),
        }
    }
}
