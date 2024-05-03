use crate::prelude::*;

/// * Trace
/// * Debug
/// * Info
/// * Warning
/// * Error
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait LoggingDriver: Send + Sync + std::fmt::Debug {
    /// Swift (apple/swift-log): `trace`
    /// Rust (log::LevelFilter): `trace`
    /// Kotlin (Android Timber): `verbose`
    fn trace(&self, msg: String);

    /// Swift (apple/swift-log): `debug`
    /// Rust (log::LevelFilter): `debug`
    /// Kotlin (Android Timber): `debug`
    fn debug(&self, msg: String);

    /// Swift (apple/swift-log): `info`
    /// Rust (log::LevelFilter): `info`
    /// Kotlin (Android Timber): `info`
    fn info(&self, msg: String);

    /// Swift (apple/swift-log): `warning`
    /// Rust (log::LevelFilter): `warn`
    /// Kotlin (Android Timber): `warn`
    fn warning(&self, msg: String);

    /// Swift (apple/swift-log): `error`
    /// Rust (log::LevelFilter): `error`
    /// Kotlin (Android Timber): `error`
    fn error(&self, msg: String);
}
