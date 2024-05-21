use crate::prelude::*;

#[derive(Debug)]
pub struct EntropyClient {
    driver: Arc<dyn EntropyProviderDriver>,
}

impl EntropyClient {
    pub fn new(driver: Arc<dyn EntropyProviderDriver>) -> Self {
        Self { driver }
    }

    pub fn bip39_entropy(&self) -> NonEmptyMax32Bytes {
        NonEmptyMax32Bytes::from(self.driver.generate_secure_random_bytes())
    }
}
