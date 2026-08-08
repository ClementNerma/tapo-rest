use std::{sync::Arc, time::SystemTime};

use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use log::{error, info};
use serde::Deserialize;

use crate::server::SharedState;

use super::{ApiError, ApiResult, sessions::Session, state::StateData};

#[derive(Deserialize)]
pub struct LoginData {
    password: String,
}

// TODO: fail2ban? rate limiting?
pub async fn login(
    State(state): State<SharedState>,
    Json(LoginData { password }): Json<LoginData>,
) -> ApiResult<String> {
    let loaded_config = state.loaded_config.read().await;

    if password != loaded_config.config.server.password {
        error!("ERROR: Invalid credentials provided");

        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "Invalid credentials provided",
        ));
    }

    info!("INFO: User logged in successfully");

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

    let Session {
        created_at: _,
        expires_at,
    } = state
        .sessions
        .get(session_id)
        .await
        .ok_or(ApiError::new(StatusCode::FORBIDDEN, "Invalid bearer token"))?;

    if SystemTime::now() > expires_at {
        error!("ERROR: Bearer token expired");
        return Err(ApiError::new(StatusCode::FORBIDDEN, "Bearer token expired"));
    }

    Ok(next.run(request).await)
}
