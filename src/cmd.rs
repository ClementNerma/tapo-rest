use std::path::PathBuf;

use clap::Parser;
use log::LevelFilter;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cmd {
    #[clap(help = "Path to the configuration file (.json)")]
    pub config_path: PathBuf,

    #[clap(short, long, env, help = "Port to serve on")]
    pub port: u16,

    #[clap(
        short,
        long,
        global = true,
        help = "Level of verbosity",
        default_value = "info"
    )]
    pub verbosity: LevelFilter,
}
