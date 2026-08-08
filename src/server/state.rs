use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use tokio::sync::RwLock;

use crate::{config::Config, devices::TapoDevice};

use super::loader::load_tapo_devices_from_config;

pub struct StateData {
    pub config_path: PathBuf,
    pub config: RwLock<Config>,
    pub devices: RwLock<HashMap<String, TapoDevice>>,
}

impl StateData {
    pub async fn init(config_path: PathBuf) -> Result<Self> {
        let (config, devices) = load_tapo_devices_from_config(&config_path).await?;

        Ok(Self {
            config_path,
            config: RwLock::new(config),
            devices: RwLock::new(
                devices
                    .into_iter()
                    .map(|device| (device.conn_infos().name.clone(), device))
                    .collect(),
            ),
        })
    }

    pub async fn reload_config(&self) -> Result<()> {
        let (config, devices) = load_tapo_devices_from_config(&self.config_path).await?;

        // Prevent TOCTOU
        let mut config_lock = self.config.write().await;
        let mut devices_lock = self.devices.write().await;

        *config_lock = config;
        *devices_lock = devices
            .into_iter()
            .map(|device| (device.conn_infos().name.clone(), device))
            .collect();

        Ok(())
    }
}
