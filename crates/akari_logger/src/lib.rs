use colored::Colorize;
use log::{info, Level, LevelFilter};

// `log` re-export
pub use log::*;

static LOGGER: AkariLogger = AkariLogger;

pub struct AkariLogger;

impl AkariLogger {
    pub fn init() {
        match log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace)) {
            Ok(_) => info!("Logger initialized successfully!"),
            Err(err) => panic!("Couldn't init AkariLogger: `{}`", err),
        }
    }
}

impl log::Log for AkariLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let prefix = match record.level() {
                Level::Debug => "  DEBUG".bright_black(),
                Level::Trace => "  TRACE".bright_white(),
                Level::Info => "   INFO".bright_green(),
                Level::Warn => "WARNING".yellow(),
                Level::Error => "  ERROR".bright_red(),
            }
            .bold();

            let origin = format!(
                "[{}] ({}:{})",
                record.module_path().unwrap(),
                record.file().unwrap(),
                record.line().unwrap()
            )
            .bright_black()
            .italic();
            println!("{} {} {}", prefix, record.args(), origin);
        }
    }

    fn flush(&self) {}
}
