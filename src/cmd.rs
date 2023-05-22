use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cmd {
    #[clap(short, long, help = "Path to the devices config file (.json)")]
    pub devices_config_path: PathBuf,

    #[clap(flatten)]
    pub server_config: ServerConfig,
}

#[derive(Parser)]
pub struct ServerConfig {
    #[clap(short, long, help = "Port to serve on")]
    pub port: u16,

    #[clap(short, long, help = "Login password")]
    pub auth_password: String,
}
