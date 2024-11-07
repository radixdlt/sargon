#![cfg(test)]
#![allow(unused)]

use crate::prelude::*;

#[derive(Debug)]
pub(crate) struct TestDerivationInteractors {
    pub(crate) poly: Arc<dyn PolyFactorKeyDerivationInteractor>,
    pub(crate) mono: Arc<dyn MonoFactorKeyDerivationInteractor>,
}
impl TestDerivationInteractors {
    pub(crate) fn new(
        poly: impl PolyFactorKeyDerivationInteractor + 'static,
        mono: impl MonoFactorKeyDerivationInteractor + 'static,
    ) -> Self {
        Self {
            poly: Arc::new(poly),
            mono: Arc::new(mono),
        }
    }
}

impl TestDerivationInteractors {
    pub(crate) fn fail() -> Self {
        Self::new(
            TestDerivationPolyInteractor::fail(),
            TestDerivationMonoInteractor::fail(),
        )
    }
}
impl Default for TestDerivationInteractors {
    fn default() -> Self {
        Self::new(
            TestDerivationPolyInteractor::default(),
            TestDerivationMonoInteractor::default(),
        )
    }
}

impl KeysDerivationInteractors for TestDerivationInteractors {
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

#[derive(Debug)]
pub(crate) struct TestDerivationPolyInteractor {
    handle: fn(
        MonoFactorKeyDerivationRequest,
    )
        -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>,
}
impl TestDerivationPolyInteractor {
    pub(crate) fn new(
        handle: fn(
            MonoFactorKeyDerivationRequest,
        )
            -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>,
    ) -> Self {
        Self { handle }
    }
    pub(crate) fn fail() -> Self {
        Self::new(|_| Err(CommonError::Unknown))
    }
    fn derive(
        &self,
        request: MonoFactorKeyDerivationRequest,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        (self.handle)(request)
    }
}
impl Default for TestDerivationPolyInteractor {
    fn default() -> Self {
        Self::new(do_derive_serially_looking_up_mnemonic_amongst_samples)
    }
}


#[async_trait::async_trait]
impl PolyFactorKeyDerivationInteractor for TestDerivationPolyInteractor {
    async fn derive(
        &self,
        request: PolyFactorKeyDerivationRequest,
    ) -> Result<KeyDerivationResponse> {
        let pairs_result: Result<
            IndexMap<
                FactorSourceIDFromHash,
                IndexSet<HierarchicalDeterministicFactorInstance>,
            >,
        > = request
            .per_factor_source
            .into_iter()
            .map(|(k, r)| {
                let instances = self.derive(r);
                instances.map(|i| (k, i))
            })
            .collect();
        let pairs = pairs_result?;
        Ok(KeyDerivationResponse::new(pairs))
    }
}

#[derive(Debug)]
pub(crate) struct TestDerivationMonoInteractor {
    handle: fn(
        MonoFactorKeyDerivationRequest,
    )
        -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>,
}
impl TestDerivationMonoInteractor {
    pub(crate) fn new(
        handle: fn(
            MonoFactorKeyDerivationRequest,
        )
            -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>,
    ) -> Self {
        Self { handle }
    }
    pub(crate) fn fail() -> Self {
        Self::new(|_| Err(CommonError::Unknown))
    }
    fn derive(
        &self,
        request: MonoFactorKeyDerivationRequest,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        (self.handle)(request)
    }
}
impl Default for TestDerivationMonoInteractor {
    fn default() -> Self {
        Self::new(do_derive_serially_looking_up_mnemonic_amongst_samples)
    }
}

#[async_trait::async_trait]
impl MonoFactorKeyDerivationInteractor for TestDerivationMonoInteractor {
    async fn derive(
        &self,
        request: MonoFactorKeyDerivationRequest,
    ) -> Result<KeyDerivationResponse> {
        let instances = self.derive(request.clone())?;
        Ok(KeyDerivationResponse::new(IndexMap::just((
            request.factor_source_id,
            instances,
        ))))
    }
}

impl KeysCollector {
    pub(crate) fn new_test_with_factor_sources(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        derivation_paths: impl IntoIterator<
            Item = (FactorSourceIDFromHash, IndexSet<DerivationPath>),
        >,
    ) -> Self {
        Self::new(
            all_factor_sources_in_profile,
            derivation_paths
                .into_iter()
                .collect::<IndexMap<FactorSourceIDFromHash, IndexSet<DerivationPath>>>(),
            Arc::new(TestDerivationInteractors::default()),
        )
        .unwrap()
    }

    pub(crate) fn new_test(
        derivation_paths: impl IntoIterator<
            Item = (FactorSourceIDFromHash, IndexSet<DerivationPath>),
        >,
    ) -> Self {
        Self::new_test_with_factor_sources(
            FactorSource::sample_all(),
            derivation_paths,
        )
    }

    pub(crate) fn with(
        factor_source: &FactorSource,
        network_id: NetworkID,
        key_kind: CAP26KeyKind,
        entity_kind: CAP26EntityKind,
        key_space: KeySpace,
    ) -> Self {
        let indices = StatelessDummyIndices;
        let path = indices.next_derivation_path(
            network_id,
            key_kind,
            entity_kind,
            key_space,
        );
        Self::new_test_with_factor_sources(
            [factor_source.clone()],
            [(factor_source.id_from_hash(), IndexSet::just(path))],
        )
    }
}
