use crate::prelude::*;

/// A manifest driver that builds TransactionManifest using Scrypto Manifest Builder.
#[derive(Debug)]
pub struct ScryptoManifestDriver;

impl ScryptoManifestDriver {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

impl ManifestDriver for ScryptoManifestDriver {
    fn build_manifest(&self, request: ManifestRequest) -> TransactionManifest {
        match request {
            ManifestRequest::DeleteAccount(r) => {
                TransactionManifest::delete_account(
                    &r.account_address,
                    r.resource_preferences_to_be_removed,
                    r.authorized_depositors_to_be_removed,
                )
            }
        }
    }
}
