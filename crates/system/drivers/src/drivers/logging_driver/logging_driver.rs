use crate::prelude::*;

/// * Trace
/// * Debug
/// * Info
/// * Warning
/// * Error
pub trait LoggingDriver: Send + Sync + std::fmt::Debug {
    fn log(&self, level: LogLevel, msg: String);
}
