use crate::{
    database::schema::{email_request::EmailRequest, login_data::LoginData, user_data::UserData},
    routing::routes::structs::EmailToken,
    util::{
        error::{generic_error, ApiError, ApiErrorCode},
        jwt::Claims,
        password::{hash_str, verify_password_requirements, verify_str},
        state::AppState,
    },
};
use ::chrono::Duration;
use axum::{extract::State, http::StatusCode, Json};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lettre::{Message, Transport};
use rand::{distributions::Alphanumeric, Rng};
use sqlx::types::chrono;

use super::structs::{ApiResult, Token};

/// `POST /register` that accepts a json version of [UserData] and registers said user into the
/// database
pub async fn register(State(state): State<AppState>, Json(mut user): Json<UserData>) -> ApiResult {
    verify_password_requirements(&user.password)?;
    user.password = hash_str(&mut user.password)?;
    user.insert(&state.pool).await?;
    tracing::info!("POST /register {}", user.username);

    Ok(())
}

/// `POST /login` that accepts a json version of [LoginData] and returns a json web token of the
/// user if the password comparison is successful
pub async fn login(
    State(state): State<AppState>,
    Json(mut user): Json<LoginData>,
) -> Result<Json<Token>, ApiError> {
    let data = user.select(&state.pool).await?;
    verify_str(&data.password, &mut user)?;
    let encoding_key = EncodingKey::from_secret(&state.settings.jwt_secret.as_bytes());
    let claims = Claims::new(&data);
    let token = encode(&Header::default(), &claims, &encoding_key)
        .map_err(|_| generic_error(ApiErrorCode::InternalJwtError))?;
    tracing::info!("POST /login {}", user.username);

    if claims.rank == 2 {
        let _ = req_email_verify(claims.clone(), axum::extract::State(state)).await;
    }

    Ok(axum::Json(Token { token }))
}

/// `GET /info` is a test function for protected routes
pub async fn info(mut claims: Claims) -> Result<Json<Claims>, ApiError> {
    tracing::info!("GET /info {}", claims.username);
    claims.iat = 0;
    claims.exp = 0;
    Ok(Json(claims))
}

/// `PUT /verify` is a protected route that accepts an email token and updates a user's status to
/// verified
pub async fn verify(
    claims: Claims,
    State(state): State<AppState>,
    Json(token): Json<EmailToken>,
) -> Result<Json<Token>, ApiError> {
    tracing::info!("PUT /verify");
    let db_email_req_dummy = EmailRequest {
        username: claims.username.clone(),
        ..Default::default()
    };
    let db_email_request = db_email_req_dummy.select(&state.pool).await?;

    if token.veri_token != db_email_request.secret {
        return Err(ApiError {
            message: "The provided token is not valid or it expired".into(),
            error_code: ApiErrorCode::RegisterInvalidEmailToken,
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    let user = UserData::select(claims.username.clone(), &state.pool).await?;
    user.verify(&state.pool).await?;
    db_email_request.delete(&state.pool).await;

    let jwt = update_jwt(claims, axum::extract::State(state)).await?;

    Ok(jwt)
}

/// `GET /reqEmailVerify` is a protected route that sends an email that allows the user to change
/// it's status to verified.
pub async fn req_email_verify(claims: Claims, State(state): State<AppState>) -> ApiResult {
    let secret: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(60)
        .map(char::from)
        .collect::<String>();

    let user = UserData::select(claims.username.clone(), &state.pool).await?;

    let email = Message::builder()
        .from("no-reply@kaytea.dev".parse().unwrap())
        .to(user.email.parse().unwrap())
        .subject("Verify your password!")
        .body(format!(
            "Verify your email address with the following link: http://localhost:45886/verify?token={}", &secret
        ))
        .unwrap();

    let request = EmailRequest {
        username: claims.username.clone(),
        secret,
        operation: 0,
        expiration: chrono::Utc::now() + Duration::hours(24),
        id: 0,
    };
    request.insert(&state.pool).await?;

    let _result = state.email_sender.send(&email).map_err(|e| {
        tracing::error!("{:?}", e);
    });

    tracing::info!("GET /reqEmailVerify");

    Ok(())
}

/// `GET /updateJwt` is a protected route that returns an updated version of a jwt
pub async fn update_jwt(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<Token>, ApiError> {
    let user = LoginData {
        username: claims.username,
        ..Default::default()
    };
    let data = user.select(&state.pool).await?;
    let encoding_key = EncodingKey::from_secret(&state.settings.jwt_secret.as_bytes());
    let claims = Claims::new(&data);
    let token = encode(&Header::default(), &claims, &encoding_key)
        .map_err(|_| generic_error(ApiErrorCode::InternalJwtError))?;
    tracing::info!("POST /update_jwt {}", user.username);
    Ok(axum::Json(Token { token }))
}

/// `POST /refreshJwt` is the unprotected version of GET /updateJwt that verifies and updates the
/// token
pub async fn refresh_jwt(
    State(state): State<AppState>,
    Json(token): Json<Token>,
) -> Result<Json<Token>, ApiError> {
    tracing::info!("POST /refreshJwt");
    let decoding_key = DecodingKey::from_secret(&state.settings.jwt_secret.as_bytes());
    let encoding_key = EncodingKey::from_secret(&state.settings.jwt_secret.as_bytes());

    let token_data = decode::<Claims>(&*token.token, &decoding_key, &Validation::default())
        .map_err(|_| ApiError {
            status_code: StatusCode::UNAUTHORIZED,
            error_code: ApiErrorCode::InternalJwtError,
            message: "You need to provide a valid token".into(),
        })?;

    let user = LoginData {
        username: token_data.claims.username,
        ..Default::default()
    };
    let data = user.select(&state.pool).await?;
    let new_jwt = Claims::new(&data);

    let new_token = encode(&Header::default(), &new_jwt, &encoding_key)
        .map_err(|_| generic_error(ApiErrorCode::InternalJwtError))?;

    Ok(Json(Token { token: new_token }))
}
