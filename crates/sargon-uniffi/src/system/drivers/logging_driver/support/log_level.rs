use crate::prelude::*;
use sargon::LogFilter as InternalLogFilter;
use sargon::LogLevel as InternalLogLevel;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
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

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
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

impl From<InternalLogFilter> for LogFilter {
    fn from(value: InternalLogFilter) -> Self {
        match value {
            InternalLogFilter::Off => Self::Off,
            InternalLogFilter::Error => Self::Error,
            InternalLogFilter::Warn => Self::Warn,
            InternalLogFilter::Info => Self::Info,
            InternalLogFilter::Debug => Self::Debug,
            InternalLogFilter::Trace => Self::Trace,
        }
    }
}

impl Into<InternalLogFilter> for LogFilter {
    fn into(self) -> InternalLogFilter {
        match self {
            LogFilter::Off => InternalLogFilter::Off,
            LogFilter::Error => InternalLogFilter::Error,
            LogFilter::Warn => InternalLogFilter::Warn,
            LogFilter::Info => InternalLogFilter::Info,
            LogFilter::Debug => InternalLogFilter::Debug,
            LogFilter::Trace => InternalLogFilter::Trace,
        }
    }
}
