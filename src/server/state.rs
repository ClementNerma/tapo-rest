use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::{config::Config, devices::TapoDevice};

use super::{loader::load_tapo_devices_from_config, sessions::Sessions};

pub struct StateData {
    pub config: Config,
    pub devices: HashMap<String, TapoDevice>,
    pub sessions: Sessions,
}

impl StateData {
    pub async fn init(config_path: &Path, sessions_file: PathBuf) -> Result<Self> {
        let (config, devices) = load_tapo_devices_from_config(config_path).await?;

        Ok(Self {
            devices: devices
                .into_iter()
                .map(|device| (device.conn_infos().name.to_owned(), device))
                .collect(),
            config,
            sessions: Sessions::create(sessions_file).await?,
        })
    }
}
