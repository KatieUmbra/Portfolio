use crate::{
    database::schema::{LoginData, UserData},
    util::{
        jwt::Claims,
        password::{hash_pwd, verify_pwd},
    },
    AppState,
};
use axum::{debug_handler, extract::State, http::StatusCode, Json};
use jsonwebtoken::{encode, EncodingKey, Header};

#[debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(mut user): Json<UserData>,
) -> Result<(), StatusCode> {
    hash_pwd(&mut user)?;
    user.insert(&state.pool).await?;

    Ok(())
}

pub async fn login(
    State(state): State<AppState>,
    Json(mut user): Json<LoginData>,
) -> Result<String, StatusCode> {
    let data = user.select(&state.pool).await?;
    let _ = verify_pwd(&data.password, &mut user)?;
    let encoding_key = EncodingKey::from_secret(&state.settings.jwt_secret.as_bytes());
    let claims = Claims::new(&data);
    let token = encode(&Header::default(), &claims, &encoding_key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(token)
}

pub async fn info(claims: Claims) -> Result<String, StatusCode> {
    Ok(format!(
        "Welcome to the protected area :D\nYour data:\n{claims}",
    ))
}
