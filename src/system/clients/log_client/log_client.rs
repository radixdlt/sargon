use std::sync::Once;

use crate::prelude::*;

#[derive(Debug)]
struct LogClient(RwLock<Option<Arc<dyn LoggingDriver>>>);

static LOG: LogClient = LogClient(RwLock::new(None));

impl log::Log for LogClient {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
#[repr(u8)]
pub enum LogLevel {
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error = 1,

    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn,

    /// The "info" level.
    ///
    /// Designates useful information.
    Info,

    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug,

    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace,
}
impl From<log::Level> for LogLevel {
    fn from(value: log::Level) -> Self {
        match value {
            log::Level::Error => Self::Error,
            log::Level::Warn => Self::Warn,
            log::Level::Info => Self::Info,
            log::Level::Debug => Self::Debug,
            log::Level::Trace => Self::Trace,
        }
    }
}
impl From<LogLevel> for log::LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Error => Self::Error,
            LogLevel::Warn => Self::Warn,
            LogLevel::Info => Self::Info,
            LogLevel::Debug => Self::Debug,
            LogLevel::Trace => Self::Trace,
        }
    }
}
