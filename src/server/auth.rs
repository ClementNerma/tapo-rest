use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::server::SharedState;

use super::{ApiError, ApiResult};

#[derive(Deserialize)]
pub struct LoginData {
    password: String,
}

// TODO: fail2ban? rate limiting?
pub async fn login(
    State(state): State<SharedState>,
    Json(LoginData { password }): Json<LoginData>,
) -> ApiResult<String> {
    let mut state = state.write().await;

    if password != state.auth_password {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "Invalid credentials provided",
        ));
    }

    let session_id = state.sessions.insert().await?;

    Ok(session_id)
}
