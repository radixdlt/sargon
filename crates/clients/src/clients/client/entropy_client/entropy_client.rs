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
        let entropy = self.driver.generate_secure_random_bytes();
        NonEmptyMax32Bytes::try_from(entropy.as_ref())
            .expect("Entropy is not empty")
    }
}
