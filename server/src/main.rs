use anyhow::Ok;
use axum::extract::State;
use axum::Router;
use axum::{routing::post, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
}

async fn register(State(state): State<AppState>, Json(user): Json<User>) {
    let query =
        "INSERT INTO users (username, displayName, email, password) VALUES ($1, $2, $3, $4)";

    let _ = sqlx::query(query)
        .bind(&user.username)
        .bind(&user.display_name)
        .bind(&user.email)
        .bind(&user.password)
        .execute(&state.pool)
        .await;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = "postgres://Kathy@localhost/Portfolio";
    let pool = sqlx::postgres::PgPool::connect(db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState { pool };

    let router = Router::new()
        .route("/register", post(register))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
