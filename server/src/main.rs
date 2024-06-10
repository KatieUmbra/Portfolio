pub mod database;
pub mod util;

use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use database::schema::{LoginData, UserData};
use jsonwebtoken::{encode, Header};
use serde::Serialize;
use sqlx::PgPool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use util::{
    jwt::{Claims, KEYS},
    password::{hash_pwd, verify_pwd},
};

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[debug_handler]
async fn register(
    State(state): State<AppState>,
    Json(mut user): Json<UserData>,
) -> Result<(), StatusCode> {
    hash_pwd(&mut user)?;
    user.insert(&state.pool).await?;

    Ok(())
}

#[derive(Serialize)]
pub struct Jwt {
    token: String,
}

async fn login(
    State(state): State<AppState>,
    Json(mut user): Json<LoginData>,
) -> Result<Json<Jwt>, StatusCode> {
    let data = user.select(&state.pool).await?;

    let _ = verify_pwd(&data.password, &mut user)?;

    let claims = Claims {
        username: data.username.clone(),
        iat: chrono::Utc::now().timestamp() as usize,
        exp: (chrono::Utc::now() + chrono::Duration::weeks(12)).timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Jwt { token }))
}

async fn info_handler(claims: Claims) -> Result<String, StatusCode> {
    Ok(format!(
        "Welcome to the protected area :D\nYour data:\n{claims}",
    ))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = "postgres://Kathy@localhost/Portfolio";
    let pool = sqlx::postgres::PgPool::connect(db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState { pool };

    let _ = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init();

    let router = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/info", get(info_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
