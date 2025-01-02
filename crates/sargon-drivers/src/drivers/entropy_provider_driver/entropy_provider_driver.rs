use crate::prelude::*;

pub trait EntropyProviderDriver: Send + Sync + std::fmt::Debug {
    fn generate_secure_random_bytes(&self) -> Entropy32Bytes;
}
