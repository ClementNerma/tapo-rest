use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use tokio::{fs, sync::RwLock};

use crate::{config::Config, devices::TapoDevice};

use super::loader::load_tapo_devices;

pub struct StateData {
    pub config_path: PathBuf,
    pub config: RwLock<Config>,
    pub devices: RwLock<HashMap<String, TapoDevice>>,
}

impl StateData {
    pub async fn init(config_path: PathBuf) -> Result<Self> {
        let (config, devices) = load_config(&config_path).await?;

        Ok(Self {
            config_path,
            config: RwLock::new(config),
            devices: RwLock::new(devices),
        })
    }

    pub async fn reload_config(&self) -> Result<()> {
        let (config, devices) = load_config(&self.config_path).await?;

        *self.config.write().await = config;
        *self.devices.write().await = devices;

        Ok(())
    }
}

async fn load_config(config_path: &PathBuf) -> Result<(Config, HashMap<String, TapoDevice>)> {
    let config_str = fs::read_to_string(config_path)
        .await
        .context("Failed to read configuration file")?;

    let config = serde_json::from_str::<Config>(&config_str)
        .context("Failed to parse the devices configuration file")?;

    let devices = load_tapo_devices(&config)
        .await
        .context("Failed to load Tapo devices from configuration")?;

    let devices = devices
        .into_iter()
        .map(|device| (device.conn_infos().name.clone(), device))
        .collect();

    Ok((config, devices))
}
