use log::{Log, Metadata, Record, LevelFilter, Level};
use chrono::{Local, Timelike};

struct QuickLogger;

impl Log for QuickLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() == Level::Error ||
            metadata.target().starts_with("automate") ||
            metadata.target().starts_with(env!("CARGO_PKG_NAME"))
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let time = Local::now();

        println!(
            "{:02}:{:02}:{:02} in {} [{}]: {}",
            time.hour(),
            time.minute(),
            time.second(),
            record.target(),
            record.level(),
            record.args()
        );
    }

    fn flush(&self) {}
}

static QUICK_LOGGER: QuickLogger = QuickLogger;

/// Sets up a very basic logger that prints
/// logs to stdout.
pub fn setup_logging() {
    log::set_logger(&QUICK_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);
}