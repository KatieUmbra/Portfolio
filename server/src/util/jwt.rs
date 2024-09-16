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

use crate::{database::schema::login_data::LoginData, util::state::AppState};

/// Struct containing the data that will become a json web token (JWT)
/// ## Notes
/// See also [Claims::from_request_parts]
#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Username of the user who owns the json web token
    pub username: String,
    /// Level of permission of the user who owns the json web token:
    /// - 0: Administrator of the website, is allowed to post and manage comments, plus the
    /// permissions below.
    /// - 1: Regular user, is allowed to comment and send contact forms, plus the permissions
    /// below.
    /// - 2: Unverified user, is allowed to like comments and posts.
    pub rank: i32,
    /// Creation date of the json web token in utc.
    pub iat: usize,
    /// Expiration date of the json web token in utc.
    pub exp: usize,
}

/// Implements display for claims
///
/// ## Example
/// ```
/// let jwt = Claims {
///     username: "User",
///     rank: 1,
///     iat: chrono::Utc::now().timestamp() as usize,
///     exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize
///     }
///
/// println!("Debug::claims: {}", jwt);
/// ```
impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Username: {}, Rank: {}, Creation: {}, Expiration: {}",
            self.username, self.rank, self.iat, self.exp
        )
    }
}

impl Claims {
    /// Constructor for the claims struct
    pub fn new(user: &LoginData) -> Claims {
        Claims {
            username: user.username.clone(),
            rank: user.verified.clone(),
            iat: chrono::Utc::now().timestamp() as usize,
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        }
    }
}

/// Security trait implementation for protecting routes.
///
/// The implementation of [FromRequestParts] tries to extract an authentication header
/// from the request, if it's not present then it will return [StatusCode::UNAUTHORIZED]
///
/// ## Example
/// ```
/// // the presence of the claims argument makes the route protected
/// async fn protected_route(claims: Claims) -> String {
///     "You accessed a protected route!"
/// }
/// ```
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
