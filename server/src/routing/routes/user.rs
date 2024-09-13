use crate::{
    database::schema::{EmailRequest, LoginData, UserData},
    util::{
        error::{ApiError, ApiErrorCode},
        jwt::Claims,
        password::{hash_str, verify_str},
    },
    AppState,
};
use ::chrono::Duration;
use axum::{debug_handler, extract::State, http::StatusCode, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::{Message, Transport};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Serialize)]
pub struct Token {
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct EmailToken {
    veri_token: String,
}

#[debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(mut user): Json<UserData>,
) -> Result<(), ApiError> {
    user.password = hash_str(&mut user.password)?;
    user.insert(&state.pool).await?;
    tracing::info!("POST /register {}", user.username);

    Ok(())
}

pub async fn login(
    State(state): State<AppState>,
    Json(mut user): Json<LoginData>,
) -> Result<Json<Token>, ApiError> {
    let data = user.select(&state.pool).await?;
    let _ = verify_str(&data.password, &mut user).map_err(|_| ApiError {
        message: "The provided password is incorrect, try again or reset your password".into(),
        error_code: ApiErrorCode::LoginWrongPassword,
        status_code: StatusCode::UNAUTHORIZED,
    });
    let encoding_key = EncodingKey::from_secret(&state.settings.jwt_secret.as_bytes());
    let claims = Claims::new(&data);
    let token = encode(&Header::default(), &claims, &encoding_key).map_err(|_| ApiError {
        message: "An internal error has occurred, please contact support".into(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: ApiErrorCode::InternalErrorContactSupport,
    })?;
    tracing::info!("POST /login {}", user.username);

    Ok(axum::Json(Token { token }))
}

pub async fn info(claims: Claims) -> Result<String, ApiError> {
    tracing::info!("GET /info {}", claims.username);
    Ok(format!(
        "Welcome to the protected area :D\nYour data:\n{claims}",
    ))
}

#[debug_handler]
pub async fn verify(
    claims: Claims,
    State(state): State<AppState>,
    Json(token): Json<EmailToken>,
) -> Result<(), ApiError> {
    tracing::debug!("Token: {:?}", &token.veri_token);
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

    let user = UserData::select(claims.username, &state.pool).await?;
    user.verify(&state.pool).await?;
    db_email_request.delete(&state.pool).await;

    tracing::info!("UPDATE /verify");

    Ok(())
}

pub async fn req_email_verify(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<(), ApiError> {
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
            "Verify your email address with the following link: http://192.168.1.20:45886/verify?token={}", &secret
        ))
        .unwrap();

    let request = EmailRequest {
        username: claims.username.clone(),
        secret,
        operation: 0,
        expiration: chrono::Utc::now() + Duration::hours(24),
    };
    request.insert(&state.pool).await?;

    let _result = state.email_sender.send(&email).map_err(|e| {
        tracing::error!("{:?}", e);
    });

    tracing::info!("POST /reqEmailVerify");

    Ok(())
}
