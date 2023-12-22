use axum::{extract::State, Json};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::Serialize;

use super::{auth::auth, ApiResult, SharedState, TapoDeviceType};

#[derive(Serialize)]
pub struct DiscoveredDevice {
    name: String,
    r#type: TapoDeviceType,
}

#[derive(Serialize)]
pub struct DiscoveryResult {
    devices: Vec<DiscoveredDevice>,
}

pub async fn discover_devices(
    auth_header: TypedHeader<Authorization<Bearer>>,
    State(state): State<SharedState>,
) -> ApiResult<Json<DiscoveryResult>> {
    let state = state.read().await;

    auth(auth_header, &state.sessions).await?;

    let devices = state
        .devices
        .values()
        .map(|device| DiscoveredDevice {
            name: device.name.clone(),
            r#type: TapoDeviceType::get_type(&device.inner),
        })
        .collect();

    Ok(Json(DiscoveryResult { devices }))
}
