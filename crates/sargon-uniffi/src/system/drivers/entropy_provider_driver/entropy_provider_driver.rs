use crate::prelude::*;
use sargon::Entropy32Bytes as InternalEntropy32Bytes;
use sargon::EntropyProviderDriver as InternalEntropyProviderDriver;

#[uniffi::export(with_foreign)]
pub trait EntropyProviderDriver: Send + Sync + std::fmt::Debug {
    fn generate_secure_random_bytes(&self) -> Entropy32Bytes;
}

#[derive(Debug)]
pub struct EntropyProviderDriverAdapter {
    pub wrapped: Arc<dyn EntropyProviderDriver>,
}

impl InternalEntropyProviderDriver for EntropyProviderDriverAdapter {
    fn generate_secure_random_bytes(&self) -> InternalEntropy32Bytes {
        self.wrapped.generate_secure_random_bytes().into()
    }
}
