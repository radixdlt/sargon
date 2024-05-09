use crate::prelude::*;

/// * Trace
/// * Debug
/// * Info
/// * Warning
/// * Error
#[uniffi::export(with_foreign)]
pub trait LoggingDriver: Send + Sync + std::fmt::Debug {
    /// Should clean this up in the future, but for now this
    /// does the job, FFI implementors should return `false`.
    /// this is just a messy solution avoiding infinite
    /// recursion in the RustLoggingDriver, which is noop for
    /// log, and instead in the logging system we avoid installing
    /// the RustLoggingDriver...
    fn is_rust_log(&self) -> bool;
    fn log(&self, level: LogLevel, msg: String);
}
