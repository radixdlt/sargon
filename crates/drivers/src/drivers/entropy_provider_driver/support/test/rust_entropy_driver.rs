use crate::prelude::*;

/// A **Rust** entropy driver using `osrnd`.
#[derive(Debug)]
pub struct RustEntropyDriver;

impl RustEntropyDriver {
    pub fn new() -> Arc<Self> {
        Arc::new(RustEntropyDriver)
    }
}

impl EntropyProviderDriver for RustEntropyDriver {
    fn generate_secure_random_bytes(&self) -> Exactly32Bytes {
        Exactly32Bytes::generate()
    }
}
