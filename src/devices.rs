use anyhow::{Context, Result};
use tapo::{ApiClient, Authenticated, ColorLightHandler, ColorLightStripHandler, LightHandler};

use crate::{
    config::{TapoConnectionInfos, TapoCredentials},
    server::TapoDeviceType,
};

pub struct TapoDevice {
    pub name: String,
    pub conn_infos: TapoConnectionInfos,
    pub inner: TapoDeviceInner,
}

pub enum TapoDeviceInner {
    L510(LightHandler<Authenticated>),
    L530(ColorLightHandler<Authenticated>),
    L610(LightHandler<Authenticated>),
    L630(ColorLightHandler<Authenticated>),
    L900(ColorLightHandler<Authenticated>),
    L920(ColorLightStripHandler<Authenticated>),
    L930(ColorLightStripHandler<Authenticated>),
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

                TapoDeviceType::L610 => {
                    let auth =
                        tapo_client.l610().login().await.with_context(|| {
                            format!("Failed to login against L610 bulb '{name}'")
                        })?;

                    TapoDeviceInner::L610(auth)
                }

                TapoDeviceType::L630 => {
                    let auth =
                        tapo_client.l630().login().await.with_context(|| {
                            format!("Failed to login against L630 bulb '{name}'")
                        })?;

                    TapoDeviceInner::L630(auth)
                }

                TapoDeviceType::L900 => {
                    let auth =
                        tapo_client.l900().login().await.with_context(|| {
                            format!("Failed to login against L900 bulb '{name}'")
                        })?;

                    TapoDeviceInner::L900(auth)
                }

                TapoDeviceType::L920 => {
                    let auth =
                        tapo_client.l920().login().await.with_context(|| {
                            format!("Failed to login against L920 bulb '{name}'")
                        })?;

                    TapoDeviceInner::L920(auth)
                }

                TapoDeviceType::L930 => {
                    let auth =
                        tapo_client.l930().login().await.with_context(|| {
                            format!("Failed to login against L930 bulb '{name}'")
                        })?;

                    TapoDeviceInner::L930(auth)
                }
            };

        Ok(Self {
            name: name.to_string(),
            conn_infos,
            inner,
        })
    }
}
