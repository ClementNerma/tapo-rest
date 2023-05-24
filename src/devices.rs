use anyhow::{Context, Result};
use tapo::{ApiClient, Authenticated, ColorLightHandler, LightHandler};

use crate::config::{TapoConnectionInfos, TapoCredentials, TapoDeviceType};

pub struct TapoDevice {
    pub name: String,
    pub conn_infos: TapoConnectionInfos,
    pub inner: TapoDeviceInner,
}

pub enum TapoDeviceInner {
    L510(LightHandler<Authenticated>),
    L530(ColorLightHandler<Authenticated>),
}

impl TapoDevice {
    pub async fn connect(
        conn_infos: TapoConnectionInfos,
        credentials: &TapoCredentials,
    ) -> Result<Self> {
        let TapoConnectionInfos {
            name,
            device_type,
            ip_addr,
        } = &conn_infos;

        let TapoCredentials { username, password } = credentials;

        let tapo_client = ApiClient::new(ip_addr.to_string(), username, password)
            .with_context(|| format!("Failed to connect to Tapo device '{name}'"))?;

        let inner =
            match device_type {
                TapoDeviceType::L510 => {
                    let auth =
                        tapo_client.l510().login().await.with_context(|| {
                            format!("Failed to login against L510 bulb '{name}'")
                        })?;

                    TapoDeviceInner::L510(auth)
                }

                TapoDeviceType::L530 => {
                    let auth =
                        tapo_client.l530().login().await.with_context(|| {
                            format!("Failed to login against L530 bulb '{name}'")
                        })?;

                    TapoDeviceInner::L530(auth)
                }
            };

        Ok(Self {
            name: name.to_string(),
            conn_infos,
            inner,
        })
    }
}
