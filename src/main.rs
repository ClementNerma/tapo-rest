#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
#![warn(unused_crate_dependencies)]
// Use logging instead
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]

use std::process::ExitCode;

use anyhow::{bail, Context, Result};
use clap::Parser;
use colored::Colorize;
use config::Config;
use devices::TapoDevice;
use log::{error, info};
use tokio::{fs, task::JoinSet};

use crate::cmd::Cmd;

use self::logger::Logger;

mod cmd;
mod config;
mod devices;
mod logger;
mod server;

#[tokio::main]
async fn main() -> ExitCode {
    match inner_main().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            error!("{err:?}");
            ExitCode::FAILURE
        }
    }
}

async fn inner_main() -> Result<()> {
    let Cmd {
        devices_config_path,
        tapo_credentials,
        server_config,
        verbosity,
    } = Cmd::parse();

    // Set up the logger
    Logger::new(verbosity).init().unwrap();

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

    info!(
        "Attempting to connect to the {} configured device(s)...",
        devices.len()
    );

    for conn_infos in devices {
        let tapo_credentials = tapo_credentials.clone();

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

    info!("Now launching server...");

    server::serve(server_config, devices, data_dir.join("sessions.json")).await
}
