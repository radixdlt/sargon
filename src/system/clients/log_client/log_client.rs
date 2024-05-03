use crate::prelude::*;

#[derive(Debug)]
pub struct LogClient {
    driver: Arc<dyn LoggingDriver>,
}

impl LogClient {
    #[allow(unused)]
    pub(crate) fn new(driver: Arc<dyn LoggingDriver>) -> Self {
        Self { driver }
    }

    pub fn trace(&self, msg: impl AsRef<str>) {
        self.driver.trace(msg.as_ref().to_owned())
    }

    pub fn debug(&self, msg: impl AsRef<str>) {
        self.driver.debug(msg.as_ref().to_owned())
    }

    pub fn info(&self, msg: impl AsRef<str>) {
        self.driver.info(msg.as_ref().to_owned())
    }

    pub fn warning(&self, msg: impl AsRef<str>) {
        self.driver.warning(msg.as_ref().to_owned())
    }

    pub fn error(&self, msg: impl AsRef<str>) {
        self.driver.error(msg.as_ref().to_owned())
    }
}
