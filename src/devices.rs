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
            // Call '.refresh_session()' on the device client
            macro_rules! refresh_session {
                ($conn: expr => $($enum_variant: ident),+) => {{
                    match $conn {
                        $(TapoDeviceInner::$enum_variant(device) => {
                            device.refresh_session().await?;
                        })+
                    }
                }}
            }

            refresh_session!(conn =>
                    L510, L520, L530, L535,
                    L610, L630,
                    L900, L920, L930,
                    P100, P105, P110, P110M, P115,
                    P300, P304, P304M, P316
            );

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

        let ip_addr = ip_addr.to_string();

        let conn = match device_type {
            TapoDeviceType::L510 => tapo_client.l510(ip_addr).await.map(TapoDeviceInner::L510),
            TapoDeviceType::L520 => tapo_client.l520(ip_addr).await.map(TapoDeviceInner::L520),
            TapoDeviceType::L530 => tapo_client.l530(ip_addr).await.map(TapoDeviceInner::L530),
            TapoDeviceType::L535 => tapo_client.l535(ip_addr).await.map(TapoDeviceInner::L535),
            TapoDeviceType::L610 => tapo_client.l610(ip_addr).await.map(TapoDeviceInner::L610),
            TapoDeviceType::L630 => tapo_client.l630(ip_addr).await.map(TapoDeviceInner::L630),
            TapoDeviceType::L900 => tapo_client.l900(ip_addr).await.map(TapoDeviceInner::L900),
            TapoDeviceType::L920 => tapo_client.l920(ip_addr).await.map(TapoDeviceInner::L920),
            TapoDeviceType::L930 => tapo_client.l930(ip_addr).await.map(TapoDeviceInner::L930),
            TapoDeviceType::P100 => tapo_client.p100(ip_addr).await.map(TapoDeviceInner::P100),
            TapoDeviceType::P105 => tapo_client.p105(ip_addr).await.map(TapoDeviceInner::P105),
            TapoDeviceType::P110 => tapo_client.p110(ip_addr).await.map(TapoDeviceInner::P110),
            TapoDeviceType::P110M => tapo_client.p110(ip_addr).await.map(TapoDeviceInner::P110M),
            TapoDeviceType::P115 => tapo_client.p115(ip_addr).await.map(TapoDeviceInner::P115),
            TapoDeviceType::P300 => tapo_client.p300(ip_addr).await.map(TapoDeviceInner::P300),
            TapoDeviceType::P304 => tapo_client.p304(ip_addr).await.map(TapoDeviceInner::P304),
            TapoDeviceType::P304M => tapo_client.p304(ip_addr).await.map(TapoDeviceInner::P304M),
            TapoDeviceType::P316 => tapo_client.p316(ip_addr).await.map(TapoDeviceInner::P316),
        };

        conn.map_err(|err| {
            anyhow!(
                "Failed to connect to {} {} '{name}': {err}",
                device_type.type_name(),
                device_type.type_description()
            )
        })
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
    P304M(PowerStripHandler),
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
            TapoDeviceInner::P304M(_) => "P304M",
            TapoDeviceInner::P316(_) => "P316",
        }
    }
}
