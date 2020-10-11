use chrono::Local;
use log::{Log, Metadata, Record, LevelFilter};
use std::future::Future;

tokio::task_local! {
    static TASK_NAME: String;
}

#[inline]
pub(crate) async fn setup_for_task<F: Future>(name: String, future: F) -> F::Output {
    TASK_NAME.scope(name, future).await
}

#[derive(Debug)]
struct QuickLogger {
    levels: Vec<(String, LevelFilter)>
}

impl Log for QuickLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let target = metadata.target();

        for (module, level) in &self.levels {
            if (target.len() >= module.len() + 2 && &target[..module.len()] == module || &module[..module.len() - 2] == target) && metadata.level() <= *level {
                return true;
            }
        }

        false
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

/// Sets up a very basic logger that prints
/// logs to stdout.
pub fn __internal_setup_logging(levels: Vec<(String, LevelFilter)>) {
    let mut max_level = LevelFilter::Off;
    for (_, level) in &levels {
        if *level >= max_level {
            max_level = *level;
        }
    }

    let logger = QuickLogger {
        levels
    };

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(max_level);
}