use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::prelude::*;

/// A type impl KeyDerivationInteractor suitable for tests.
///
/// Uses Sample values of MnemonicWithPassphrase for derivation, or looks up the mnemonic
/// using a SecureStorageClient
#[derive(Debug)]
pub struct TestDerivationInteractor {
    pub always_fail: bool,
    pub mnemonic_loading: Arc<dyn MnemonicLoading>,
}

impl Default for TestDerivationInteractor {
    fn default() -> Self {
        Self {
            always_fail: false,
            mnemonic_loading: Arc::new(FailingMnemonicLoader),
        }
    }
}

impl TestDerivationInteractor {
    pub fn with_mnemonic_loading(
        always_fail: bool,
        mnemonic_loading: Arc<dyn MnemonicLoading>,
    ) -> Self {
        Self {
            always_fail,
            mnemonic_loading,
        }
    }

    pub fn fail() -> Self {
        Self::with_mnemonic_loading(true, Arc::new(FailingMnemonicLoader))
    }

    async fn do_derive(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        derivation_paths: IndexSet<DerivationPath>,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        if self.always_fail {
            return Err(CommonError::Unknown {
                error_message: "Always fail".to_string(),
            });
        }

        let cloned_mnemonic_loading = self.mnemonic_loading.clone();

        Self::do_derive_serially_looking_up_with_secure_storage_and_extra(
            factor_source_id,
            derivation_paths,
            cloned_mnemonic_loading,
            async move |id| {
                id.maybe_sample_associated_mnemonic()
                    .ok_or(CommonError::FactorSourceDiscrepancy)
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
        mnemonic_loading: Arc<dyn MnemonicLoading>,
        lookup_mnemonic: F,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>
    where
        F: async Fn(FactorSourceIDFromHash) -> Result<MnemonicWithPassphrase>,
    {
        let cloned_mnemonic_loading = mnemonic_loading.clone();
        Self::do_derive_serially_with_lookup_of_mnemonic(
            factor_source_id,
            derivation_paths,
            async move |id| {
                if let Ok(m) = lookup_mnemonic(id).await {
                    return Ok(m);
                }
                let cloned_cloned_mnemonic_loading =
                    cloned_mnemonic_loading.clone();
                cloned_cloned_mnemonic_loading.load_mnemonic(id).await
            },
        )
        .await
    }
}

// ==== HERE BE DRAGONS ====
// Workaround from:
// https://github.com/rust-lang/rust/issues/64552#issuecomment-604419315
// for Implementation of `Send` is not general enough bug
struct IamSend<F: Future> {
    f: F,
}
impl<F: Future> IamSend<F> {
    pub unsafe fn new(f: F) -> Self {
        IamSend { f }
    }
}
unsafe impl<F: Future> Send for IamSend<F> {}
impl<F: Future> Future for IamSend<F> {
    type Output = F::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        unsafe { self.map_unchecked_mut(|s| &mut s.f).poll(cx) }
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
            let instances =
                unsafe { IamSend::new(self.do_derive(k, r)) }.await?;
            pairs.insert(k, instances);
        }
        Ok(KeyDerivationResponse::new(pairs))
    }
}
