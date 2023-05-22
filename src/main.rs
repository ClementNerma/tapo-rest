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
use tokio::task::JoinSet;

use crate::cmd::Cmd;

mod cmd;
mod config;
mod devices;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let Cmd {
        config_path,
        server_config,
    } = Cmd::parse();

    if !config_path.is_file() {
        bail!(
            "Configuration was not found at path {}",
            config_path.to_string_lossy()
        );
    }

    // NOTE: we can afford to block here as we didn't launch any task or future yet
    let config_str =
        std::fs::read_to_string(&config_path).context("Failed to read configuration file")?;

    let Config { account, devices } =
        serde_json::from_str(&config_str).context("Failed to parse configuration file")?;

    let mut tasks = JoinSet::new();

    println!(
        "| Attempting to connect to the {} configured devices...",
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

    server::serve(server_config, devices).await
}
