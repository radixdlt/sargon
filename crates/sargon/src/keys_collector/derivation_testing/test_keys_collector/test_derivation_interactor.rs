#![cfg(test)]
#![allow(unused)]

use crate::prelude::*;

/// A type impl PolyFactorKeyDerivationInteractor and MonoFactorKeyDerivationInteractor
/// suitable for tests.
///
/// Uses Sample values of MnemonicWithPassphrase for derivation, or looks up the mnemonic
/// using a SecureStorageClient
#[derive(Debug)]
pub(crate) struct TestDerivationInteractor {
    pub always_fail: bool,
    pub secure_storage_client: Arc<SecureStorageClient>,
}

impl Default for TestDerivationInteractor {
    fn default() -> Self {
        Self {
            always_fail: false,
            secure_storage_client: Arc::new(SecureStorageClient::ephemeral().0),
        }
    }
}

impl TestDerivationInteractor {
    pub(crate) fn new(
        always_fail: bool,
        secure_storage_client: Arc<SecureStorageClient>,
    ) -> Self {
        Self {
            always_fail,
            secure_storage_client,
        }
    }

    pub(crate) fn fail() -> Self {
        Self::new(true, Arc::new(SecureStorageClient::always_fail()))
    }

    async fn do_derive(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        derivation_paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        if self.always_fail {
            return Err(CommonError::Unknown);
        }

        let cloned_client = self.secure_storage_client.clone();

        __do_derive_serially_looking_up_with_secure_storage_and_extra(
            factor_source_id,
            derivation_paths,
            cloned_client,
            async move |id| {
                id.maybe_sample_associated_mnemonic()
                    .ok_or(CommonError::Unknown)
            },
        )
        .await
    }
}

#[async_trait::async_trait]
impl KeyDerivationInteractor for TestDerivationInteractor {
    async fn derive(
        &self,
        request: KeyDerivationRequest,
    ) -> Result<KeyDerivationResponse> {
        let mut pairs = IndexMap::new();
        for (k, r) in request.per_factor_source {
            let instances = self.do_derive(k, r).await?;
            pairs.insert(k, instances);
        }
        Ok(KeyDerivationResponse::new(pairs))
    }
}
