use log::{Log, Metadata, Record, LevelFilter};
use chrono::Local;
use std::future::Future;

tokio::task_local! {
    static TASK_NAME: String;
}

pub(crate) async fn setup_for_task<S: Into<String>, F: Future>(name: S, future: F) -> F::Output {
    TASK_NAME.scope(name.into(), future).await
}

struct QuickLogger;

impl Log for QuickLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let time = Local::now().format("%Y-%m-%d %H:%M:%S");

        let result = TASK_NAME.try_with(|task_name| {
            println!(
                "{} in {}({}) [{}]: {}",
                time,
                record.target(),
                task_name,
                record.level(),
                record.args()
            );
        });

        if result.is_err() {
            println!(
                "{} in {}(unknown) [{}]: {}",
                time,
                record.target(),
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static QUICK_LOGGER: QuickLogger = QuickLogger;

/// Sets up a very basic logger that prints
/// logs to stdout.
#[deprecated(since = "0.3.1", note = "Logger is automatically set up, use `Configuration::log_level` and `Configuration::disable_logging` to configure it. Using this function may cause crashes.")]
pub fn setup_logging() {
    log::set_logger(&QUICK_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);
}

/// Sets up a very basic logger that prints
/// logs to stdout.
pub fn __internal_setup_logging(level: LevelFilter) {
    log::set_logger(&QUICK_LOGGER).unwrap();
    log::set_max_level(level);
}