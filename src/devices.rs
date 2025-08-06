use std::sync::Arc;

use anyhow::{Result, anyhow};
use log::debug;
use tapo::{
    ApiClient, ColorLightHandler, LightHandler, PlugEnergyMonitoringHandler, PlugHandler,
    PowerStripHandler, RgbLightStripHandler, RgbicLightStripHandler,
};
use tokio::sync::RwLock;

use crate::{
    config::{TapoConnectionInfos, TapoCredentials},
    server::TapoDeviceType,
};

pub struct TapoDevice {
    conn_infos: TapoConnectionInfos,
    credentials: Arc<TapoCredentials>,
    client: RwLock<Option<TapoDeviceInner>>,
}

impl TapoDevice {
    pub fn new(conn_infos: TapoConnectionInfos, credentials: Arc<TapoCredentials>) -> Self {
        Self {
            conn_infos,
            credentials,
            client: RwLock::new(None),
        }
    }

    pub fn conn_infos(&self) -> &TapoConnectionInfos {
        &self.conn_infos
    }

    // pub async fn is_connected(&self) -> bool {
    //     self.client.read().await.is_some()
    // }

    pub async fn try_connect(&self) -> Result<()> {
        self.with_client(async |_| {}).await
    }

    pub async fn with_client<T>(&self, func: impl AsyncFnOnce(&TapoDeviceInner) -> T) -> Result<T> {
        {
            if let Some(conn) = &*self.client.read().await {
                return Ok(func(conn).await);
            }
        }

        self.with_client_mut(async move |client| func(&*client).await)
            .await
    }

    pub async fn with_client_mut<T>(
        &self,
        func: impl AsyncFnOnce(&mut TapoDeviceInner) -> T,
    ) -> Result<T> {
        let mut conn_lock = self.client.write().await;

        if let Some(conn) = conn_lock.as_mut() {
            return Ok(func(conn).await);
        }

        let mut conn = self._establish_conn().await?;

        debug!(
            "Established a connection with device '{}'!",
            self.conn_infos.name
        );

        let out = func(&mut conn).await;
        *conn_lock = Some(conn);
        Ok(out)
    }

    pub async fn refresh_session(&self) -> Result<()> {
        self.with_client_mut(async |conn| -> Result<()> {
            match conn {
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

                        TapoDeviceInner::P300(device) => {device.refresh_session().await?;},

                        TapoDeviceInner::P304(device) => {device.refresh_session().await?;},

                        TapoDeviceInner::P316(device) => {device.refresh_session().await?;},
                            }

            Ok(())
        })
        .await?
    }

    async fn _establish_conn(&self) -> Result<TapoDeviceInner> {
        let TapoConnectionInfos {
            name,
            device_type,
            ip_addr,
        } = &self.conn_infos;

        let TapoCredentials { email, password } = &*self.credentials;

        let tapo_client = ApiClient::new(email, password);

        let conn =
            // TODO: deduplicate some code here
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

                TapoDeviceType::P300 => {
                    let auth = tapo_client.p300(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login in P300 power strip '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::P300(auth)
                }

                TapoDeviceType::P304 => {
                    let auth = tapo_client.p304(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login in P304 power strip '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::P304(auth)
                }

                TapoDeviceType::P316 => {
                    let auth = tapo_client.p316(ip_addr.to_string()).await.map_err(|err| {
                        anyhow!("Failed to login in P316 power strip '{name}': {err:?}")
                    })?;

                    TapoDeviceInner::P316(auth)
                }
            };

        Ok(conn)
    }
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
    P300(PowerStripHandler),
    P304(PowerStripHandler),
    P316(PowerStripHandler),
}

impl TapoDeviceInner {
    pub fn type_name(&self) -> &'static str {
        match self {
            TapoDeviceInner::L510(_) => "L510",
            TapoDeviceInner::L520(_) => "L520",
            TapoDeviceInner::L530(_) => "L530",
            TapoDeviceInner::L535(_) => "L535",
            TapoDeviceInner::L610(_) => "L610",
            TapoDeviceInner::L630(_) => "L630",
            TapoDeviceInner::L900(_) => "L900",
            TapoDeviceInner::L920(_) => "L920",
            TapoDeviceInner::L930(_) => "L930",
            TapoDeviceInner::P100(_) => "P100",
            TapoDeviceInner::P105(_) => "P105",
            TapoDeviceInner::P110(_) => "P110",
            TapoDeviceInner::P110M(_) => "P110M",
            TapoDeviceInner::P115(_) => "P115",
            TapoDeviceInner::P300(_) => "P300",
            TapoDeviceInner::P304(_) => "P304",
            TapoDeviceInner::P316(_) => "P316",
        }
    }
}
