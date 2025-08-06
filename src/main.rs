#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
#![warn(unused_crate_dependencies)]
// Use logging instead
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]

use std::process::ExitCode;

use anyhow::{Context, Result, bail};
use clap::Parser;
use log::{error, info};
use tokio::fs;

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
        config_path,
        port,
        verbosity,
    } = Cmd::parse();

    // Set up the logger
    Logger::new(verbosity).init().unwrap();

    let data_dir = dirs::state_dir()
        .or_else(dirs::data_local_dir)
        .context("Failed to find a valid local data directory")?
        .join(env!("CARGO_PKG_NAME"));

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .await
            .context("Failed to create a local data directory")?;
    }

    if !config_path.is_file() {
        bail!(
            "Configuration was not found at path {}",
            config_path.to_string_lossy()
        );
    }

    info!("Now launching server...");

    server::serve(port, config_path, data_dir.join("sessions.json")).await
}
