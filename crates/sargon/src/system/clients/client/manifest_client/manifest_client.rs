use crate::prelude::*;

/// A `ManifestClient` needs a "builder" to be able to create the different manifests.
#[derive(Debug)]
pub struct ManifestClient {
    /// An object implementing the `ManifestDriver` trait.
    pub driver: Arc<dyn ManifestDriver>,
}

impl ManifestClient {
    pub fn new(driver: Arc<dyn ManifestDriver>) -> Self {
        Self { driver }
    }
}

impl ManifestClient {
    pub fn build_manifest(&self, request: ManifestRequest) -> TransactionManifest {
        self.driver.build_manifest(request)
    }
}
