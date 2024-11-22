#![cfg(test)]
#![allow(unused)]

use crate::prelude::*;

/// A type impl PolyFactorKeyDerivationInteractor and MonoFactorKeyDerivationInteractor
/// suitable for tests.
///
/// Uses Sample values of MnemonicWithPassphrase for derivation, or looks up the mnemonic
/// using a SecureStorageClient
#[derive(Debug)]
pub(crate) struct TestDerivationMonoAndPolyInteractor {
    pub always_fail: bool,
    pub secure_storage_client: Arc<SecureStorageClient>,
}

impl Default for TestDerivationMonoAndPolyInteractor {
    fn default() -> Self {
        Self {
            always_fail: false,
            secure_storage_client: Arc::new(SecureStorageClient::ephemeral().0),
        }
    }
}

impl TestDerivationMonoAndPolyInteractor {
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
        request: MonoFactorKeyDerivationRequest,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        if self.always_fail {
            return Err(CommonError::Unknown);
        }

        let cloned_client = self.secure_storage_client.clone();

        __do_derive_serially_looking_up_with_secure_storage_and_extra(
            request,
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
impl PolyFactorKeyDerivationInteractor for TestDerivationMonoAndPolyInteractor {
    async fn derive(
        &self,
        request: PolyFactorKeyDerivationRequest,
    ) -> Result<KeyDerivationResponse> {
        let mut pairs = IndexMap::new();
        for (k, r) in request.per_factor_source {
            let instances = self.do_derive(r).await?;
            pairs.insert(k, instances);
        }
        Ok(KeyDerivationResponse::new(pairs))
    }
}

#[async_trait::async_trait]
impl MonoFactorKeyDerivationInteractor for TestDerivationMonoAndPolyInteractor {
    async fn derive(
        &self,
        request: MonoFactorKeyDerivationRequest,
    ) -> Result<KeyDerivationResponse> {
        let instances = self.do_derive(request.clone()).await?;
        Ok(KeyDerivationResponse::new(IndexMap::just((
            request.factor_source_id,
            instances,
        ))))
    }
}
