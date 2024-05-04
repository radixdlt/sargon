use std::sync::Once;

use crate::prelude::*;

#[derive(Debug)]
struct LogSystem(RwLock<Option<Arc<dyn LoggingDriver>>>);

static LOG: LogSystem = LogSystem(RwLock::new(None));

impl log::Log for LogSystem {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        if let Some(driver) = &*self.0.read().unwrap() {
            let msg = record.args().to_string();
            driver.log(LogLevel::from(record.level()), msg)
        }
    }

    fn flush(&self) {}
}

fn init() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        log::set_logger(&LOG)
            .expect("Should always be able to install a logger.");
        log::set_max_level(log::LevelFilter::Trace);
    });
}

pub(crate) fn install_logger(logging_driver: Arc<dyn LoggingDriver>) {
    init();
    *LOG.0.write().unwrap() = Some(logging_driver);
    rust_logger_set_level(LogLevel::Trace) // can be called from FFI later
}

#[uniffi::export]
pub fn rust_logger_set_level(level: LogLevel) {
    let log_level = log::LevelFilter::from(level);
    log::set_max_level(log_level);
}
