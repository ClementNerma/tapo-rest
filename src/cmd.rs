use std::path::PathBuf;

use argh::FromArgs;
use log::LevelFilter;

#[derive(FromArgs)]
#[argh(description = "Tapo REST server")]
pub struct Cmd {
    #[argh(positional, description = "path to the configuration file (.json)")]
    pub config_path: PathBuf,

    #[argh(option, short = 'p', long = "port", description = "port to serve on")]
    pub port: u16,

    #[argh(
        option,
        short = 'v',
        long = "verbosity",
        description = "level of verbosity",
        default = "LevelFilter::Info"
    )]
    pub verbosity: LevelFilter,
}
