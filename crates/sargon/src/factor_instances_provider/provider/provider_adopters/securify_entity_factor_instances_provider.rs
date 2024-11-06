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
        cache: &mut FactorInstancesCache,
        profile: Profile,
        matrix_of_factor_sources: MatrixOfFactorSources,
        account_addresses: IndexSet<AccountAddress>,
        interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<FactorInstancesProviderOutcome> {
        Self::for_entity_mfa::<Account>(
            cache,
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
        cache: &mut FactorInstancesCache,
        profile: Profile,
        matrix_of_factor_sources: MatrixOfFactorSources,
        persona_addresses: IndexSet<IdentityAddress>,
        interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<FactorInstancesProviderOutcome> {
        Self::for_entity_mfa::<Persona>(
            cache,
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
    pub async fn for_entity_mfa<E: IsEntity>(
        cache: &mut FactorInstancesCache,
        profile: Profile,
        matrix_of_factor_sources: MatrixOfFactorSources,
        addresses_of_entities: IndexSet<E::Address>,
        interactors: Arc<dyn KeysDerivationInteractors>,
    ) -> Result<FactorInstancesProviderOutcome> {
        let factor_sources_to_use = matrix_of_factor_sources
            .all_factors()
            .into_iter()
            .map(|x| x.to_owned())
            .collect::<IndexSet<FactorSource>>();
        let factor_sources =
            IndexSet::<FactorSource>::from_iter(profile.factor_sources.iter());
        assert!(
            factor_sources.is_superset(&factor_sources_to_use),
            "Missing FactorSources"
        );
        assert!(!addresses_of_entities.is_empty(), "No entities");
        assert!(
            addresses_of_entities
                .iter()
                .all(|e| profile.contains_entity_by_address::<E>(e)),
            "unknown entity"
        );
        let network_id = addresses_of_entities.first().unwrap().network_id();
        assert!(
            addresses_of_entities
                .iter()
                .all(|a| a.network_id() == network_id),
            "wrong network"
        );

        let entity_kind = E::entity_kind();

        let provider = FactorInstancesProvider::new(
            network_id,
            factor_sources_to_use,
            profile,
            cache,
            interactors,
        );

        let outcome = provider
            .provide(QuantifiedDerivationPreset::new(
                DerivationPreset::mfa_entity_kind(entity_kind),
                addresses_of_entities.len(),
            ))
            .await?;

        Ok(outcome.into())
    }
}
/*
#[cfg(test)]
mod tests {

    use std::ops::Add;

    use crate::{factor_instances_provider::provider::test_sargon_os::SargonOS, prelude::*};

    type Sut = SecurifyEntityFactorInstancesProvider;

    #[should_panic]
    #[actix_rt::test]
    async fn mfa_panics_if_entities_empty() {
        let fs = FactorSource::fs0();
        let a = Account::sample_unsecurified();
        let _ = Sut::for_account_mfa(
            &mut FactorInstancesCache::default(),
            Profile::new([fs.clone()], [&a], []),
            MatrixOfFactorSources::new([], 1, [fs.clone()]),
            IndexSet::new(), // <---- EMPTY => should_panic
            Arc::new(TestDerivationInteractors::default()),
        )
        .await
        .unwrap();
    }

    #[should_panic]
    #[actix_rt::test]
    async fn mfa_panics_if_entity_unknown() {
        let fs = FactorSource::fs0();
        let a = Account::sample_unsecurified();
        let _ = Sut::for_account_mfa(
            &mut FactorInstancesCache::default(),
            Profile::new([fs.clone()], [&a], []),
            MatrixOfFactorSources::new([], 1, [fs.clone()]),
            IndexSet::just(Account::a1().entity_address()), // <---- unknown => should_panic
            Arc::new(TestDerivationInteractors::default()),
        )
        .await
        .unwrap();
    }

    #[should_panic]
    #[actix_rt::test]
    async fn mfa_panics_if_wrong_network() {
        let fs = FactorSource::fs0();
        let network = NetworkID::Mainnet;
        let mainnet_account = Account::unsecurified_on_network(
            "main",
            network,
            HierarchicalDeterministicFactorInstance::tx_on_network(
                CAP26EntityKind::Account,
                network,
                HDPathComponent::unsecurified_hardening_base_index(0),
                fs.factor_source_id(),
            ),
        );
        let network = NetworkID::Stokenet;
        let stokenet_account = Account::unsecurified_on_network(
            "stoknet",
            network,
            HierarchicalDeterministicFactorInstance::tx_on_network(
                CAP26EntityKind::Account,
                network,
                HDPathComponent::unsecurified_hardening_base_index(0),
                fs.factor_source_id(),
            ),
        );
        let profile = Profile::new([fs.clone()], [&mainnet_account, &stokenet_account], []);
        assert_eq!(profile.networks.len(), 2);
        let _ = Sut::for_account_mfa(
            &mut FactorInstancesCache::default(),
            profile,
            MatrixOfFactorSources::new([], 1, [fs.clone()]),
            IndexSet::from_iter([
                mainnet_account.entity_address(),
                stokenet_account.entity_address(),
            ]), // <---- wrong network => should_panic
            Arc::new(TestDerivationInteractors::default()),
        )
        .await
        .unwrap();
    }

    #[actix_rt::test]
    async fn securify_accounts_and_personas_with_override_factor() {
        // this is mostly a soundness test for the two functions `for_persona_mfa` and `for_account_mfa`
        // using `os` because I'm lazy. We might in fact remove `for_persona_mfa` and `for_account_mfa`
        // and only use the `for_entity_mfa` function... but we have these to get code coverage.
        let (mut os, bdfs) = SargonOS::with_bdfs().await;

        let (batman, stats) = os.new_mainnet_persona_with_bdfs("Batman").await.unwrap();
        assert!(stats.debug_was_derived.is_empty());

        let (alice, stats) = os.new_mainnet_account_with_bdfs("alice").await.unwrap();
        assert!(stats.debug_was_derived.is_empty());

        let shield_0 = MatrixOfFactorSources::new([], 0, [bdfs.clone()]);
        let mut cache = os.cache_snapshot();
        let interactors = Arc::new(TestDerivationInteractors::default());
        let outcome = Sut::for_account_mfa(
            &mut cache,
            os.profile_snapshot(),
            shield_0.clone(),
            IndexSet::just(alice.entity_address()),
            interactors.clone(),
        )
        .await
        .unwrap();
        let outcome = outcome.per_factor.get(&bdfs.factor_source_id()).unwrap();
        assert_eq!(outcome.to_use_directly.len(), 1);

        let outcome = Sut::for_persona_mfa(
            &mut cache,
            os.profile_snapshot(),
            shield_0.clone(),
            IndexSet::just(batman.entity_address()),
            interactors.clone(),
        )
        .await
        .unwrap();
        let outcome = outcome.per_factor.get(&bdfs.factor_source_id()).unwrap();
        assert_eq!(outcome.to_use_directly.len(), 1);
    }
}
*/
