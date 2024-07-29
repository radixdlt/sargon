use crate::prelude::*;

/// * Trace
/// * Debug
/// * Info
/// * Warning
/// * Error
#[uniffi::export(with_foreign)]
pub trait LoggingDriver: Send + Sync + std::fmt::Debug {
    fn log(&self, level: LogLevel, msg: String);
}
