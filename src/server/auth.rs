use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::Deserialize;

use crate::server::SharedState;

use super::{sessions::Session, state::StateData, ApiError, ApiResult};

#[derive(Deserialize)]
pub struct LoginData {
    password: String,
}

// TODO: fail2ban? rate limiting?
pub async fn login(
    State(state): State<SharedState>,
    Json(LoginData { password }): Json<LoginData>,
) -> ApiResult<String> {
    if password != state.loaded_config.read().await.config.server_password {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "Invalid credentials provided",
        ));
    }

    let session_id = state.sessions.insert().await?;

    Ok(session_id)
}

pub async fn auth_middleware(
    State(state): State<Arc<StateData>>,
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let session_id = auth_header.0.token();

    let Session {} = state
        .sessions
        .get(session_id)
        .await
        .ok_or(ApiError::new(StatusCode::FORBIDDEN, "Invalid bearer token"))?;

    Ok(next.run(request).await)
}
