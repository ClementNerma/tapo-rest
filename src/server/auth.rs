use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::server::SharedState;

use super::ApiResult;

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
        return Err((
            StatusCode::FORBIDDEN,
            "Invalid credentials provided".to_string(),
        ));
    }

    let session_id = state
        .sessions
        .insert()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")))?;

    Ok(session_id)
}
