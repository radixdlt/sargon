use crate::prelude::*;
use sargon::LogFilter as InternalLogFilter;
use sargon::LogLevel as InternalLogLevel;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
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

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
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
