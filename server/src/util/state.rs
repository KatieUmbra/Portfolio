use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use dotenv::dotenv;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};
use sqlx::PgPool;

use super::settings::AppSettings;

/// Contains the global state of the app, accesible by all routes.
#[derive(Clone, Debug)]
pub struct AppState {
    /// The sqlx connection pool
    pub pool: PgPool,
    /// The lettre email sender object
    pub email_sender: SmtpTransport,
    /// The [AppSettings] settings state
    pub settings: AppSettings,
}

#[async_trait]
impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync + core::fmt::Debug,
{
    type Rejection = StatusCode;

    /// Extractor for [AppState] to be used inside route handler functions
    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}

impl AppState {
    /// Initializes the struct with all required fields
    /// Contains mostly boilerplate
    pub async fn init() -> anyhow::Result<AppState> {
        dotenv().ok();
        let settings = AppSettings::new()?;
        let pool = sqlx::postgres::PgPool::connect(&settings.db_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        let creds = Credentials::new(
            settings.smtp_username.clone(),
            settings.smtp_password.clone(),
        );

        let email_sender = SmtpTransport::relay("mail.smtp2go.com")
            .map_err(|e| {
                tracing::debug!("{:?}", e);
                e
            })?
            .credentials(creds)
            .build();
        Ok(AppState {
            pool,
            email_sender,
            settings: settings.clone(),
        })
    }
}
