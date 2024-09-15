use std::env;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

/// Stores the app settings from a .env file
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppSettings {
    /// Local ip address where the app will be running
    pub host: String,
    /// The port where the api will be listening to
    pub port: String,
    /// The environment for the app
    /// - Debug
    /// - Release
    pub environment: String,
    /// Postgres connection url
    pub db_url: String,
    /// Random assortment of bytes used for generating json web tokens
    pub jwt_secret: String,
    /// Smtp server username (for email)
    pub smtp_username: String,
    /// Smtp server password (for email)
    pub smtp_password: String,
}

impl AppSettings {
    /// Loads variables from the .env file into [AppSettings]
    pub fn new() -> Result<AppSettings, anyhow::Error> {
        dotenv().ok();
        Ok(AppSettings {
            host: env::var("APP_HOST")?.to_string(),
            port: env::var("APP_PORT")?.to_string(),
            environment: env::var("APP_ENVIRONMENT")?.to_string(),
            db_url: env::var("DATABASE_URL")?.to_string(),
            jwt_secret: env::var("JWT_SECRET")?.to_string(),
            smtp_username: env::var("SMTP_USERNAME")?.to_string(),
            smtp_password: env::var("SMTP_PASSWORD")?.to_string(),
        })
    }
}
