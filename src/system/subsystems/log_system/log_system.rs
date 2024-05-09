use std::sync::Once;

use crate::prelude::*;

#[derive(Debug)]
struct LogSystem(RwLock<Option<Arc<dyn LoggingDriver>>>);

static LOG: LogSystem = LogSystem(RwLock::new(None));

impl log::Log for LogSystem {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        let msg = record.args().to_string();
        let level = record.level();
        if let Some(driver) = &*self.0.read().unwrap()
            && !driver.is_rust_log()
        {
            driver.log(LogLevel::from(level), msg)
        } else {
            match record.level() {
                log::Level::Error => error!("{}", msg),
                log::Level::Warn => warn!("{}", msg),
                log::Level::Info => info!("{}", msg),
                log::Level::Debug => debug!("{}", msg),
                log::Level::Trace => trace!("{}", msg),
            }
        }
    }

    fn flush(&self) {}
}

fn init() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        log::set_logger(&LOG)
            .expect("Should always be able to install a logger.");
        log::set_max_level(log::LevelFilter::Trace);
    });
}

pub(crate) fn install_logger(logging_driver: Arc<dyn LoggingDriver>) {
    if !logging_driver.is_rust_log() {
        init();
        *LOG.0.write().unwrap() = Some(logging_driver);
    } else {
        pretty_env_logger::init();
    }
    rust_logger_set_level(LogLevel::Trace); // can be called from FFI later
    info!("Finished installing logger");
}

#[uniffi::export]
pub fn rust_logger_set_level(level: LogLevel) {
    let log_level = log::LevelFilter::from(level);
    log::set_max_level(log_level);
}
