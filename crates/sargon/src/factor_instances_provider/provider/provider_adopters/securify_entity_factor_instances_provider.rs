use std::collections::HashSet;

use crate::prelude::*;

pub struct SecurifyEntityFactorInstancesProvider;
impl SecurifyEntityFactorInstancesProvider {
    /// Reads FactorInstances for every `factor_source` in matrix_of_factor_sources
    /// on `network_id` of kind `account_mfa`,
    /// meaning `(EntityKind::Account, KeyKind::TransactionSigning, KeySpace::Securified)`,
    /// from cache, if any, otherwise derives more of that kind AND other kinds:
    /// identity_veci, account_veci, identity_mfa
    /// and saves into the cache and returns a collection of instances, per factor source,
    /// split into factor instance to use directly and factor instances which was cached, into
    /// the mutable `cache` parameter.
    ///
    /// We are always reading from the beginning of each FactorInstance collection in the cache,
    /// and we are always appending to the end.
    pub async fn for_account_mfa(
        cache_client: Arc<FactorInstancesCacheClient>,
        profile: Arc<Profile>,
        matrix_of_factor_sources: MatrixOfFactorSources,
        account_addresses: IndexSet<AccountAddress>,
        interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(InstancesInCacheConsumer, FactorInstancesProviderOutcome)>
    {
        Self::for_entity_mfa::<AccountAddress>(
            cache_client,
            profile,
            matrix_of_factor_sources,
            account_addresses,
            interactors,
        )
        .await
    }

    /// Reads FactorInstances for every `factor_source` in matrix_of_factor_sources
    /// on `network_id` of kind `identity_mfa`,
    /// meaning `(EntityKind::Identity, KeyKind::TransactionSigning, KeySpace::Securified)`,
    /// from cache, if any, otherwise derives more of that kind AND other kinds:
    /// identity_veci, account_veci, account_mfa
    /// and saves into the cache and returns a collection of instances, per factor source,
    /// split into factor instance to use directly and factor instances which was cached, into
    /// the mutable `cache` parameter.
    ///
    /// We are always reading from the beginning of each FactorInstance collection in the cache,
    /// and we are always appending to the end.
    pub async fn for_persona_mfa(
        cache_client: Arc<FactorInstancesCacheClient>,
        profile: Arc<Profile>,
        matrix_of_factor_sources: MatrixOfFactorSources,
        persona_addresses: IndexSet<IdentityAddress>,
        interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(InstancesInCacheConsumer, FactorInstancesProviderOutcome)>
    {
        Self::for_entity_mfa::<IdentityAddress>(
            cache_client,
            profile,
            matrix_of_factor_sources,
            persona_addresses,
            interactors,
        )
        .await
    }

    /// Reads FactorInstances for every `factor_source` in matrix_of_factor_sources
    /// on `network_id` of kind `account_mfa` or `identity_mfa` depending on Entity kind,
    /// meaning `(EntityKind::_, KeyKind::TransactionSigning, KeySpace::Securified)`,
    /// from cache, if any, otherwise derives more of that kind AND other kinds:
    /// identity_veci, account_veci, identity_mfa/account_mfa
    /// and saves into the cache and returns a collection of instances, per factor source,
    /// split into factor instance to use directly and factor instances which was cached, into
    /// the mutable `cache` parameter.
    ///
    /// We are always reading from the beginning of each FactorInstance collection in the cache,
    /// and we are always appending to the end.
    pub async fn for_entity_mfa<A: IsEntityAddress>(
        cache_client: Arc<FactorInstancesCacheClient>,
        profile: Arc<Profile>,
        matrix_of_factor_sources: MatrixOfFactorSources,
        addresses_of_entities: IndexSet<A>,
        interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<(InstancesInCacheConsumer, FactorInstancesProviderOutcome)>
    {
        let factor_sources_to_use = matrix_of_factor_sources
            .all_factors()
            .into_iter()
            .map(|x| x.to_owned())
            .collect::<IndexSet<FactorSource>>();

        let ids_of_factor_sources_in_profile =
            IndexSet::<FactorSourceIDFromHash>::from_iter(
                profile.factor_sources.iter().map(|f| f.id_from_hash()),
            );

        assert!(
            ids_of_factor_sources_in_profile.is_superset(
                &factor_sources_to_use
                    .iter()
                    .map(|f| f.id_from_hash())
                    .collect::<IndexSet<FactorSourceIDFromHash>>()
            ),
            "Missing FactorSources"
        );

        assert!(!addresses_of_entities.is_empty(), "No entities");

        assert!(
            addresses_of_entities
                .iter()
                .all(|a| profile.contains_entity_by_address::<A>(a)),
            "unknown entity"
        );

        let network_id = addresses_of_entities.first().unwrap().network_id();
        assert!(
            addresses_of_entities
                .iter()
                .all(|a| a.network_id() == network_id),
            "wrong network"
        );

        let entity_kind = A::entity_kind();

        let provider = FactorInstancesProvider::new(
            DerivatinPurpose::securify_entities(entity_kind),
            network_id,
            factor_sources_to_use,
            profile,
            cache_client,
            interactors,
        );

        let (instances_in_cache_consumer, outcome) = provider
            .provide(QuantifiedDerivationPreset::new(
                DerivationPreset::mfa_entity_kind(entity_kind),
                addresses_of_entities.len(),
            ))
            .await?;

        Ok((instances_in_cache_consumer, outcome.into()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurifyEntityFactorInstancesProvider;

    #[should_panic]
    #[actix_rt::test]
    async fn mfa_panics_if_entities_empty() {
        let fs = FactorSource::sample_at(0);
        let a = Account::sample();
        let cache_client = FactorInstancesCacheClient::in_memory();

        let _ = SUT::for_account_mfa(
            Arc::new(cache_client),
            Arc::new(Profile::sample_from([fs.clone()], [&a], [])),
            MatrixOfFactorSources::new(
                PrimaryRoleWithFactorSources::override_only([fs.clone()])
                    .unwrap(),
                RecoveryRoleWithFactorSources::override_only([fs.clone()])
                    .unwrap(),
                ConfirmationRoleWithFactorSources::override_only([fs.clone()])
                    .unwrap(),
                FactorRolesValidation::Skip, /* TODO: MFA-Rules: change to `Validate` */
            )
            .unwrap(),
            IndexSet::<AccountAddress>::new(), // <---- EMPTY => should_panic
            Arc::new(TestDerivationInteractors::default()),
        )
        .await
        .unwrap();
    }

    #[should_panic]
    #[actix_rt::test]
    async fn mfa_panics_if_entity_unknown() {
        let fs = FactorSource::sample_at(0);
        let a = Account::sample();
        let cache_client = FactorInstancesCacheClient::in_memory();

        let _ = SUT::for_account_mfa(
            Arc::new(cache_client),
            Arc::new(Profile::sample_from([fs.clone()], [&a], [])),
            MatrixOfFactorSources::new(
                PrimaryRoleWithFactorSources::override_only([fs.clone()])
                    .unwrap(),
                RecoveryRoleWithFactorSources::override_only([fs.clone()])
                    .unwrap(),
                ConfirmationRoleWithFactorSources::override_only([fs.clone()])
                    .unwrap(),
                FactorRolesValidation::Skip, /* TODO: MFA-Rules: change to `Validate` */
            )
            .unwrap(),
            IndexSet::just(Account::sample_other().address()), // <---- unknown => should_panic
            Arc::new(TestDerivationInteractors::default()),
        )
        .await
        .unwrap();
    }

    #[should_panic(expected = "Missing FactorSources")]
    #[actix_rt::test]
    async fn mfa_panics_if_factor_source_missing() {
        let fs = FactorSource::sample_at(0);
        let network = NetworkID::Mainnet;

        let mainnet_account = Account::new(HDFactorInstanceTransactionSigning::new(HierarchicalDeterministicFactorInstance::new_for_entity_on_network(
            network,
            fs.id_from_hash(),
            CAP26EntityKind::Account,
            Hardened::Unsecurified(
                UnsecurifiedHardened::try_from(0u32).unwrap(),
            ),
        )).unwrap(), DisplayName::sample(), AppearanceID::sample());

        let profile = Profile::sample_from(
            [], // <---- missing factor source => should_panic
            [&mainnet_account],
            [],
        );
        let cache_client = FactorInstancesCacheClient::in_memory();

        let _ = SUT::for_account_mfa(
            Arc::new(cache_client),
            Arc::new(profile),
            MatrixOfFactorSources::new(
                PrimaryRoleWithFactorSources::override_only([fs.clone()])
                    .unwrap(),
                RecoveryRoleWithFactorSources::override_only([fs.clone()])
                    .unwrap(),
                ConfirmationRoleWithFactorSources::override_only([fs.clone()])
                    .unwrap(),
                FactorRolesValidation::Skip, /* TODO: MFA-Rules: change to `Validate` */
            )
            .unwrap(),
            IndexSet::from_iter([mainnet_account.address()]),
            Arc::new(TestDerivationInteractors::default()),
        )
        .await
        .unwrap();
    }

    #[should_panic]
    #[actix_rt::test]
    async fn mfa_panics_if_wrong_network() {
        let fs = FactorSource::sample_at(0);
        let network = NetworkID::Mainnet;

        let mainnet_account = Account::new(HDFactorInstanceTransactionSigning::new(HierarchicalDeterministicFactorInstance::new_for_entity_on_network(
            network,
            fs.id_from_hash(),
            CAP26EntityKind::Account,
            Hardened::Unsecurified(
                UnsecurifiedHardened::try_from(0u32).unwrap(),
            ),
        )).unwrap(), DisplayName::sample(), AppearanceID::sample());

        let network = NetworkID::Stokenet;

        let stokenet_account = Account::new(HDFactorInstanceTransactionSigning::new(HierarchicalDeterministicFactorInstance::new_for_entity_on_network(
            network,
            fs.id_from_hash(),
            CAP26EntityKind::Account,
            Hardened::Unsecurified(
                UnsecurifiedHardened::try_from(0u32).unwrap(),
            ),
        )).unwrap(), DisplayName::sample(), AppearanceID::sample());

        let profile = Profile::sample_from(
            [fs.clone()],
            [&mainnet_account, &stokenet_account],
            [],
        );

        assert_eq!(profile.networks.len(), 2);
        let cache_client = FactorInstancesCacheClient::in_memory();

        let _ = SUT::for_account_mfa(
            Arc::new(cache_client),
            Arc::new(profile),
            MatrixOfFactorSources::new(
                PrimaryRoleWithFactorSources::override_only([fs.clone()])
                    .unwrap(),
                RecoveryRoleWithFactorSources::override_only([fs.clone()])
                    .unwrap(),
                ConfirmationRoleWithFactorSources::override_only([fs.clone()])
                    .unwrap(),
                FactorRolesValidation::Skip, /* TODO: MFA-Rules: change to `Validate` */
            )
            .unwrap(),
            IndexSet::from_iter([
                mainnet_account.address(),
                stokenet_account.address(),
            ]), // <---- wrong network => should_panic
            Arc::new(TestDerivationInteractors::default()),
        )
        .await
        .unwrap();
    }

    #[actix_rt::test]
    async fn securify_accounts_and_personas_with_override_factor() {
        // this is mostly a soundness test for the two functions `for_persona_mfa` and `for_account_mfa`
        // using `os` to create a profile, and BDFS because I'm lazy.
        // We might in fact remove `for_persona_mfa` and `for_account_mfa`
        // and only use the `for_entity_mfa` function... but we have these to get code coverage.
        let (os, bdfs) = SargonOS::with_bdfs().await;

        let (batman, derivation_outcome) = os
            .create_and_save_new_mainnet_persona_with_derivation_outcome(
                "Batman",
            )
            .await
            .unwrap();
        assert!(derivation_outcome.debug_was_derived.is_empty());

        let (alice, derivation_outcome) = os
            .create_and_save_new_mainnet_account_with_derivation_outcome(
                "alice",
            )
            .await
            .unwrap();
        assert!(derivation_outcome.debug_was_derived.is_empty());

        let matrix_0 = MatrixOfFactorSources::new(
            PrimaryRoleWithFactorSources::override_only([bdfs.clone()])
                .unwrap(),
            RecoveryRoleWithFactorSources::override_only([bdfs.clone()])
                .unwrap(),
            ConfirmationRoleWithFactorSources::override_only([bdfs.clone()])
                .unwrap(),
            FactorRolesValidation::Skip, /* TODO: MFA-Rules: change to `Validate` */
        )
        .unwrap();

        let cache_client = Arc::new(os.clients.factor_instances_cache.clone());
        let profile = Arc::new(os.profile().unwrap());
        let derivation_interactors = os.keys_derivation_interactors();

        let (instances_in_cache_consumer, outcome) = SUT::for_account_mfa(
            cache_client.clone(),
            profile,
            matrix_0.clone(),
            IndexSet::just(alice.address()),
            derivation_interactors.clone(),
        )
        .await
        .unwrap();

        // don't forget to consume
        instances_in_cache_consumer.consume().await.unwrap();
        let outcome = outcome.per_factor.get(&bdfs.id_from_hash()).unwrap();
        assert_eq!(outcome.to_use_directly.len(), 1);

        let profile = Arc::new(os.profile().unwrap());
        let (instances_in_cache_consumer, outcome) = SUT::for_persona_mfa(
            cache_client.clone(),
            profile,
            matrix_0.clone(),
            IndexSet::just(batman.address()),
            derivation_interactors.clone(),
        )
        .await
        .unwrap();

        // don't forget to consume
        instances_in_cache_consumer.consume().await.unwrap();
        let outcome = outcome.per_factor.get(&bdfs.id_from_hash()).unwrap();
        assert_eq!(outcome.to_use_directly.len(), 1);
    }
}
