use crate::prelude::*;
use sargon::{ManifestRequest, ScryptoManifestDriver as InternalScryptoManifestDriver};
use sargon::ManifestDriver as InternalManifestDriver;


#[derive(Debug)]
pub struct ManifestDriverAdapter {
    pub wrapped: Arc<InternalScryptoManifestDriver>,
}

impl ManifestDriverAdapter {
    pub fn new() -> Self {
        Self {
            wrapped: InternalScryptoManifestDriver::new(),
        }
    }
}

impl InternalManifestDriver for ManifestDriverAdapter {
    fn build_manifest(&self, request: ManifestRequest) -> sargon::TransactionManifest {
        self.wrapped.build_manifest(request)
    }
}
