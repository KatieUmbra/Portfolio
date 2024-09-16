use serde::{Deserialize, Serialize};

/// Utility struct that is used to extract a json web token from a request part
#[derive(Serialize, Debug)]
pub struct Token {
    pub token: String,
}

/// Utility struct that is used to extract an email token from a request part
#[derive(Serialize, Deserialize)]
pub struct EmailToken {
    pub veri_token: String,
}
