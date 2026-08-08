//
// Enable some strict rules
//
#![forbid(unsafe_code, unused_must_use)]
//
// Enable some additional warnings
//
#![warn(unused_crate_dependencies, missing_debug_implementations)]
//
// Enable all of Clippy's lints by default
//
#![warn(clippy::pedantic, clippy::cargo)]
//
// -> Enable some more lints from `restriction`
#![warn(clippy::as_conversions)]
//
// -> Then disable a few ones
//
#![allow(
    clippy::float_cmp,
    clippy::arithmetic_side_effects,
    clippy::integer_division,
    clippy::map_err_ignore,
    clippy::missing_const_for_fn,
    clippy::multiple_crate_versions,
    clippy::option_if_let_else,
    clippy::shadow_unrelated,
    clippy::unused_trait_names,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::wildcard_enum_match_arm,
    clippy::wildcard_imports,
    clippy::similar_names
)]

use std::process::ExitCode;

use anyhow::{Result, bail};
use log::{error, info};

use crate::cmd::Cmd;

use self::{logger::Logger, server::ServeOptions};

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
    } = argh::from_env::<Cmd>();

    // Set up the logger
    Logger::new(verbosity).init().unwrap();

    if !config_path.is_file() {
        bail!(
            "Configuration was not found at path {}",
            config_path.to_string_lossy()
        );
    }

    info!("Now launching server...");

    server::serve(ServeOptions { config_path, port }).await
}
