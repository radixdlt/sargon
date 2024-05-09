use crate::prelude::*;

#[derive(Debug)]
pub struct RustLoggingDriver;

impl RustLoggingDriver {
    pub fn new() -> Arc<Self> {
        Arc::new(RustLoggingDriver)
    }
}

#[async_trait::async_trait]
impl LoggingDriver for RustLoggingDriver {
    fn log(&self, level: LogLevel, msg: String) {
        log::log!(level.into(), "{}", msg)
    }
}
