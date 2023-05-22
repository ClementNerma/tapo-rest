use axum::{
    extract::{Query, State},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    TypedHeader,
};
use serde::Deserialize;
use tapo::requests::Color;

use crate::devices::{TapoDevice, TapoDeviceInner};

use super::{ApiResult, SharedState, SharedStateInner};

#[derive(Deserialize)]
pub struct DeviceNameQuery {
    device: String,
}

pub async fn on(
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    Query(device_name): Query<DeviceNameQuery>,
    State(state): State<SharedState>,
) -> ApiResult<&'static str> {
    let state = state.read().await;

    let device = get_device(auth_header, &device_name, &state).await?;

    match &device.inner {
        TapoDeviceInner::L530(client) => client
            .on()
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")))?,
    }

    Ok("OK")
}

pub async fn off(
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    Query(device_name): Query<DeviceNameQuery>,
    State(state): State<SharedState>,
) -> ApiResult<&'static str> {
    let state = state.read().await;

    let device = get_device(auth_header, &device_name, &state).await?;

    match &device.inner {
        TapoDeviceInner::L530(client) => client
            .off()
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")))?,
    }

    Ok("OK")
}

#[derive(Deserialize)]
pub struct SetBrightnessParams {
    level: u8,
}

pub async fn set_brightness(
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    Query(device_name): Query<DeviceNameQuery>,
    State(state): State<SharedState>,
    Query(SetBrightnessParams { level }): Query<SetBrightnessParams>,
) -> ApiResult<&'static str> {
    let state = state.read().await;

    let device = get_device(auth_header, &device_name, &state).await?;

    match &device.inner {
        TapoDeviceInner::L530(client) => client
            .set_brightness(level)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")))?,
    }

    Ok("OK")
}

#[derive(Deserialize)]
pub struct SetColorParams {
    name: Color,
}

pub async fn set_color(
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    Query(device_name): Query<DeviceNameQuery>,
    State(state): State<SharedState>,
    Query(SetColorParams { name }): Query<SetColorParams>,
) -> ApiResult<&'static str> {
    let state = state.read().await;

    let device = get_device(auth_header, &device_name, &state).await?;

    match &device.inner {
        TapoDeviceInner::L530(client) => client
            .set_color(name)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")))?,
    }

    Ok("OK")
}

async fn get_device<'s>(
    auth_header: Authorization<Bearer>,
    device_name: &DeviceNameQuery,
    state: &'s SharedStateInner,
) -> ApiResult<&'s TapoDevice> {
    let session_id = auth_header.0.token();

    let _ = state
        .sessions
        .get(session_id)
        .ok_or((StatusCode::FORBIDDEN, "Invalid bearer token".to_string()))?;

    // TODO: session expiration, etc.?

    let device = state.devices.get(&device_name.device).ok_or((
        StatusCode::NOT_FOUND,
        "Provided device name was not found".to_string(),
    ))?;

    Ok(device)
}
