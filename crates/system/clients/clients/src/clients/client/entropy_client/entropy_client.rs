use crate::prelude::*;

#[derive(Debug)]
pub struct EntropyClient {
    driver: Arc<dyn EntropyProviderDriver>,
}

impl EntropyClient {
    pub(crate) fn new(driver: Arc<dyn EntropyProviderDriver>) -> Self {
        Self { driver }
    }

    pub fn bip39_entropy(&self) -> BIP39Entropy {
        BIP39Entropy::from(self.driver.generate_secure_random_bytes())
    }
}
