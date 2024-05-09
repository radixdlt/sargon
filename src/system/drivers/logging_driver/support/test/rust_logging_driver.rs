use crate::prelude::*;

#[derive(Debug)]
pub struct RustLoggingDriver;

impl RustLoggingDriver {
    pub fn new() -> Arc<Self> {
        Arc::new(RustLoggingDriver)
    }
}

impl LoggingDriver for RustLoggingDriver {
    fn is_rust_log(&self) -> bool {
        true
    }
    fn log(&self, _level: LogLevel, _msg: String) {
        /* special */
    }
}
