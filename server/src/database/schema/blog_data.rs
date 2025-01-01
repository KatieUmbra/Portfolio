use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow, Deserialize, Clone, Default)]
pub struct Post {
    creator: String,
    content: String,
    description: String,
    title: String,
    creation: DateTime<Utc>,
    likes: u32,
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
