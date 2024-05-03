use crate::prelude::*;

/// * Trace
/// * Debug
/// * Info
/// * Warning
/// * Error
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait LoggingDriver: Send + Sync + std::fmt::Debug {
    fn log(&self, level: LogLevel, msg: String);
}
