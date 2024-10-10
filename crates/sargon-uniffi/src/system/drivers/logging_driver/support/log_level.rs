use crate::prelude::*;
use sargon::LogLevel as InternalLogLevel;

#[derive(
    Clone,
    
    
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
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

impl From<InternalLogLevel> for LogLevel {
    fn from(value: InternalLogLevel) -> Self {
        match value {
            InternalLogLevel::Error => Self::Error,
            InternalLogLevel::Warn => Self::Warn,
            InternalLogLevel::Info => Self::Info,
            InternalLogLevel::Debug => Self::Debug,
            InternalLogLevel::Trace => Self::Trace,
        }
    }
}

impl Into<InternalLogLevel> for LogLevel {
    fn into(self) -> InternalLogLevel {
        match self {
            LogLevel::Error => InternalLogLevel::Error,
            LogLevel::Warn => InternalLogLevel::Warn,
            LogLevel::Info => InternalLogLevel::Info,
            LogLevel::Debug => InternalLogLevel::Debug,
            LogLevel::Trace => InternalLogLevel::Trace,
        }
    }
}

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
#[repr(u8)]
pub enum LogFilter {
    /// Logging is turned off
    Off = 0,

    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error,

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

impl From<LogLevel> for log::Level {
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

impl From<LogFilter> for log::LevelFilter {
    fn from(value: LogFilter) -> Self {
        match value {
            LogFilter::Off => Self::Off,
            LogFilter::Error => Self::Error,
            LogFilter::Warn => Self::Warn,
            LogFilter::Info => Self::Info,
            LogFilter::Debug => Self::Debug,
            LogFilter::Trace => Self::Trace,
        }
    }
}

impl From<log::LevelFilter> for LogFilter {
    fn from(value: log::LevelFilter) -> Self {
        match value {
            log::LevelFilter::Off => Self::Off,
            log::LevelFilter::Error => Self::Error,
            log::LevelFilter::Warn => Self::Warn,
            log::LevelFilter::Info => Self::Info,
            log::LevelFilter::Debug => Self::Debug,
            log::LevelFilter::Trace => Self::Trace,
        }
    }
}

