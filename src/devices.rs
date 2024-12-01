use anyhow::{anyhow, Result};
use tapo::{
    ApiClient, ColorLightHandler, LightHandler, PlugEnergyMonitoringHandler, PlugHandler,
    RgbLightStripHandler, RgbicLightStripHandler,
};

use crate::{cmd::TapoCredentials, config::TapoConnectionInfos, server::TapoDeviceType};

pub struct TapoDevice {
    pub name: String,
    pub inner: TapoDeviceInner,
}

pub enum TapoDeviceInner {
    L510(LightHandler),
    L520(LightHandler),
    L530(ColorLightHandler),
    L535(ColorLightHandler),
    L610(LightHandler),
    L630(ColorLightHandler),
    L900(RgbLightStripHandler),
    L920(RgbicLightStripHandler),
    L930(RgbicLightStripHandler),
    P100(PlugHandler),
    P105(PlugHandler),
    P110(PlugEnergyMonitoringHandler),
    P110M(PlugEnergyMonitoringHandler),
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

        let TapoCredentials {
            tapo_email,
            tapo_password,
        } = credentials;

        let tapo_client = ApiClient::new(tapo_email, tapo_password);

        let inner =
            match device_type {
                TapoDeviceType::L510 => {
                    let auth = tapo_client.l510(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into L510 bulb '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::L510(auth)
                }

                TapoDeviceType::L520 => {
                    let auth = tapo_client.l520(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into L520 bulb '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::L520(auth)
                }

                TapoDeviceType::L530 => {
                    let auth = tapo_client.l530(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into L530 bulb '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::L530(auth)
                }

                TapoDeviceType::L535 => {
                    let auth = tapo_client.l535(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into L535 bulb '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::L535(auth)
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

                TapoDeviceType::P110M => {
                    // P110M is fully compatible with P110
                    let auth = tapo_client.p110(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login into P110M plug '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::P110M(auth)
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

    pub async fn refresh_session(&mut self) -> Result<()> {
        match &mut self.inner {
            TapoDeviceInner::L510(device) => {
                device.refresh_session().await?;
            }

            TapoDeviceInner::L520(device) => {
                device.refresh_session().await?;
            }

            TapoDeviceInner::L530(device) => {
                device.refresh_session().await?;
            }

            TapoDeviceInner::L535(device) => {
                device.refresh_session().await?;
            }

            TapoDeviceInner::L610(device) => {
                device.refresh_session().await?;
            }

            TapoDeviceInner::L630(device) => {
                device.refresh_session().await?;
            }

            TapoDeviceInner::L900(device) => {
                device.refresh_session().await?;
            }

            TapoDeviceInner::L920(device) => {
                device.refresh_session().await?;
            }

            TapoDeviceInner::L930(device) => {
                device.refresh_session().await?;
            }

            TapoDeviceInner::P100(device) => {
                device.refresh_session().await?;
            }

            TapoDeviceInner::P105(device) => {
                device.refresh_session().await?;
            }

            TapoDeviceInner::P110(device) => {
                device.refresh_session().await?;
            }

            TapoDeviceInner::P110M(device) => {
                device.refresh_session().await?;
            }

            TapoDeviceInner::P115(device) => {
                device.refresh_session().await?;
            }
        }

        Ok(())
    }
}
