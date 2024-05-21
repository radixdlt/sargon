use crate::prelude::*;

#[uniffi::export(with_foreign)]
pub trait EntropyProviderDriver: Send + Sync + std::fmt::Debug {
    fn generate_secure_random_bytes(&self) -> Exactly32Bytes;
}
