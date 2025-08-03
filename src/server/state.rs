use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;

use crate::devices::TapoDevice;

use super::sessions::Sessions;

pub struct StateData {
    pub auth_password: String,
    pub devices: HashMap<String, TapoDevice>,
    pub sessions: Sessions,
}

impl StateData {
    pub async fn init(
        auth_password: String,
        devices: Vec<TapoDevice>,
        sessions_file: PathBuf,
    ) -> Result<Self> {
        Ok(Self {
            auth_password,
            devices: devices
                .into_iter()
                .map(|device| (device.conn_infos().name.to_owned(), device))
                .collect(),
            sessions: Sessions::create(sessions_file).await?,
        })
    }
}
