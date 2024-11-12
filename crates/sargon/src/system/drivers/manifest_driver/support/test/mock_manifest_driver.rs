#![cfg(test)]

use crate::prelude::*;

#[derive(Debug)]
pub struct MockManifestDriver {
    mock_response: TransactionManifest,
    spy: fn(ManifestRequest) -> (),
}

impl ManifestDriver for MockManifestDriver {
    fn build_manifest(&self, request: ManifestRequest) -> TransactionManifest {
        (self.spy)(request);
        self.mock_response.clone()
    }
}

impl MockManifestDriver {
    pub fn new(mock_response: TransactionManifest, spy: fn(ManifestRequest) -> ()) -> Self {
        Self {
            mock_response,
            spy,
        }
    }
}
