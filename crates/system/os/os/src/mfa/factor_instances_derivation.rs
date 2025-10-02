use crate::prelude::*;

impl SargonOS {
    /// Derive the factor instances for the given matrix factors and rola factor if needed.
    pub async fn derive_factor_instances(
        &self,
        entity: AccountOrPersona,
        matrix_factors: HashSet<&FactorSource>,
        rola_factor: Option<FactorSource>,
    ) -> Result<IndexMap<FactorSourceIDFromHash, FactorInstances>> {
        let profile_snapshot = self.profile()?;
        let network_id = profile_snapshot.current_network_id();
        let key_derivation_interactors = self.keys_derivation_interactor();

        // Used to determine the next derivation index
        let index_assigner =
        NextDerivationEntityIndexProfileAnalyzingAssigner::new(
            network_id,
            Some(Arc::new(profile_snapshot)),
        );

        // Collects all FactorSources used for derivation
        let mut all_factor_sources = matrix_factors
            .clone()
            .into_iter()
            .map(|x| x.to_owned())
            .collect::<IndexSet<FactorSource>>();

        // 1. Create per factor derivation paths

        // Collects all derivation paths per factor.
        // Usually all factors will have only one derivation path, even if it is used in more than one place in the security matrix.
        // The only scenario when there could be 2 paths for the factor, is when the same factor is used in matrix and as rola factor.
        let mut per_factor_paths =
            IndexMap::<FactorSourceIDFromHash, IndexSet<DerivationPath>>::new();

        if let Some(rola_factor) = rola_factor {
            all_factor_sources.insert(rola_factor.clone());

            let rola_idx_agnostic_path =
                DerivationPreset::rola_entity_kind(entity.get_entity_kind())
                    .index_agnostic_path_on_network(network_id);

            let default_index_rola_index =
                HDPathComponent::from_local_key_space(
                    0u32,
                    rola_idx_agnostic_path.key_space,
                )?;

            let rola_derivation_path = index_assigner
                .next(rola_factor.id_from_hash(), rola_idx_agnostic_path)
                .map(|index| {
                    DerivationPath::from_index_agnostic_path_and_component(
                        rola_idx_agnostic_path,
                        index.unwrap_or(default_index_rola_index),
                    )
                })?;

            per_factor_paths.append_or_insert_element_to(
                rola_factor.id_from_hash(),
                rola_derivation_path,
            );
        };

        let mfa_idx_agnostic_path =
            DerivationPreset::mfa_entity_kind(entity.get_entity_kind())
                .index_agnostic_path_on_network(network_id);

        let default_index_mfa_index = HDPathComponent::from_local_key_space(
            0u32,
            mfa_idx_agnostic_path.key_space,
        )?;

        let matrix_paths = matrix_factors
        .into_iter()
        .map(|factor| {
            let path = index_assigner
            .next(
                factor.id_from_hash(),
                mfa_idx_agnostic_path
            )
            .map(|index| {
                DerivationPath::from_index_agnostic_path_and_component(
                    mfa_idx_agnostic_path,
                    index.unwrap_or(default_index_mfa_index),
                 )
            })?;

            Ok((factor.id_from_hash(), path))
        })
        .collect::<Result<IndexMap<FactorSourceIDFromHash, DerivationPath>>>()?;

        for (id, path) in matrix_paths {
            per_factor_paths.append_or_insert_element_to(id, path);
        }

        // 2. Setup keys collector and derive the keys
        let collector = KeysCollector::new(
            all_factor_sources,
            per_factor_paths.clone(),
            key_derivation_interactors,
            DerivationPurpose::SecurifyingAccount,
        )?;

        let keys_output = collector.collect_keys().await?;

        Ok(
            keys_output
            .factors_by_source
            .into_iter()
            .map(|(id, factors)| {
                let instances = FactorInstances::from(factors);
                (id, instances)
            })
            .collect::<IndexMap<FactorSourceIDFromHash, FactorInstances>>()
        )
    }
}
