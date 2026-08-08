use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use log::error;

use super::{ApiError, state::StateData};

// TODO: fail2ban? rate limiting?
pub async fn auth_middleware(
    State(state): State<Arc<StateData>>,
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let api_key = auth_header.0.token();

    let config = state.config.read().await;

    if !config
        .server
        .api_keys
        .iter()
        .any(|api_key_entry| api_key_entry.key == api_key)
    {
        error!("Provided invalid API key (bearer token): {api_key}");
        return Err(ApiError::new(StatusCode::FORBIDDEN, "Invalid bearer token"));
    }

    Ok(next.run(request).await)
}
