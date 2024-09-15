use crate::{
    database::schema::{email_request::EmailRequest, login_data::LoginData, user_data::UserData},
    routing::routes::structs::EmailToken,
    util::{
        error::{ApiError, ApiErrorCode},
        jwt::Claims,
        password::{hash_str, verify_str},
        state::AppState,
    },
};
use ::chrono::Duration;
use axum::{debug_handler, extract::State, http::StatusCode, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::{Message, Transport};
use rand::{distributions::Alphanumeric, Rng};
use sqlx::types::chrono;

use super::structs::Token;

/// `POST /register` that accepts a json version of [UserData] and registers said user into the
/// database
pub async fn register(
    State(state): State<AppState>,
    Json(mut user): Json<UserData>,
) -> Result<(), ApiError> {
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
    let _ = verify_str(&data.password, &mut user).map_err(|_| ApiError {
        message: "The provided password is incorrect, try again or reset your password".into(),
        error_code: ApiErrorCode::LoginWrongPassword,
        status_code: StatusCode::UNAUTHORIZED,
    });
    let encoding_key = EncodingKey::from_secret(&state.settings.jwt_secret.as_bytes());
    let claims = Claims::new(&data, 2);
    let token = encode(&Header::default(), &claims, &encoding_key).map_err(|_| ApiError {
        message: "An internal error has occurred, please contact support".into(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: ApiErrorCode::InternalErrorContactSupport,
    })?;
    tracing::info!("POST /login {}", user.username);

    Ok(axum::Json(Token { token }))
}

/// `GET /info` is a test function for protected routes
pub async fn info(claims: Claims) -> Result<String, ApiError> {
    tracing::info!("GET /info {}", claims.username);
    Ok(format!(
        "Welcome to the protected area :D\nYour data:\n{claims}",
    ))
}

/// `PUT /verify` is a protected route that accepts an email token and updates a user's status to
/// verified
#[debug_handler]
pub async fn verify(
    claims: Claims,
    State(state): State<AppState>,
    Json(token): Json<EmailToken>,
) -> Result<(), ApiError> {
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

    tracing::info!("PUT /verify");

    Ok(())
}

/// `GET /reqEmailVerify` is a protected route that sends an email that allows the user to change
/// it's status to verified.
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
        id: 0,
    };
    request.insert(&state.pool).await?;

    let _result = state.email_sender.send(&email).map_err(|e| {
        tracing::error!("{:?}", e);
    });

    tracing::info!("GET /reqEmailVerify");

    Ok(())
}
