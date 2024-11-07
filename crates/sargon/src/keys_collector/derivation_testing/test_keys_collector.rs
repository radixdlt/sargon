#![cfg(test)]
#![allow(unused)]

use crate::prelude::*;

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

    pub(crate) fn mono_and_poly_with_extra_mnemonics(
        extra_mnemonics: IndexMap<
            FactorSourceIDFromHash,
            MnemonicWithPassphrase,
        >,
    ) -> Self {
        Self::new(
            TestDerivationMonoAndPolyInteractor::new(
                false,
                extra_mnemonics.clone(),
            ),
            TestDerivationMonoAndPolyInteractor::new(
                false,
                extra_mnemonics.clone(),
            ),
        )
    }
}

impl TestDerivationInteractors {
    pub(crate) fn fail() -> Self {
        Self::new(
            TestDerivationMonoAndPolyInteractor::fail(),
            TestDerivationMonoAndPolyInteractor::fail(),
        )
    }
}
impl Default for TestDerivationInteractors {
    fn default() -> Self {
        Self::new(
            TestDerivationMonoAndPolyInteractor::default(),
            TestDerivationMonoAndPolyInteractor::default(),
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

#[derive(Debug, Default)]
pub(crate) struct TestDerivationMonoAndPolyInteractor {
    pub always_fail: bool,
    pub extra_mnemonics:
        IndexMap<FactorSourceIDFromHash, MnemonicWithPassphrase>,
}
impl TestDerivationMonoAndPolyInteractor {
    pub(crate) fn new(
        always_fail: bool,
        extra_mnemonics: IndexMap<
            FactorSourceIDFromHash,
            MnemonicWithPassphrase,
        >,
    ) -> Self {
        Self {
            always_fail,
            extra_mnemonics,
        }
    }
    pub(crate) fn fail() -> Self {
        Self::new(true, IndexMap::default())
    }

    async fn do_derive(
        &self,
        request: MonoFactorKeyDerivationRequest,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        if self.always_fail {
            return Err(CommonError::Unknown);
        }
        do_derive_serially_looking_up_mnemonic_amongst_samples(
            request,
            self.extra_mnemonics.clone(),
        )
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

// #[derive(Debug)]
// pub(crate) struct TestDerivationMonoInteractor {
//     handle: fn(
//         MonoFactorKeyDerivationRequest,
//     )
//         -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>,
// }
// impl TestDerivationMonoInteractor {
//     pub(crate) fn new(
//         handle: fn(
//             MonoFactorKeyDerivationRequest,
//         )
//             -> Result<IndexSet<HierarchicalDeterministicFactorInstance>>,
//     ) -> Self {
//         Self { handle }
//     }
//     pub(crate) fn fail() -> Self {
//         Self::new(|_| Err(CommonError::Unknown))
//     }
//     async fn do_derive(
//         &self,
//         request: MonoFactorKeyDerivationRequest,
//     ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
//         (self.handle)(request)
//     }
// }
// impl Default for TestDerivationMonoInteractor {
//     fn default() -> Self {
//         Self::new(do_derive_serially_looking_up_mnemonic_amongst_samples)
//     }
// }

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
