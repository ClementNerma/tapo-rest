use std::{path::Path, sync::Arc};

use anyhow::{Context, Result};
use colored::Colorize;
use log::{error, info};
use tokio::{fs, task::JoinSet};

use crate::{config::Config, devices::TapoDevice};

pub async fn load_tapo_devices_from_config(
    config_path: &Path,
) -> Result<(Config, Vec<TapoDevice>)> {
    let config_str = fs::read_to_string(&config_path)
        .await
        .context("Failed to read the devices configuration file")?;

    let config = serde_json::from_str::<Config>(&config_str)
        .context("Failed to parse the devices configuration file")?;

    let devices = load_tapo_devices(&config).await?;

    Ok((config, devices))
}

async fn load_tapo_devices(config: &Config) -> Result<Vec<TapoDevice>> {
    let Config {
        devices,
        tapo_credentials,
        server_password: _,
    } = config;

    let mut tasks = JoinSet::new();

    info!(
        "Attempting to connect to the {} configured device(s)...",
        devices.len()
    );

    let tapo_credentials = Arc::new(tapo_credentials.clone());

    for conn_infos in devices {
        let tapo_credentials = Arc::clone(&tapo_credentials);
        let conn_infos = conn_infos.clone();

        tasks.spawn(async move {
            let device = TapoDevice::new(conn_infos, tapo_credentials);
            let conn_result = device.try_connect().await;
            (device, conn_result)
        });
    }

    let mut devices = vec![];

    while let Some(result) = tasks.join_next().await {
        let (device, conn_result) = result?;
        let name = &device.conn_infos().name;

        match conn_result {
            Ok(()) => info!("|> Device {} connected successfully!", name.bright_yellow()),

            Err(err) => error!("! Failed to connect to device '{name}': {err}",),
        }

        devices.push(device);
    }

    Ok(devices)
}
