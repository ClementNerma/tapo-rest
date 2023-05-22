use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;

use crate::devices::TapoDevice;

use super::sessions::Sessions;

pub struct StateInit {
    pub auth_password: String,
    pub devices: Vec<TapoDevice>,
    pub sessions_file: PathBuf,
}

pub struct State {
    pub auth_password: String,
    pub devices: HashMap<String, TapoDevice>,
    pub sessions: Sessions,
}

impl State {
    pub async fn init(
        StateInit {
            auth_password,
            devices,
            sessions_file,
        }: StateInit,
    ) -> Result<Self> {
        Ok(Self {
            auth_password,
            devices: devices
                .into_iter()
                .map(|device| (device.name.clone(), device))
                .collect(),
            sessions: Sessions::create(sessions_file).await?,
        })
    }
}
