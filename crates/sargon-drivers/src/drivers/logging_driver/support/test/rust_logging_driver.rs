use log::Log;

use crate::prelude::*;

#[derive(Debug)]
pub struct RustLoggingDriver {
    logger: pretty_env_logger::env_logger::Logger,
}

impl RustLoggingDriver {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            logger: pretty_env_logger::env_logger::builder()
                .parse_default_env()
                .build(),
        })
    }
}

impl LoggingDriver for RustLoggingDriver {
    fn log(&self, level: LogLevel, msg: String) {
        self.logger.log(
            &log::Record::builder()
                .level(level.into())
                .args(format_args!("{}", msg))
                .build(),
        );
    }
}
