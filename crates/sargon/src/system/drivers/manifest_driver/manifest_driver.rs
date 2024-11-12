use crate::prelude::*;

/// Trait for building `TransactionManifests`.
#[async_trait::async_trait]
pub trait ManifestDriver: Send + Sync + Debug {
    /// Build a `TransactionManifest` from its request.
    fn build_manifest(&self, request: ManifestRequest) -> TransactionManifest;
}
