use colored::Colorize;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

pub struct Logger {
    level: LevelFilter,
}

impl Logger {
    pub fn new(level: LevelFilter) -> Self {
        Self { level }
    }

    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.level);
        log::set_boxed_logger(Box::new(self))
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let msg = record.args().to_string();

        let colored = match record.level() {
            Level::Error => msg.bright_red(),
            Level::Warn => msg.bright_yellow(),
            Level::Info => msg.bright_blue(),
            Level::Debug => msg.bright_magenta(),
            Level::Trace => msg.bright_black(),
        };

        #[allow(clippy::print_stderr)]
        {
            eprintln!("{colored}");
        }
    }

    fn flush(&self) {}
}
