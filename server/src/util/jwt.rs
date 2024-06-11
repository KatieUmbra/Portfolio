use std::fmt::Display;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::{database::schema::LoginData, AppState};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub iat: usize,
    pub exp: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username: {}", self.username)
    }
}

impl Claims {
    pub fn new(user: &LoginData) -> Claims {
        Claims {
            username: user.username.clone(),
            iat: chrono::Utc::now().timestamp() as usize,
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    AppState: FromRef<S>,
    S: Send + Sync + std::fmt::Debug,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = parts
            .extract_with_state::<AppState, _>(state)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let decoding_key = DecodingKey::from_secret(&state.settings.jwt_secret.as_bytes());

        let token_data = decode::<Claims>(bearer.token(), &decoding_key, &Validation::default())
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(token_data.claims)
    }
}
