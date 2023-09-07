#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
#![warn(unused_crate_dependencies)]

// OpenSSL is vendored, this instruction is not required
// but allows to get rind of the "unused external dependency" lint
use openssl as _;

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

    let Config { account, devices } = serde_json::from_str(&config_str)
        .context("Failed to parse the devices configuration file")?;

    let mut tasks = JoinSet::new();

    println!(
        "| Attempting to connect to the {} configured device(s)...",
        devices.len()
    );

    let mut remaining = devices.len();

    for conn_infos in devices {
        let account = account.clone();
        tasks.spawn(async move { TapoDevice::connect(conn_infos, &account).await });
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
