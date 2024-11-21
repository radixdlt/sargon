use crate::prelude::*;

/// A type for preprocessing of input data for the `KeysCollector`.
///
/// It analyzes the derivation paths and factor sources to determine
/// which factor sources are used to derive keys, and in what order.
pub(crate) struct KeysCollectorPreprocessor {
    derivation_paths:
        IndexMap<FactorSourceIDFromHash, IndexSet<DerivationPath>>,
}

impl KeysCollectorPreprocessor {
    pub(crate) fn new(
        derivation_paths: IndexMap<
            FactorSourceIDFromHash,
            IndexSet<DerivationPath>,
        >,
    ) -> Self {
        Self { derivation_paths }
    }

    pub(crate) fn preprocess(
        &self,
        all_factor_sources_in_profile: IndexSet<FactorSource>,
    ) -> Result<(KeysCollectorState, IndexSet<FactorSourcesOfKind>)> {
        let all_factor_sources_in_profile = all_factor_sources_in_profile
            .into_iter()
            .map(|f| (f.id_from_hash(), f))
            .collect::<HashMap<FactorSourceIDFromHash, FactorSource>>();

        let unsorted = self
            .derivation_paths
            .clone()
            .keys()
            .map(|id| {
                all_factor_sources_in_profile.get(id).cloned().ok_or(
                    CommonError::ProfileDoesNotContainFactorSourceWithID {
                        bad_value: (*id).into(),
                    },
                )
            })
            .collect::<Result<HashSet<_>>>()?;

        let factor_sources_of_kind = sort_group_factors(unsorted);
        let state = KeysCollectorState::new(self.derivation_paths.clone());

        Ok((state, factor_sources_of_kind))
    }
}
