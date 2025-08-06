use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use tokio::sync::RwLock;

use crate::{config::Config, devices::TapoDevice};

use super::{loader::load_tapo_devices_from_config, sessions::Sessions};

pub struct StateData {
    pub config_path: PathBuf,
    pub loaded_config: RwLock<LoadedConfig>,
    pub sessions: Sessions,
}

impl StateData {
    pub async fn init(config_path: PathBuf, sessions_file: PathBuf) -> Result<Self> {
        let (config, devices) = load_tapo_devices_from_config(&config_path).await?;

        Ok(Self {
            config_path,
            loaded_config: RwLock::new(LoadedConfig::new(config, devices)),
            sessions: Sessions::create(sessions_file).await?,
        })
    }

    pub async fn reload_config(&self) -> Result<()> {
        let (config, devices) = load_tapo_devices_from_config(&self.config_path).await?;

        *self.loaded_config.write().await = LoadedConfig::new(config, devices);

        Ok(())
    }
}

pub struct LoadedConfig {
    pub config: Config,
    pub devices: HashMap<String, TapoDevice>,
}

impl LoadedConfig {
    pub fn new(config: Config, devices: Vec<TapoDevice>) -> Self {
        Self {
            config,
            devices: devices
                .into_iter()
                .map(|device| (device.conn_infos().name.to_owned(), device))
                .collect(),
        }
    }
}
