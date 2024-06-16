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
use lettre::{
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
    Message, SmtpTransport, Transport,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::Serialize;
use sqlx::types::chrono;

#[derive(Serialize)]
pub struct Token {
    token: String,
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

pub async fn req_email_verify(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<(), ApiError> {
    let mut s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(60)
        .map(char::from)
        .collect::<String>();

    let res = hash_str(&mut s)?;

    let request = EmailRequest {
        username: claims.username.clone(),
        secret: res,
        operation: 0,
        expiration: chrono::Utc::now() + Duration::hours(24),
    };

    request.insert(&state.pool).await?;

    let user = UserData::select(claims.username, &state.pool).await?;

    let creds = Credentials::new(
        state.settings.smtp_username.clone(),
        state.settings.smtp_password.clone(),
    );

    let email = Message::builder()
        .from("no-reply@kaytea.dev".parse().unwrap())
        .to(user.email.parse().unwrap())
        .subject("Verify your password!")
        .body(String::from("Email :3"))
        .unwrap();

    let tls = TlsParameters::builder("kaytea.dev".to_owned())
        .dangerous_accept_invalid_certs(true)
        .build()
        .unwrap();

    let sender = SmtpTransport::relay("kaytea.dev")
        .unwrap()
        .tls(Tls::Required(tls))
        .build();

    let _result = sender.send(&email).map_err(|e| {
        tracing::error!("{:?}", e);
    });

    tracing::info!("POST /reqEmailVerify");

    Ok(())
}
