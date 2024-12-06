#![cfg(test)]
#![allow(unused)]

use crate::prelude::*;

/// A type impl KeyDerivationInteractor suitable for tests.
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

        Self::do_derive_serially_looking_up_with_secure_storage_and_extra(
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

    /// Derives FactorInstances for `request` using the `lookup_mnemonic` closure
    async fn do_derive_serially_with_lookup_of_mnemonic<F>(
        factor_source_id: FactorSourceIDFromHash,
        derivation_paths: IndexSet<DerivationPath>,
        lookup_mnemonic: F,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>
    where
        F: async Fn(FactorSourceIDFromHash) -> Result<MnemonicWithPassphrase>,
    {
        let mnemonic = lookup_mnemonic(factor_source_id).await?;
        let keys = mnemonic
            ._derive_entity_creation_factor_instances(
                factor_source_id,
                derivation_paths,
            )
            .into_iter()
            .map(HierarchicalDeterministicFactorInstance::from)
            .collect::<IndexSet<_>>();
        Ok(keys)
    }

    /// Uses `do_derive_serially_with_lookup_of_mnemonic` to derive keys, providing
    /// an async closure which uses predefined samples or looks up the mnemonic using
    /// the factor source id, apart from a secondary lookup, `lookup_mnemonic`, passed
    /// as an argument, which could e.g. use secure storage client to try to load
    /// the mnemonic
    async fn do_derive_serially_looking_up_with_secure_storage_and_extra<F>(
        factor_source_id: FactorSourceIDFromHash,
        derivation_paths: IndexSet<DerivationPath>,
        secure_storage: Arc<SecureStorageClient>,
        lookup_mnemonic: F,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>
    where
        F: async Fn(FactorSourceIDFromHash) -> Result<MnemonicWithPassphrase>,
    {
        let cloned_client = secure_storage.clone();
        Self::do_derive_serially_with_lookup_of_mnemonic(
            factor_source_id,
            derivation_paths,
            async move |id| {
                if let Ok(m) = lookup_mnemonic(id).await {
                    return Ok(m);
                }
                let cloned_cloned_client = cloned_client.clone();
                cloned_cloned_client.load_mnemonic_with_passphrase(id).await
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
