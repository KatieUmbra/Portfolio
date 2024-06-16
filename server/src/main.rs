pub mod database;
pub mod routing;
pub mod util;

use crate::routing::routes::user::{info, login, register, req_email_verify};
use crate::util::setup::AppSettings;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    routing::{get, post},
    Router,
};
use database::schema::LoginData;
use dotenv::dotenv;
use sqlx::PgPool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
    pub settings: AppSettings,
}

#[async_trait]
impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync + core::fmt::Debug,
{
    type Rejection = StatusCode;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let settings = AppSettings::new()?;

    let _ = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init();

    let pool = sqlx::postgres::PgPool::connect(&settings.db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let state = AppState {
        pool,
        settings: settings.clone(),
    };
    let router = Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/info", get(info))
        .route("/reqEmailVerify", post(req_email_verify))
        .with_state(state);

    let bind_address = settings.host + ":" + &settings.port;
    let listener = tokio::net::TcpListener::bind(bind_address).await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
