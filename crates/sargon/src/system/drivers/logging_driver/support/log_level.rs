use crate::prelude::*;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    enum_iterator::Sequence,
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

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    enum_iterator::Sequence,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_from_log() {
        let test = |l: log::LevelFilter, exp: LogFilter| {
            let x = LogFilter::from(l);
            assert_eq!(x, exp);
        };
        test(log::LevelFilter::Off, LogFilter::Off);
        test(log::LevelFilter::Error, LogFilter::Error);
        test(log::LevelFilter::Warn, LogFilter::Warn);
        test(log::LevelFilter::Info, LogFilter::Info);
        test(log::LevelFilter::Debug, LogFilter::Debug);
        test(log::LevelFilter::Trace, LogFilter::Trace);
    }

    #[test]
    fn test_filter_to_log() {
        let test = |l: LogFilter, exp: log::LevelFilter| {
            let x = log::LevelFilter::from(l);
            assert_eq!(x, exp);
        };
        test(LogFilter::Off, log::LevelFilter::Off);
        test(LogFilter::Error, log::LevelFilter::Error);
        test(LogFilter::Warn, log::LevelFilter::Warn);
        test(LogFilter::Info, log::LevelFilter::Info);
        test(LogFilter::Debug, log::LevelFilter::Debug);
        test(LogFilter::Trace, log::LevelFilter::Trace);
    }

    #[test]
    fn test_level_to_log() {
        let test = |l: LogLevel, exp: log::Level| {
            let x = log::Level::from(l);
            assert_eq!(x, exp);
        };
        test(LogLevel::Error, log::Level::Error);
        test(LogLevel::Warn, log::Level::Warn);
        test(LogLevel::Info, log::Level::Info);
        test(LogLevel::Debug, log::Level::Debug);
        test(LogLevel::Trace, log::Level::Trace);
    }

    #[test]
    fn test_level_from_log() {
        let test = |l: log::Level, exp: LogLevel| {
            let x = LogLevel::from(l);
            assert_eq!(x, exp);
        };
        test(log::Level::Error, LogLevel::Error);
        test(log::Level::Warn, LogLevel::Warn);
        test(log::Level::Info, LogLevel::Info);
        test(log::Level::Debug, LogLevel::Debug);
        test(log::Level::Trace, LogLevel::Trace);
    }
}
