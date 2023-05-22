use axum::{extract::State, http::StatusCode, Json};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;

use crate::server::{state::Session, SharedState};

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

    let mut rng = rand::thread_rng();

    let session_id = (1..128)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect::<String>();

    state.sessions.insert(session_id.clone(), Session {});

    Ok(session_id)
}
