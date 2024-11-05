#![cfg(test)]

use crate::prelude::*;

/// Should be merged with SargonOS in Sargon repo...
/// contains three fundamentally new methods:
/// * new_account (using `FactorInstancesProvider`)
/// * new_persona (using `FactorInstancesProvider`)
/// * securify_accounts (using `FactorInstancesProvider`)
/// * add_factor_source (using `FactorInstancesProvider`)
pub(super) struct SargonOS {
    /// FactorInstancesCache of prederived FactorInstances for each factor source in Profile.
    pub(super) cache: FactorInstancesCache,
    profile: RwLock<Profile>,
}

impl SargonOS {
    pub(super) fn profile_snapshot(&self) -> Profile {
        self.profile.try_read().unwrap().clone()
    }

    pub(super) fn new() -> Self {
        Arc::new(TestDerivationInteractors::default());
        Self {
            cache: FactorInstancesCache::default(),
            profile: RwLock::new(Profile::default()),
        }
    }

    pub(super) async fn with_bdfs() -> (Self, FactorSource) {
        let mut self_ = Self::new();
        let bdfs = FactorSource::device();
        self_.add_factor_source(bdfs.clone()).await.unwrap();
        (self_, bdfs)
    }

    pub(super) fn cache_snapshot(&self) -> FactorInstancesCache {
        self.cache.clone()
    }

    pub(super) fn clear_cache(&mut self) {
        println!("💣 CLEAR CACHE");
        self.cache = FactorInstancesCache::default()
    }

    pub(super) async fn new_mainnet_account_with_bdfs(
        &mut self,
        name: impl AsRef<str>,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        self.new_account_with_bdfs(NetworkID::Mainnet, name).await
    }

    pub(super) async fn new_account_with_bdfs(
        &mut self,
        network: NetworkID,
        name: impl AsRef<str>,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        let bdfs = self.profile_snapshot().bdfs();
        self.new_account(bdfs, network, name).await
    }

    pub(super) async fn new_account(
        &mut self,
        factor_source: FactorSource,
        network: NetworkID,
        name: impl AsRef<str>,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        self.new_entity(factor_source, network, name).await
    }

    pub(super) async fn new_mainnet_persona_with_bdfs(
        &mut self,
        name: impl AsRef<str>,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        self.new_persona_with_bdfs(NetworkID::Mainnet, name).await
    }

    pub(super) async fn new_persona_with_bdfs(
        &mut self,
        network: NetworkID,
        name: impl AsRef<str>,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        let bdfs = self.profile_snapshot().bdfs();
        self.new_persona(bdfs, network, name).await
    }

    pub(super) async fn new_persona(
        &mut self,
        factor_source: FactorSource,
        network: NetworkID,
        name: impl AsRef<str>,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        self.new_entity(factor_source, network, name).await
    }

    pub(super) async fn new_entity<E: IsEntity>(
        &mut self,
        factor_source: FactorSource,
        network: NetworkID,
        name: impl AsRef<str>,
    ) -> Result<(E, FactorInstancesProviderOutcomeForFactor)> {
        let profile_snapshot = self.profile_snapshot();
        let outcome = VirtualEntityCreatingInstanceProvider::for_entity_veci(
            E::kind(),
            &mut self.cache,
            Some(profile_snapshot),
            factor_source.clone(),
            network,
            Arc::new(TestDerivationInteractors::default()),
        )
        .await
        .unwrap();

        let outcome_for_factor = outcome;

        let instances_to_use_directly = outcome_for_factor.to_use_directly.clone();

        assert_eq!(instances_to_use_directly.len(), 1);
        let instance = instances_to_use_directly.first().unwrap();

        let address = E::Address::new(network, instance.public_key_hash());
        let security_state = EntitySecurityState::Unsecured(instance);
        let entity = E::new(name, address, security_state, DepositRule::default());
        self.profile
            .try_write()
            .unwrap()
            .insert_entities(IndexSet::just(Into::<AccountOrPersona>::into(
                entity.clone(),
            )))
            .unwrap();

        Ok((entity, outcome_for_factor))
    }

    pub(super) async fn securify_account(
        &mut self,
        account_addresses: AccountAddress,
        shield: MatrixOfFactorSources,
    ) -> Result<(SecurifiedAccount, FactorInstancesProviderOutcome)> {
        let (accounts, stats) = self
            .securify_accounts(IndexSet::just(account_addresses), shield)
            .await?;
        assert_eq!(accounts.len(), 1);
        let account = accounts.into_iter().next().unwrap();
        Ok((account, stats))
    }

    pub(super) async fn securify_accounts(
        &mut self,
        account_addresses: IndexSet<AccountAddress>,
        shield: MatrixOfFactorSources,
    ) -> Result<(SecurifiedAccounts, FactorInstancesProviderOutcome)> {
        self.securify_accounts_with_interactor(
            Arc::new(TestDerivationInteractors::default()),
            account_addresses,
            shield,
        )
        .await
    }

    pub(super) async fn securify_accounts_with_interactor(
        &mut self,
        interactor: Arc<TestDerivationInteractors>,
        account_addresses: IndexSet<AccountAddress>,
        shield: MatrixOfFactorSources,
    ) -> Result<(SecurifiedAccounts, FactorInstancesProviderOutcome)> {
        assert!(!account_addresses.is_empty());
        let network = account_addresses.first().unwrap().network_id();
        let (entities, stats) = self
            .securify_entities_with_interactor::<SecurifiedAccount>(
                interactor,
                account_addresses,
                shield,
            )
            .await?;
        Ok((SecurifiedAccounts::new(network, entities).unwrap(), stats))
    }

    pub(super) async fn securify_personas(
        &mut self,
        identity_addresses: IndexSet<IdentityAddress>,
        shield: MatrixOfFactorSources,
    ) -> Result<(SecurifiedPersonas, FactorInstancesProviderOutcome)> {
        self.securify_personas_with_interactor(
            Arc::new(TestDerivationInteractors::default()),
            identity_addresses,
            shield,
        )
        .await
    }

    pub(super) async fn securify_personas_with_interactor(
        &mut self,
        interactor: Arc<TestDerivationInteractors>,
        identity_addresses: IndexSet<IdentityAddress>,
        shield: MatrixOfFactorSources,
    ) -> Result<(SecurifiedPersonas, FactorInstancesProviderOutcome)> {
        assert!(!identity_addresses.is_empty());
        let network = identity_addresses.first().unwrap().network_id();
        let (entities, stats) = self
            .securify_entities_with_interactor::<SecurifiedPersona>(
                interactor,
                identity_addresses,
                shield,
            )
            .await?;
        Ok((SecurifiedPersonas::new(network, entities).unwrap(), stats))
    }

    pub(super) async fn securify_entities_with_interactor<E: IsSecurifiedEntity>(
        &mut self,
        interactor: Arc<TestDerivationInteractors>,
        addresses_of_entities: IndexSet<<E::BaseEntity as IsEntity>::Address>,
        shield: MatrixOfFactorSources,
    ) -> Result<(IndexSet<E>, FactorInstancesProviderOutcome)> {
        let profile_snapshot = self.profile_snapshot();

        let outcome = SecurifyEntityFactorInstancesProvider::for_entity_mfa::<E::BaseEntity>(
            &mut self.cache,
            profile_snapshot.clone(),
            shield.clone(),
            addresses_of_entities.clone(),
            interactor,
        )
        .await?;

        let mut instance_per_factor = outcome
            .clone()
            .per_factor
            .into_iter()
            .map(|(k, outcome_per_factor)| (k, outcome_per_factor.to_use_directly))
            .collect::<IndexMap<FactorSourceIDFromHash, FactorInstances>>();

        assert_eq!(
            instance_per_factor
                .keys()
                .cloned()
                .collect::<HashSet<FactorSourceIDFromHash>>(),
            shield
                .all_factors()
                .into_iter()
                .map(|f| f.factor_source_id())
                .collect::<HashSet<FactorSourceIDFromHash>>()
        );

        // Now we need to map the flat set of instances into many MatrixOfFactorInstances, and assign
        // one to each account
        let updated_entities = addresses_of_entities
            .clone()
            .into_iter()
            .map(|a| {
                let entity = profile_snapshot.get_entity::<E::BaseEntity>(&a).unwrap();
                let matrix_of_instances =
                    MatrixOfFactorInstances::fulfilling_matrix_of_factor_sources_with_instances(
                        &mut instance_per_factor,
                        shield.clone(),
                    )
                    .unwrap();

                let access_controller = match entity.security_state() {
                    EntitySecurityState::Unsecured(_) => {
                        AccessController::from_unsecurified_address(a)
                    }
                    EntitySecurityState::Securified(sec) => sec.access_controller.clone(),
                };
                let veci = match entity.security_state() {
                    EntitySecurityState::Unsecured(veci) => Some(veci),
                    EntitySecurityState::Securified(sec) => {
                        sec.veci.clone().map(|x| x.factor_instance())
                    }
                };
                let sec = SecuredEntityControl::new(
                    matrix_of_instances,
                    access_controller,
                    veci.map(|x| VirtualEntityCreatingInstance::new(x, entity.address())),
                );

                E::new(entity.name(), entity.entity_address(), sec)
            })
            .collect::<IndexSet<E>>();

        for entity in updated_entities.clone().into_iter() {
            self.profile
                .try_write()
                .unwrap()
                .update_entity::<E::BaseEntity>(entity.into())
        }
        assert!(
            instance_per_factor.values().all(|x| x.is_empty()),
            "should have used all instances, but have unused instances: {:?}",
            instance_per_factor
        );

        Ok((updated_entities, outcome))
    }

    /// Pre-Derives FactorInstances and saves them into the cache
    pub(super) async fn add_factor_source(&mut self, factor_source: FactorSource) -> Result<()> {
        let profile_snapshot = self.profile_snapshot();
        assert!(
            !profile_snapshot
                .factor_sources
                .iter()
                .any(|x| x.factor_source_id() == factor_source.factor_source_id()),
            "factor already in Profile"
        );
        let outcome = CacheFiller::for_new_factor_source(
            &mut self.cache,
            Some(profile_snapshot),
            factor_source.clone(),
            NetworkID::Mainnet,
            Arc::new(TestDerivationInteractors::default()),
        )
        .await
        .unwrap();

        assert_eq!(outcome.factor_source_id, factor_source.factor_source_id());

        assert_eq!(outcome.debug_found_in_cache.len(), 0);

        assert_eq!(
            outcome.debug_was_cached.len(),
            DerivationPreset::all().len() * CACHE_FILLING_QUANTITY
        );

        assert_eq!(
            outcome.debug_was_derived.len(),
            DerivationPreset::all().len() * CACHE_FILLING_QUANTITY
        );

        self.profile
            .try_write()
            .unwrap()
            .add_factor_source(factor_source.clone())
            .unwrap();

        Ok(())
    }
}
