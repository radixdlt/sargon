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
    pub secure_storage_client: SecureStorageClient,
}

impl Default for TestDerivationMonoAndPolyInteractor {
    fn default() -> Self {
        Self {
            always_fail: false,
            secure_storage_client: SecureStorageClient::ephemeral().0,
        }
    }
}

impl TestDerivationMonoAndPolyInteractor {
    pub(crate) fn new(
        always_fail: bool,
        secure_storage_client: SecureStorageClient,
    ) -> Self {
        Self {
            always_fail,
            secure_storage_client,
        }
    }

    pub(crate) fn fail() -> Self {
        Self::new(true, SecureStorageClient::always_fail())
    }

    async fn do_derive(
        &self,
        request: MonoFactorKeyDerivationRequest,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        if self.always_fail {
            return Err(CommonError::Unknown);
        }

        let cloned_client = Arc::new(self.secure_storage_client.clone());
        do_derive_serially_looking_up_mnemonic_amongst_samples(
            request,
            move |id| {
                let cloned_cloned_client = cloned_client.clone();
                async move {
                    cloned_cloned_client.load_mnemonic_with_passphrase(id).await
                }
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

/// Derives FactorInstances for `request` using the `lookup_mnemonic` closure
async fn __do_derive_serially_with_lookup_of_mnemonic<F>(
    request: MonoFactorKeyDerivationRequest,
    lookup_mnemonic: F,
) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>
where
    F: async Fn(FactorSourceIDFromHash) -> Result<MnemonicWithPassphrase>,
{
    let factor_source_id = request.factor_source_id;
    let mut out = IndexSet::<HierarchicalDeterministicFactorInstance>::new();

    for path in request.derivation_paths {
        let mnemonic = lookup_mnemonic(factor_source_id).await?;
        let seed = mnemonic.to_seed();
        let hd_private_key = seed.derive_private_key(&path);
        out.insert(HierarchicalDeterministicFactorInstance::new(
            factor_source_id,
            hd_private_key.public_key(),
        ));
    }
    Ok(out)
}

/// Uses `__do_derive_serially_with_lookup_of_mnemonic` to derive keys, providing
/// an async closure which uses predefined samples or looks up the mnemonic using
/// the factor source id, apart from a secondary lookup, `lookup_mnemonic`, passed
/// as an argument, which could e.g. use secure storage client to try to load
/// the mnemonic
async fn do_derive_serially_looking_up_mnemonic_amongst_samples<F>(
    request: MonoFactorKeyDerivationRequest,
    lookup_mnemonic: F,
) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>
where
    F: async Fn(FactorSourceIDFromHash) -> Result<MnemonicWithPassphrase>,
{
    __do_derive_serially_with_lookup_of_mnemonic(
        request,
        async move |f: FactorSourceIDFromHash| {
            if let Some(value) = f.maybe_sample_associated_mnemonic() {
                return Ok(value);
            };
            lookup_mnemonic(f).await
        },
    )
    .await
}
