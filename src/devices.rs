use anyhow::{anyhow, Result};
use tapo::{
    ApiClient, ColorLightHandler, ColorLightStripHandler, LightHandler,
    PlugEnergyMonitoringHandler, PlugHandler,
};

use crate::{
    config::{TapoConnectionInfos, TapoCredentials},
    server::TapoDeviceType,
};

pub struct TapoDevice {
    pub name: String,
    // pub conn_infos: TapoConnectionInfos,
    pub inner: TapoDeviceInner,
}

pub enum TapoDeviceInner {
    L510(LightHandler),
    L530(ColorLightHandler),
    L610(LightHandler),
    L630(ColorLightHandler),
    L900(ColorLightHandler),
    L920(ColorLightStripHandler),
    L930(ColorLightStripHandler),
    P100(PlugHandler),
    P105(PlugHandler),
    P110(PlugEnergyMonitoringHandler),
    P115(PlugEnergyMonitoringHandler),
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

        let tapo_client = ApiClient::new(username, password);

        let inner =
            match device_type {
                TapoDeviceType::L510 => {
                    let auth = tapo_client.l510(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into L510 bulb '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::L510(auth)
                }

                TapoDeviceType::L530 => {
                    let auth = tapo_client.l530(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into L530 bulb '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::L530(auth)
                }

                TapoDeviceType::L610 => {
                    let auth = tapo_client.l610(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into L610 bulb '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::L610(auth)
                }

                TapoDeviceType::L630 => {
                    let auth = tapo_client.l630(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into L630 bulb '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::L630(auth)
                }

                TapoDeviceType::L900 => {
                    let auth = tapo_client.l900(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into L900 bulb '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::L900(auth)
                }

                TapoDeviceType::L920 => {
                    let auth = tapo_client.l920(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into L920 strip '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::L920(auth)
                }

                TapoDeviceType::L930 => {
                    let auth = tapo_client.l930(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into L930 strip '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::L930(auth)
                }

                TapoDeviceType::P100 => {
                    let auth = tapo_client.p100(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into P100 plug '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::P100(auth)
                }

                TapoDeviceType::P105 => {
                    let auth = tapo_client.p105(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into P105 plug '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::P105(auth)
                }

                TapoDeviceType::P110 => {
                    let auth = tapo_client.p110(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into P110 plug '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::P110(auth)
                }

                TapoDeviceType::P115 => {
                    let auth = tapo_client.p115(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into P115 plug '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::P115(auth)
                }
            };

        Ok(Self {
            name: name.to_string(),
            // conn_infos,
            inner,
        })
    }
}
