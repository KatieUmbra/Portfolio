use std::env;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppSettings {
    pub host: String,
    pub port: String,
    pub environment: String,
    pub db_url: String,
    pub jwt_secret: String,
}

impl AppSettings {
    pub fn new() -> Result<AppSettings, anyhow::Error> {
        dotenv().ok();
        Ok(AppSettings {
            host: env::var("APP_HOST")?.to_string(),
            port: env::var("APP_PORT")?.to_string(),
            environment: env::var("APP_ENVIRONMENT")?.to_string(),
            db_url: env::var("DATABASE_URL")?.to_string(),
            jwt_secret: env::var("JWT_SECRET")?.to_string(),
        })
    }
}
