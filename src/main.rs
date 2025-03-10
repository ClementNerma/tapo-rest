#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
#![warn(unused_crate_dependencies)]

use std::sync::Arc;

use anyhow::{bail, Context, Result};
use clap::Parser;
use config::Config;
use devices::TapoDevice;
use tokio::{fs, task::JoinSet};

use crate::cmd::Cmd;

mod cmd;
mod config;
mod devices;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let Cmd {
        devices_config_path,
        tapo_credentials,
        server_config,
    } = Cmd::parse();

    let data_dir = dirs::data_local_dir()
        .context("Failed to find a valid local data directory")?
        .join(env!("CARGO_PKG_NAME"));

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .await
            .context("Failed to create a local data directory")?;
    }

    if !devices_config_path.is_file() {
        bail!(
            "Configuration was not found at path {}",
            devices_config_path.to_string_lossy()
        );
    }

    let config_str = fs::read_to_string(&devices_config_path)
        .await
        .context("Failed to read the devices configuration file")?;

    let Config { devices } = serde_json::from_str(&config_str)
        .context("Failed to parse the devices configuration file")?;

    let mut tasks = JoinSet::new();

    println!(
        "| Attempting to connect to the {} configured device(s)...",
        devices.len()
    );

    let mut remaining = devices.len();

    let tapo_credentials = Arc::new(tapo_credentials);

    for conn_infos in devices {
        let tapo_credentials = Arc::clone(&tapo_credentials);
        tasks.spawn(async move { TapoDevice::connect(conn_infos, &tapo_credentials).await });
    }

    let mut devices = vec![];

    while let Some(result) = tasks.join_next().await {
        devices.push(result??);

        remaining -= 1;

        if remaining > 0 {
            println!("| {} remaining...", remaining);
        }
    }

    println!("| Successfully connected to all devices!");

    server::serve(server_config, devices, data_dir.join("sessions.json")).await
}
