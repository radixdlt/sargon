#![cfg(test)]
#![allow(unused)]

use network::prelude::NetworkID;

use crate::prelude::*;

impl KeysCollector {
    pub(crate) fn new_test_with_factor_sources(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        derivation_paths: impl IntoIterator<
            Item = (FactorSourceIDFromHash, IndexSet<DerivationPath>),
        >,
        derivation_purpose: DerivationPurpose,
    ) -> Self {
        Self::new(
            all_factor_sources_in_profile,
            derivation_paths
                .into_iter()
                .collect::<IndexMap<FactorSourceIDFromHash, IndexSet<DerivationPath>>>(),
            Arc::new(TestDerivationInteractor::default()),
            derivation_purpose
        )
        .unwrap()
    }

    pub(crate) fn new_test(
        derivation_paths: impl IntoIterator<
            Item = (FactorSourceIDFromHash, IndexSet<DerivationPath>),
        >,
        derivation_purpose: DerivationPurpose,
    ) -> Self {
        Self::new_test_with_factor_sources(
            FactorSource::sample_all(),
            derivation_paths,
            derivation_purpose,
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
            DerivationPurpose::PreDerivingKeys,
        )
    }
}
