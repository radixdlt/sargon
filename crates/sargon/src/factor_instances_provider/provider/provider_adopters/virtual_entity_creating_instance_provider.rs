use crate::prelude::*;

/// Uses a `FactorInstancesProvider` provide a VECI for a new virtual entity.
pub struct VirtualEntityCreatingInstanceProvider;
impl VirtualEntityCreatingInstanceProvider {
    /// Uses a `FactorInstancesProvider` to provide a VECI for a new virtual entity.
    ///
    /// Reads FactorInstances for `factor_source` on `network_id` of kind `account_veci`,
    /// meaning `(EntityKind::Account, KeyKind::TransactionSigning, KeySpace::Unsecurified)`,
    /// from cache, if any, otherwise derives more of that kind AND other kinds:
    /// identity_veci, account_mfa, identity_mfa
    /// and saves into the cache and returns a collection of instances, split into
    /// factor instance to use directly and factor instances which was cached, into
    /// the mutable `cache` parameter.
    ///
    /// We are always reading from the beginning of each FactorInstance collection in the cache,
    /// and we are always appending to the end.
    pub async fn for_account_veci(
        cache_client: Arc<FactorInstancesCacheClient>,
        profile: Option<Arc<Profile>>,
        factor_source: FactorSource,
        network_id: NetworkID,
        interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Result<(
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        Self::for_entity_veci(
            CAP26EntityKind::Account,
            cache_client,
            profile,
            factor_source,
            network_id,
            interactor,
        )
        .await
    }

    /// Reads FactorInstances for `factor_source` on `network_id` of kind `persona_veci`,
    /// meaning `(EntityKind::Identity, KeyKind::TransactionSigning, KeySpace::Unsecurified)`,
    /// from cache, if any, otherwise derives more of that kind AND other kinds:
    /// account_veci, account_mfa, identity_mfa
    /// and saves into the cache and returns a collection of instances, split into
    /// factor instance to use directly and factor instances which was cached, into
    /// the mutable `cache` parameter.
    ///
    /// We are always reading from the beginning of each FactorInstance collection in the cache,
    /// and we are always appending to the end.
    pub async fn for_persona_veci(
        cache_client: Arc<FactorInstancesCacheClient>,
        profile: Option<Arc<Profile>>,
        factor_source: FactorSource,
        network_id: NetworkID,
        interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Result<(
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        Self::for_entity_veci(
            CAP26EntityKind::Identity,
            cache_client,
            profile,
            factor_source,
            network_id,
            interactor,
        )
        .await
    }

    /// Reads FactorInstances for `factor_source` on `network_id` of kind `account_veci`,
    /// meaning `(EntityKind::Account, KeyKind::TransactionSigning, KeySpace::Unsecurified)`,
    /// from cache, if any, otherwise derives more of that kind AND other kinds:
    /// identity_veci, account_mfa, identity_mfa
    /// and saves into the cache and returns a collection of instances, split into
    /// factor instance to use directly and factor instances which was cached, into
    /// the mutable `cache` parameter.
    ///
    /// We are always reading from the beginning of each FactorInstance collection in the cache,
    /// and we are always appending to the end.
    pub async fn for_entity_veci(
        entity_kind: CAP26EntityKind,
        cache_client: Arc<FactorInstancesCacheClient>,
        profile: Option<Arc<Profile>>,
        factor_source: FactorSource,
        network_id: NetworkID,
        interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Result<(
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        Self::for_many_entity_vecis(
            1,
            entity_kind,
            cache_client,
            profile,
            factor_source,
            network_id,
            interactor,
        )
        .await
    }

    /// Uses a `FactorInstancesProvider` to provide `count` many VECIs for new virtual entities.
    ///
    /// Reads FactorInstances for `factor_source` on `network_id` of kind `account_veci`,
    /// meaning `(EntityKind::Account, KeyKind::TransactionSigning, KeySpace::Unsecurified)`,
    /// from cache, if any, otherwise derives more of that kind AND other kinds:
    /// identity_veci, account_mfa, identity_mfa
    /// and saves into the cache and returns a collection of instances, split into
    /// factor instance to use directly and factor instances which was cached, into
    /// the mutable `cache` parameter.
    ///
    /// We are always reading from the beginning of each FactorInstance collection in the cache,
    /// and we are always appending to the end.
    pub async fn for_many_entity_vecis(
        count: usize,
        entity_kind: CAP26EntityKind,
        cache_client: Arc<FactorInstancesCacheClient>,
        profile: impl Into<Option<Arc<Profile>>>,
        factor_source: FactorSource,
        network_id: NetworkID,
        interactor: Arc<dyn KeyDerivationInteractor>,
    ) -> Result<(
        InstancesInCacheConsumer,
        FactorInstancesProviderOutcomeForFactor,
    )> {
        let provider = FactorInstancesProvider::new(
            network_id,
            IndexSet::just(factor_source.clone()),
            profile,
            cache_client,
            interactor,
        );
        let (instances_in_cache_consumer, outcome) = provider
            .provide(
                QuantifiedDerivationPreset::new(
                    DerivationPreset::veci_entity_kind(entity_kind),
                    count,
                ),
                KeysCollectionReason::new_for_creating(entity_kind),
            )
            .await?;

        let outcome = outcome
            .per_factor
            .get(&factor_source.id_from_hash())
            .cloned()
            .expect("Expected to have instances for the factor source");

        Ok((instances_in_cache_consumer, outcome.into()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[allow(clippy::upper_case_acronyms)]
    type SUT = VirtualEntityCreatingInstanceProvider;

    #[actix_rt::test]
    async fn cache_is_always_filled_persona_veci_then_after_all_used_we_start_over_at_zero_if_no_profile_is_used(
    ) {
        let network = NetworkID::Mainnet;
        let bdfs = FactorSource::sample();
        let cache_client = Arc::new(FactorInstancesCacheClient::in_memory());

        let (consumer, outcome) = SUT::for_persona_veci(
            cache_client.clone(),
            None,
            bdfs.clone(),
            network,
            Arc::new(TestDerivationInteractor::default()),
        )
        .await
        .unwrap();
        consumer.consume().await.unwrap();

        assert_eq!(outcome.factor_source_id, bdfs.id_from_hash());

        assert_eq!(outcome.debug_found_in_cache.len(), 0);

        assert_eq!(
            outcome.debug_was_cached.len(),
            DerivationPreset::all().len() * CACHE_FILLING_QUANTITY
        );

        assert_eq!(
            outcome.debug_was_derived.len(),
            DerivationPreset::all().len() * CACHE_FILLING_QUANTITY + 1
        );

        let instances_used_directly =
            outcome.to_use_directly.factor_instances();
        assert_eq!(instances_used_directly.len(), 1);
        let instances_used_directly = instances_used_directly.first().unwrap();

        assert_eq!(
            instances_used_directly.derivation_path().index(),
            HDPathComponent::from_local_key_space(
                0,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap()
        );

        cache_client
            .assert_is_full(network, bdfs.id_from_hash())
            .await;

        let cached = cache_client
            .peek_all_instances_of_factor_source(bdfs.id_from_hash())
            .await
            .unwrap()
            .unwrap();

        let persona_veci_paths = cached
            .clone()
            .get(
                &DerivationPreset::IdentityVeci
                    .index_agnostic_path_on_network(network),
            )
            .unwrap()
            .factor_instances()
            .into_iter()
            .map(|x| x.derivation_path())
            .collect_vec();

        assert_eq!(persona_veci_paths.len(), CACHE_FILLING_QUANTITY);

        assert!(persona_veci_paths.iter().all(|x| x.get_entity_kind()
            == CAP26EntityKind::Identity
            && x.network_id() == network
            && !x.key_space().is_securified()
            && x.get_key_kind() == CAP26KeyKind::TransactionSigning));

        let persona_veci_indices = persona_veci_paths
            .into_iter()
            .map(|x| x.index())
            .collect_vec();

        assert_eq!(
            persona_veci_indices.first().unwrap().clone(),
            HDPathComponent::from_local_key_space(
                1,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap()
        );

        assert_eq!(
            persona_veci_indices.last().unwrap().clone(),
            HDPathComponent::from_local_key_space(
                30,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap()
        );
    }

    #[actix_rt::test]
    async fn cache_is_always_filled_account_veci_then_after_all_used_we_start_over_at_zero_if_no_profile_is_used(
    ) {
        let network = NetworkID::Mainnet;
        let bdfs = FactorSource::sample();
        let cache_client = Arc::new(FactorInstancesCacheClient::in_memory());
        let (consumer, outcome) = SUT::for_account_veci(
            cache_client.clone(),
            None,
            bdfs.clone(),
            network,
            Arc::new(TestDerivationInteractor::default()),
        )
        .await
        .unwrap();
        consumer.consume().await.unwrap();

        assert_eq!(outcome.factor_source_id, bdfs.id_from_hash());

        assert_eq!(outcome.debug_found_in_cache.len(), 0);

        assert_eq!(
            outcome.debug_was_cached.len(),
            DerivationPreset::all().len() * CACHE_FILLING_QUANTITY
        );

        assert_eq!(
            outcome.debug_was_derived.len(),
            DerivationPreset::all().len() * CACHE_FILLING_QUANTITY + 1
        );

        let instances_used_directly =
            outcome.to_use_directly.factor_instances();
        assert_eq!(instances_used_directly.len(), 1);
        let instances_used_directly = instances_used_directly.first().unwrap();

        assert_eq!(
            instances_used_directly.derivation_entity_index(),
            HDPathComponent::from_local_key_space(
                0,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap()
        );

        cache_client
            .assert_is_full(network, bdfs.id_from_hash())
            .await;

        let cached = cache_client
            .peek_all_instances_of_factor_source(bdfs.id_from_hash())
            .await
            .unwrap()
            .unwrap();

        let account_veci_paths = cached
            .clone()
            .get(
                &DerivationPreset::AccountVeci
                    .index_agnostic_path_on_network(network),
            )
            .unwrap()
            .factor_instances()
            .into_iter()
            .map(|x| x.derivation_path())
            .collect_vec();

        assert_eq!(account_veci_paths.len(), CACHE_FILLING_QUANTITY);

        assert!(account_veci_paths.iter().all(|x| x.get_entity_kind()
            == CAP26EntityKind::Account
            && x.network_id() == network
            && x.key_space() == KeySpace::Unsecurified { is_hardened: true }
            && x.get_key_kind() == CAP26KeyKind::TransactionSigning));

        let account_veci_indices = account_veci_paths
            .into_iter()
            .map(|x| x.index())
            .collect_vec();

        assert_eq!(
            account_veci_indices.first().unwrap().clone(),
            HDPathComponent::from_local_key_space(
                1,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap()
        );

        assert_eq!(
            account_veci_indices.last().unwrap().clone(),
            HDPathComponent::from_local_key_space(
                30,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap()
        );

        let account_mfa_paths = cached
            .clone()
            .get(
                &DerivationPreset::AccountMfa
                    .index_agnostic_path_on_network(network),
            )
            .unwrap()
            .factor_instances()
            .into_iter()
            .map(|x| x.derivation_path())
            .collect_vec();

        assert!(account_mfa_paths.iter().all(|x| x.get_entity_kind()
            == CAP26EntityKind::Account
            && x.network_id() == network
            && x.key_space() == KeySpace::Securified
            && x.get_key_kind() == CAP26KeyKind::TransactionSigning));

        let account_mfa_indices = account_mfa_paths
            .into_iter()
            .map(|x| x.index())
            .collect_vec();

        assert_eq!(
            account_mfa_indices.first().unwrap().clone(),
            HDPathComponent::from_local_key_space(0, KeySpace::Securified)
                .unwrap()
        );

        assert_eq!(
            account_mfa_indices.last().unwrap().clone(),
            HDPathComponent::from_local_key_space(29, KeySpace::Securified)
                .unwrap()
        );

        let identity_mfa_paths = cached
            .clone()
            .get(
                &DerivationPreset::IdentityMfa
                    .index_agnostic_path_on_network(network),
            )
            .unwrap()
            .factor_instances()
            .into_iter()
            .map(|x| x.derivation_path())
            .collect_vec();

        assert!(identity_mfa_paths.iter().all(|x| x.get_entity_kind()
            == CAP26EntityKind::Identity
            && x.network_id() == network
            && x.key_space() == KeySpace::Securified
            && x.get_key_kind() == CAP26KeyKind::TransactionSigning));

        let identity_mfa_indices = identity_mfa_paths
            .into_iter()
            .map(|x| x.index())
            .collect_vec();

        assert_eq!(
            identity_mfa_indices.first().unwrap().clone(),
            HDPathComponent::from_local_key_space(0, KeySpace::Securified)
                .unwrap()
        );

        assert_eq!(
            identity_mfa_indices.last().unwrap().clone(),
            HDPathComponent::from_local_key_space(29, KeySpace::Securified)
                .unwrap()
        );

        let identity_veci_paths = cached
            .clone()
            .get(
                &DerivationPreset::IdentityVeci
                    .index_agnostic_path_on_network(network),
            )
            .unwrap()
            .factor_instances()
            .into_iter()
            .map(|x| x.derivation_path())
            .collect_vec();

        assert!(identity_veci_paths.iter().all(|x| x.get_entity_kind()
            == CAP26EntityKind::Identity
            && x.network_id() == network
            && x.key_space() == KeySpace::Unsecurified { is_hardened: true }
            && x.get_key_kind() == CAP26KeyKind::TransactionSigning));

        let identity_veci_indices = identity_veci_paths
            .into_iter()
            .map(|x| x.index())
            .collect_vec();

        assert_eq!(
            identity_veci_indices.first().unwrap().clone(),
            HDPathComponent::from_local_key_space(
                0,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap()
        );

        assert_eq!(
            identity_veci_indices.last().unwrap().clone(),
            HDPathComponent::from_local_key_space(
                29,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap()
        );

        // lets create another account (same network, same factor source)

        let (consumer, outcome) = SUT::for_account_veci(
            cache_client.clone(),
            None,
            bdfs.clone(),
            network,
            Arc::new(TestDerivationInteractor::default()),
        )
        .await
        .unwrap();
        consumer.consume().await.unwrap();

        assert_eq!(outcome.factor_source_id, bdfs.id_from_hash());
        assert_eq!(outcome.debug_found_in_cache.len(), 1); // This time we found in cache
        assert_eq!(outcome.debug_was_cached.len(), 0);
        assert_eq!(outcome.debug_was_derived.len(), 0);

        let instances_used_directly =
            outcome.to_use_directly.factor_instances();
        assert_eq!(instances_used_directly.len(), 1);
        let instances_used_directly = instances_used_directly.first().unwrap();

        assert_eq!(
            instances_used_directly.derivation_entity_index(),
            HDPathComponent::from_local_key_space(
                1,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap()
        );

        let is_cache_full =
            cache_client.is_full(network, bdfs.id_from_hash()).await;
        assert!(!is_cache_full); // not full anymore, since we just used a veci

        let cached = cache_client
            .peek_all_instances_of_factor_source(bdfs.id_from_hash())
            .await
            .unwrap()
            .unwrap();

        let account_veci_paths = cached
            .clone()
            .get(
                &DerivationPreset::AccountVeci
                    .index_agnostic_path_on_network(network),
            )
            .unwrap()
            .factor_instances()
            .into_iter()
            .map(|x| x.derivation_path())
            .collect_vec();

        assert_eq!(account_veci_paths.len(), CACHE_FILLING_QUANTITY - 1);

        assert!(account_veci_paths.iter().all(|x| x.get_entity_kind()
            == CAP26EntityKind::Account
            && x.network_id() == network
            && x.key_space() == KeySpace::Unsecurified { is_hardened: true }
            && x.get_key_kind() == CAP26KeyKind::TransactionSigning));

        let account_veci_indices = account_veci_paths
            .into_iter()
            .map(|x| x.index())
            .collect_vec();

        assert_eq!(
            account_veci_indices.first().unwrap().clone(),
            HDPathComponent::from_local_key_space(
                2,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap() // first is not `1` anymore
        );

        assert_eq!(
            account_veci_indices.last().unwrap().clone(),
            HDPathComponent::from_local_key_space(
                30,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap()
        );

        // create 29 more accounts, then we should be able to crate one more which should ONLY derive
        // more instances for ACCOUNT VECI, and not Identity Veci, Identity MFA and Account MFA, since that is
        // not needed.
        let (consumer, outcome) = SUT::for_many_entity_vecis(
            29,
            CAP26EntityKind::Account,
            cache_client.clone(),
            None,
            bdfs.clone(),
            network,
            Arc::new(TestDerivationInteractor::default()),
        )
        .await
        .unwrap();
        consumer.consume().await.unwrap();

        assert_eq!(outcome.factor_source_id, bdfs.id_from_hash());

        assert_eq!(outcome.debug_found_in_cache.len(), 29);
        assert_eq!(outcome.debug_was_cached.len(), 0);
        assert_eq!(outcome.debug_was_derived.len(), 0);

        let cached = cache_client
            .peek_all_instances_of_factor_source(bdfs.id_from_hash())
            .await
            .unwrap()
            .unwrap();

        assert!(
            cached
                .get(
                    &DerivationPreset::AccountVeci
                        .index_agnostic_path_on_network(network)
                )
                .is_none(),
            "should have used the last instance..."
        );

        // Great, now lets create one more account, and this time we should derive more instances for
        // it. We should derive 31 instances, 30 for account veci to cache and 1 to use directly.
        // we should NOT derive more instances for Identity Veci, Identity MFA and Account MFA, since
        // that cache is already full.
        let (consumer, outcome) = SUT::for_account_veci(
            cache_client.clone(),
            None,
            bdfs.clone(),
            network,
            Arc::new(TestDerivationInteractor::default()),
        )
        .await
        .unwrap();
        consumer.consume().await.unwrap();

        assert_eq!(outcome.factor_source_id, bdfs.id_from_hash());

        assert_eq!(outcome.debug_found_in_cache.len(), 0);
        assert_eq!(outcome.debug_was_cached.len(), CACHE_FILLING_QUANTITY); // ONLY 30, not 120...
        assert_eq!(outcome.debug_was_derived.len(), CACHE_FILLING_QUANTITY + 1);

        let instances_used_directly =
            outcome.to_use_directly.factor_instances();
        assert_eq!(instances_used_directly.len(), 1);
        let instances_used_directly = instances_used_directly.first().unwrap();

        assert_eq!(
            instances_used_directly.derivation_entity_index(),
            HDPathComponent::from_local_key_space(
                0,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .unwrap() // IMPORTANT! Index 0 is used again! Why?! Well because are not using a Profile here, and we are not eagerly filling cache just before we are using the last index.
        );
    }
}
