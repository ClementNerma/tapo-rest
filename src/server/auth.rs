use axum::{extract::State, http::StatusCode, Json};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::Deserialize;

use crate::server::SharedState;

use super::{
    sessions::{Session, Sessions},
    ApiError, ApiResult,
};

#[derive(Deserialize)]
pub struct LoginData {
    password: String,
}

// TODO: fail2ban? rate limiting?
pub async fn login(
    State(state): State<SharedState>,
    Json(LoginData { password }): Json<LoginData>,
) -> ApiResult<String> {
    if password != state.auth_password {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "Invalid credentials provided",
        ));
    }

    let session_id = state.sessions.insert().await?;

    Ok(session_id)
}

pub async fn auth(
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    sessions: &Sessions,
) -> ApiResult<Session> {
    let session_id = auth_header.0.token();

    sessions
        .get(session_id)
        .await
        .ok_or(ApiError::new(StatusCode::FORBIDDEN, "Invalid bearer token"))
}
