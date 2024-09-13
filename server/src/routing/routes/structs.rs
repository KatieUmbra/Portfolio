use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Token {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct EmailToken {
    pub veri_token: String,
}
