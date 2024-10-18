use crate::prelude::*;

#[uniffi::export]
pub fn rust_logger_set_level(level: LogFilter) {
    sargon::rust_logger_set_level(level.into_internal());
}

/// Returns every supported LogFilter
#[uniffi::export]
pub fn rust_logger_get_all_filters() -> Vec<LogFilter> {
    sargon::rust_logger_get_all_filters().into_type()
}

/// Returns every supported LogLevel
#[uniffi::export]
pub fn rust_logger_get_all_levels() -> Vec<LogLevel> {
    sargon::rust_logger_get_all_levels().into_type()
}

#[uniffi::export]
pub fn rust_logger_get_level() -> LogFilter {
    sargon::rust_logger_get_level().into()
}

#[uniffi::export]
pub fn rust_logger_log_at_every_level() {
    sargon::rust_logger_log_at_every_level()
}
