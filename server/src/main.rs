use anyhow::Ok;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    debug_handler,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;
use zeroize::Zeroize;

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

#[derive(Serialize, sqlx::FromRow, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

async fn register(State(state): State<AppState>, Json(mut user): Json<User>) {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hashed = argon2
        .hash_password(&user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    user.password.zeroize();

    let query =
        "INSERT INTO users (username, displayUsername, displayName, email, password, verified) VALUES ($1, $2, $3, $4, $5, $6)";
    let _ = sqlx::query(query)
        .bind(&user.username.to_lowercase())
        .bind(&user.username)
        .bind(&user.display_name)
        .bind(&user.email.to_lowercase())
        .bind(hashed)
        .bind(false)
        .execute(&state.pool)
        .await;
}

#[debug_handler]
async fn login(
    State(state): State<AppState>,
    Json(user): Json<Login>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let query = "SELECT username, password FROM users WHERE username=$1";
    let data = sqlx::query_as::<_, Login>(query)
        .bind(user.username)
        .fetch_one(&state.pool)
        .await;

    match data {
        Result::Ok(data) => {
            let argon2 = Argon2::default();
            let parsed_hash = PasswordHash::new(&data.password).unwrap();
            let result = argon2.verify_password(&user.password.as_bytes(), &parsed_hash);

            match result {
                Result::Ok(_) => {
                    let claims = Claims {
                        sub: data.username.clone(),
                        exp: (chrono::Utc::now() + chrono::Duration::weeks(12)).timestamp()
                            as usize,
                    };

                    let token = match encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret("secret".as_ref()),
                    ) {
                        Result::Ok(tok) => tok,
                        Err(e) => {
                            eprintln!("Error generating token {}", e);
                            return Err(StatusCode::INTERNAL_SERVER_ERROR);
                        }
                    };

                    Result::Ok(Json(LoginResponse { token }))
                }
                Err(_) => Err(StatusCode::UNAUTHORIZED),
            }
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn info_handler(header_map: HeaderMap) -> Result<Json<String>, StatusCode> {
    if let Some(auth_header) = header_map.get("Authorization") {
        if let Result::Ok(auth_header_str) = auth_header.to_str() {
            if auth_header_str.starts_with("Bearer ") {
                let token = auth_header_str.trim_start_matches("Bearer ").to_string();

                return match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret("secret".as_ref()),
                    &Validation::default(),
                ) {
                    Result::Ok(_) => {
                        let info = "You are valid here is info".to_string();
                        Result::Ok(Json(info))
                    }
                    Err(e) => {
                        eprintln!("Error Generating Token {}", e);
                        Err(StatusCode::UNAUTHORIZED)
                    }
                };
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = "postgres://Kathy@localhost/Portfolio";
    let pool = sqlx::postgres::PgPool::connect(db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState { pool };

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let router = Router::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/info", get(info_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
