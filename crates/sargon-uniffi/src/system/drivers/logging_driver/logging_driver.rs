use crate::prelude::*;
use sargon::LogLevel as InternalLogLevel;
use sargon::LoggingDriver as InternalLoggingDriver;

/// * Trace
/// * Debug
/// * Info
/// * Warning
/// * Error
#[uniffi::export(with_foreign)]
pub trait LoggingDriver: Send + Sync + std::fmt::Debug {
    fn log(&self, level: LogLevel, msg: String);
}

#[derive(Debug)]
pub struct LoggingDriverAdapter {
    pub wrapped: Arc<dyn LoggingDriver>,
}

impl InternalLoggingDriver for LoggingDriverAdapter {
    fn log(&self, level: InternalLogLevel, msg: String) {
        self.wrapped.log(level.into(), msg)
    }
}
