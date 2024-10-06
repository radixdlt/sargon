use std::{ffi::OsStr, sync::Once};

use crate::prelude::*;

#[derive(Debug)]
struct LogSystem(RwLock<Option<Arc<dyn LoggingDriver>>>);

static LOG: LogSystem = LogSystem(RwLock::new(None));

impl log::Log for LogSystem {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let msg = record.args().to_string();
        let level = record.level();
        if let Some(driver) = &*self.0.read().unwrap() {
            driver.log(LogLevel::from(level), msg)
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
    rust_logger_set_level(LogFilter::Trace); // can be called from FFI later
    debug!("Finished installing logger");
}

pub fn rust_logger_init() {
    install_logger(RustLoggingDriver::new())
}

pub fn rust_logger_set_level(level: LogFilter) {
    let log_level = log::LevelFilter::from(level);
    log::set_max_level(log_level);
    std::env::set_var(
        "RUST_LOG",
        std::ffi::OsStr::new(&format!("{:?}", log_level)),
    );
}

/// Returns every supported LogFilter
pub fn rust_logger_get_all_filters() -> Vec<LogFilter> {
    all::<LogFilter>().collect()
}

/// Returns every supported LogLevel
pub fn rust_logger_get_all_levels() -> Vec<LogLevel> {
    all::<LogLevel>().collect()
}

pub fn rust_logger_get_level() -> LogFilter {
    LogFilter::from(log::max_level())
}

pub fn rust_logger_log_at_every_level() {
    error!("Rust test: 'error'");
    warn!("Rust test: 'warn'");
    info!("Rust test: 'info'");
    debug!("Rust test: 'debug'");
    trace!("Rust test: 'trace'");
}

#[cfg(test)]
mod tests {

    use log::Log;

    use super::*;

    #[test]
    fn install_rust_logger_change_level() {
        install_logger(RustLoggingDriver::new());
        let new = LogFilter::Warn;
        rust_logger_set_level(new);
        assert_eq!(rust_logger_get_level(), new)
    }

    #[test]
    fn test_flush() {
        let driver = RustLoggingDriver::new();
        install_logger(driver);
        LOG.flush();
    }

    #[test]
    fn test_rust_logger_init() {
        rust_logger_init()
    }

    #[test]
    fn test_rust_logger_get_all_levels() {
        assert_eq!(
            rust_logger_get_all_levels(),
            all::<LogLevel>().collect_vec()
        );
    }
}
