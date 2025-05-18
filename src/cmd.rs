use std::path::PathBuf;

use clap::Parser;
use log::LevelFilter;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cmd {
    #[clap(help = "Path to the devices config file (.json)")]
    pub devices_config_path: PathBuf,

    #[clap(flatten)]
    pub tapo_credentials: TapoCredentials,

    #[clap(flatten)]
    pub server_config: ServerConfig,

    #[clap(
        short,
        long,
        global = true,
        help = "Level of verbosity",
        default_value = "info"
    )]
    pub verbosity: LevelFilter,
}

#[derive(Parser, Clone)]
pub struct TapoCredentials {
    #[clap(long, env, help = "Your tapo account's email address")]
    pub tapo_email: String,

    #[clap(long, env, help = "Your tapo account's password")]
    pub tapo_password: String,
}

#[derive(Parser)]
pub struct ServerConfig {
    #[clap(short, long, env, help = "Port to serve on")]
    pub port: u16,

    #[clap(flatten)]
    pub password: PasswordArgGroup,
}

#[derive(Parser)]
#[group(required = true, multiple = false)]
pub struct PasswordArgGroup {
    #[clap(short, long, env, help = "Login password")]
    pub auth_password: Option<String>,

    #[clap(short = 'f', long, env, help = "Read the login password from a file")]
    pub password_from_file: Option<PathBuf>,
}
