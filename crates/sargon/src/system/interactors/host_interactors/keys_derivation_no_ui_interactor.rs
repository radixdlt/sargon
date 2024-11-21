#![allow(unused)]
use futures::future::ready;

use crate::prelude::*;

pub struct NoUIInteractorForKeyDerivation {
    pub(crate) poly: Arc<dyn PolyFactorKeyDerivationInteractor>,
    pub(crate) mono: Arc<dyn MonoFactorKeyDerivationInteractor>,
}

impl KeysDerivationInteractors for NoUIInteractorForKeyDerivation {
    fn interactor_for(
        &self,
        kind: FactorSourceKind,
    ) -> KeyDerivationInteractor {
        match kind {
            FactorSourceKind::Device => {
                KeyDerivationInteractor::poly(self.poly.clone())
            }
            _ => KeyDerivationInteractor::mono(self.mono.clone()),
        }
    }
}

impl NoUIInteractorForKeyDerivation {
    pub fn new(secure_storage_client: Arc<SecureStorageClient>) -> Self {
        let interactor =
            Arc::new(NoUIMonoAndPolyInteractorForKeyDerivation::new(
                secure_storage_client.clone(),
            ));
        Self {
            mono: interactor.clone(),
            poly: interactor.clone(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct NoUIMonoAndPolyInteractorForKeyDerivation {
    pub secure_storage_client: Arc<SecureStorageClient>,
}

impl NoUIMonoAndPolyInteractorForKeyDerivation {
    pub fn new(secure_storage_client: Arc<SecureStorageClient>) -> Self {
        Self {
            secure_storage_client,
        }
    }

    async fn do_derive(
        &self,
        request: MonoFactorKeyDerivationRequest,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        __do_derive_serially_looking_up_with_secure_storage_and_extra(
            request,
            self.secure_storage_client.clone(),
            |_| ready(Err(CommonError::Unknown)),
        )
        .await
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
    let paths = request.derivation_paths;
    let mnemonic = lookup_mnemonic(factor_source_id).await?;
    let keys = mnemonic
        ._derive_entity_creation_factor_instances(factor_source_id, paths)
        .into_iter()
        .map(HierarchicalDeterministicFactorInstance::from)
        .collect::<IndexSet<_>>();
    Ok(keys)
}

/// Uses `__do_derive_serially_with_lookup_of_mnemonic` to derive keys, providing
/// an async closure which uses predefined samples or looks up the mnemonic using
/// the factor source id, apart from a secondary lookup, `lookup_mnemonic`, passed
/// as an argument, which could e.g. use secure storage client to try to load
/// the mnemonic
pub async fn __do_derive_serially_looking_up_with_secure_storage_and_extra<F>(
    request: MonoFactorKeyDerivationRequest,
    secure_storage: Arc<SecureStorageClient>,
    lookup_mnemonic: F,
) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>
where
    F: async Fn(FactorSourceIDFromHash) -> Result<MnemonicWithPassphrase>,
{
    let cloned_client = secure_storage.clone();
    __do_derive_serially_with_lookup_of_mnemonic(request, async move |id| {
        if let Ok(m) = lookup_mnemonic(id).await {
            return Ok(m);
        }
        let cloned_cloned_client = cloned_client.clone();
        cloned_cloned_client.load_mnemonic_with_passphrase(id).await
    })
    .await
}

#[async_trait::async_trait]
impl PolyFactorKeyDerivationInteractor
    for NoUIMonoAndPolyInteractorForKeyDerivation
{
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
impl MonoFactorKeyDerivationInteractor
    for NoUIMonoAndPolyInteractorForKeyDerivation
{
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
