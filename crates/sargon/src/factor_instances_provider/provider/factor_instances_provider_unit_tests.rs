#![allow(non_snake_case)]

use std::ops::{Add, AddAssign};

impl SargonOS {
    pub(crate) async fn with_bdfs() -> (Arc<Self>, FactorSource) {
        let os = Self::fast_boot().await;
        let bdfs = os.bdfs().unwrap();
        (os, bdfs.into())
    }

    pub(crate) async fn create_and_save_new_mainnet_account_with_derivation_outcome(
        &self,
        name: impl AsRef<str>,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_mainnet_account_with_bdfs_with_derivation_outcome(display_name).await
    }

    pub(crate) async fn create_and_save_new_mainnet_persona(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Persona> {
        self.create_and_save_new_mainnet_persona_with_derivation_outcome(name)
            .await
            .map(|(p, _)| p)
    }

    pub(crate) async fn create_and_save_new_mainnet_account(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Account> {
        self.create_and_save_new_mainnet_account_with_derivation_outcome(name)
            .await
            .map(|(a, _)| a)
    }

    pub(crate) async fn create_and_save_new_persona_with_factor_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: impl AsRef<str>,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_persona_with_factor_source_with_derivation_outcome(factor_source, network_id, display_name).await
    }

    pub(crate) async fn create_and_save_new_account_with_factor_with_derivation_outcome(
        &self,
        factor_source: FactorSource,
        network_id: NetworkID,
        name: impl AsRef<str>,
    ) -> Result<(Account, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_account_with_factor_source_with_derivation_outcome(factor_source, network_id, display_name).await
    }

    pub(crate) async fn create_and_save_new_mainnet_persona_with_derivation_outcome(
        &self,
        name: impl AsRef<str>,
    ) -> Result<(Persona, FactorInstancesProviderOutcomeForFactor)> {
        let display_name = DisplayName::new(name)?;
        self.create_and_save_new_mainnet_persona_with_bdfs_with_derivation_outcome(display_name).await
    }

    /// Mutates Accounts in Profile ONLY, DOES NOT submit any transaction changing
    /// security state on chain
    pub(crate) fn __OFFLINE_ONLY_securify_account_without_saving(
        &self,
        account_address: AccountAddress,
        security_structure_of_factor_instances: SecurityStructureOfFactorInstances,
    ) -> Result<Account> {
        let mut account = self.account_by_address(account_address).unwrap();

        let veci: HierarchicalDeterministicFactorInstance;
        let access_controller_address: AccessControllerAddress;

        match account.security_state() {
            EntitySecurityState::Unsecured { value } => {
                veci = value.transaction_signing.clone();
                // THIS IS COMPLETELY WRONG!
                // The real solution should get the AccessControllerAddress on chain
                access_controller_address =
                    AccessControllerAddress::with_node_id_of(
                        &account.address(),
                    );
            }
            EntitySecurityState::Securified { value } => {
                veci = value.veci.clone().unwrap();
                access_controller_address = value.access_controller_address;
            }
        };

        let securified_control = SecuredEntityControl::new(
            veci,
            access_controller_address,
            security_structure_of_factor_instances,
        )?;

        account.security_state = EntitySecurityState::Securified {
            value: securified_control,
        };

        Ok(account)
    }

    /// Uses FactorInstancesProvider to get factor instances for the `shield`.
    /// Mutates Accounts in Profile ONLY, DOES NOT submit any transaction changing
    /// security state on chain
    async fn __OFFLINE_ONLY_securify_accounts(
        &self,
        account_addresses: IndexSet<AccountAddress>,
        shield: &SecurityStructureOfFactorSources,
    ) -> Result<(Accounts, FactorInstancesProviderOutcome)> {
        account_addresses
            .iter()
            .for_each(|a| assert!(self.account_by_address(*a).is_ok()));

        let outcome = self.make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
            account_addresses.clone(),
                    shield.clone()).await?;

        let (
            security_structures_of_factor_instances,
            instances_in_cache_consumer,
            derivation_outcome,
        ) = outcome;

        let mut security_structures_of_factor_instances =
            security_structures_of_factor_instances;

        // consume!
        instances_in_cache_consumer.consume().await?;

        let securified_accounts = account_addresses
            .into_iter()
            .map(|account_address| {
                let security_structure_of_factor_instances =
                    security_structures_of_factor_instances
                        .shift_remove(&account_address)
                        .unwrap();

                // Production ready code should batch update accounts, submit batch transaction to
                // network, and then batch update all accounts in Profile.
                self.__OFFLINE_ONLY_securify_account_without_saving(
                    account_address,
                    security_structure_of_factor_instances,
                )
            })
            .collect::<Result<Accounts>>()?;

        assert!(security_structures_of_factor_instances.is_empty());

        // Assert that none of the NEW FactorInstances collide with the existing ones
        self.profile()
            .unwrap()
            .assert_new_factor_instances_not_already_used(
                securified_accounts.clone(),
            )?;
        self.update_entities(securified_accounts.clone()).await?;

        Ok((
            securified_accounts.into_iter().collect(),
            derivation_outcome,
        ))
    }

    /// Uses FactorInstancesProvider to get factor instances for the `shield`.
    /// Mutates Accounts in Profile ONLY, DOES NOT submit any transaction changing
    /// security state on chain
    #[allow(non_camel_case_types)]
    async fn __OFFLINE_ONLY_securify_account(
        &self,
        account_address: AccountAddress,
        shield: &SecurityStructureOfFactorSources,
    ) -> Result<(Account, FactorInstancesProviderOutcome)> {
        let (accounts, outcome) = self
            .__OFFLINE_ONLY_securify_accounts(
                IndexSet::just(account_address),
                shield,
            )
            .await?;
        assert_eq!(accounts.len(), 1);
        let account = accounts.first().unwrap().clone();
        Ok((account, outcome))
    }
}

use sbor::prelude::indexmap::IndexSet;

use crate::prelude::*;

#[actix_rt::test]
async fn create_accounts_when_last_is_used_cache_is_fill_only_with_account_vecis_and_if_profile_is_used_a_new_account_is_created(
) {
    let (os, bdfs) = SargonOS::with_bdfs().await;
    os.cache_snapshot()
        .await
        .assert_is_full(NetworkID::Mainnet, bdfs.id_from_hash());
    let prefix = "Acco";
    let (_, derivation_outcome) = os
        .batch_create_many_accounts_with_bdfs_with_derivation_outcome_then_save_once(CACHE_FILLING_QUANTITY as u16, NetworkID::Mainnet, prefix.to_owned())
            .await
            .unwrap();
    assert_eq!(derivation_outcome.debug_was_cached.len(), 0);
    assert_eq!(derivation_outcome.debug_was_derived.len(), 0);

    assert_eq!(
        os.profile()
            .unwrap()
            .accounts_on_all_networks_including_hidden()
            .len(),
        CACHE_FILLING_QUANTITY
    );

    let (acco, derivation_outcome) = os
        .create_and_save_new_mainnet_account_with_derivation_outcome(
            "newly derive",
        )
        .await
        .unwrap();

    assert_eq!(
        os.profile()
            .unwrap()
            .accounts_on_all_networks_including_hidden()
            .len(),
        CACHE_FILLING_QUANTITY + 1
    );

    assert_eq!(
        derivation_outcome.debug_was_cached.len(),
        CACHE_FILLING_QUANTITY
    );
    assert_eq!(
        derivation_outcome.debug_was_derived.len(),
        CACHE_FILLING_QUANTITY + 1
    );

    assert_eq!(
        acco.try_get_unsecured_control()
            .unwrap()
            .transaction_signing
            .derivation_path()
            .index(),
        HDPathComponent::from_local_key_space(
            30,
            KeySpace::Unsecurified { is_hardened: true }
        )
        .unwrap()
    );

    os.cache_snapshot()
        .await
        .assert_is_full(NetworkID::Mainnet, bdfs.id_from_hash());

    // and another one
    let (acco, derivation_outcome) = os
        .create_and_save_new_mainnet_account_with_derivation_outcome(
            "newly derive 2",
        )
        .await
        .unwrap();

    assert_eq!(
        os.profile()
            .unwrap()
            .accounts_on_all_networks_including_hidden()
            .len(),
        CACHE_FILLING_QUANTITY + 2
    );

    assert_eq!(derivation_outcome.debug_was_cached.len(), 0);
    assert_eq!(derivation_outcome.debug_was_derived.len(), 0);

    assert_eq!(
        acco.try_get_unsecured_control()
            .unwrap()
            .transaction_signing
            .derivation_path()
            .index(),
        HDPathComponent::from_local_key_space(
            31,
            KeySpace::Unsecurified { is_hardened: true }
        )
        .unwrap()
    );

    let cache = os.cache_snapshot().await;
    assert!(
        !cache.is_full(NetworkID::Mainnet, bdfs.id_from_hash()),
        "just consumed one, so not full"
    );
}

#[actix_rt::test]
async fn add_factor_source() {
    let os = SargonOS::fast_boot().await;
    let cache = os.cache_snapshot().await;
    assert_eq!(
        cache.total_number_of_factor_instances(),
        DerivationPreset::all().len() * CACHE_FILLING_QUANTITY
    );
    assert_eq!(os.profile().unwrap().factor_sources.len(), 1);
    let factor_source = FactorSource::sample();
    os.add_factor_source(factor_source.clone()).await.unwrap();
    assert_eq!(os.profile().unwrap().factor_sources.len(), 2);
    let cache = os.cache_snapshot().await;
    assert!(
        cache.is_full(NetworkID::Mainnet, factor_source.id_from_hash()),
        "Should have put factors into the cache."
    );
}

#[actix_rt::test]
async fn adding_accounts_and_clearing_cache_in_between() {
    let os = SargonOS::fast_boot().await;
    assert!(os
        .profile()
        .unwrap()
        .accounts_on_all_networks_including_hidden()
        .is_empty(),);
    let (alice, derivation_outcome) = os
        .create_and_save_new_mainnet_account_with_derivation_outcome("alice")
        .await
        .unwrap();
    assert!(!derivation_outcome.debug_found_in_cache.is_empty());
    assert!(derivation_outcome.debug_was_cached.is_empty());
    assert!(derivation_outcome.debug_was_derived.is_empty());

    os.clear_cache().await;

    let (bob, derivation_outcome) = os
        .create_and_save_new_mainnet_account_with_derivation_outcome("bob")
        .await
        .unwrap();
    assert!(derivation_outcome.debug_found_in_cache.is_empty());
    assert!(!derivation_outcome.debug_was_cached.is_empty());
    assert!(!derivation_outcome.debug_was_derived.is_empty());
    assert_ne!(alice, bob);

    assert_eq!(
        os.profile()
            .unwrap()
            .accounts_on_all_networks_including_hidden()
            .len(),
        2
    );
}

#[actix_rt::test]
async fn adding_personas_and_clearing_cache_in_between() {
    let os = SargonOS::fast_boot().await;
    assert!(os
        .profile()
        .unwrap()
        .personas_on_all_networks_including_hidden()
        .is_empty());
    let (batman, derivation_outcome) = os
        .create_and_save_new_mainnet_persona_with_derivation_outcome("Batman")
        .await
        .unwrap();

    assert_eq!(
        batman
            .try_get_unsecured_control()
            .unwrap()
            .transaction_signing
            .derivation_path()
            .get_entity_kind(),
        CAP26EntityKind::Identity
    );

    assert!(!derivation_outcome.debug_found_in_cache.is_empty());
    assert!(derivation_outcome.debug_was_cached.is_empty());
    assert!(derivation_outcome.debug_was_derived.is_empty());
    os.clear_cache().await;

    let (satoshi, derivation_outcome) = os
        .create_and_save_new_mainnet_persona_with_derivation_outcome("Satoshi")
        .await
        .unwrap();
    assert!(derivation_outcome.debug_found_in_cache.is_empty());
    assert!(!derivation_outcome.debug_was_cached.is_empty());
    assert!(!derivation_outcome.debug_was_derived.is_empty());
    assert_ne!(batman, satoshi);

    assert_eq!(
        os.profile()
            .unwrap()
            .personas_on_all_networks_including_hidden()
            .len(),
        2
    );
}

#[cfg(test)]
impl PrimaryRoleWithFactorInstances {
    pub fn all_hd_factors(
        &self,
    ) -> Vec<HierarchicalDeterministicFactorInstance> {
        self.all_factors()
            .into_iter()
            .map(|f| {
                HierarchicalDeterministicFactorInstance::try_from(f.clone())
                    .unwrap()
            })
            .collect()
    }
}

#[cfg(test)]
impl RecoveryRoleWithFactorInstances {
    pub fn all_hd_factors(
        &self,
    ) -> Vec<HierarchicalDeterministicFactorInstance> {
        self.all_factors()
            .into_iter()
            .map(|f| {
                HierarchicalDeterministicFactorInstance::try_from(f.clone())
                    .unwrap()
            })
            .collect()
    }
}

#[cfg(test)]
impl ConfirmationRoleWithFactorInstances {
    pub fn all_hd_factors(
        &self,
    ) -> Vec<HierarchicalDeterministicFactorInstance> {
        self.all_factors()
            .into_iter()
            .map(|f| {
                HierarchicalDeterministicFactorInstance::try_from(f.clone())
                    .unwrap()
            })
            .collect()
    }
}

#[actix_rt::test]
async fn cache_is_unchanged_in_case_of_failure() {
    let os = SargonOS::fast_boot().await;
    let bdfs = FactorSource::from(os.bdfs().unwrap());
    let factor_sources = os.profile().unwrap().factor_sources.clone();
    assert_eq!(
        factor_sources.clone().into_iter().collect_vec(),
        vec![bdfs.clone(),]
    );

    let n = CACHE_FILLING_QUANTITY / 2;

    let (_, derivation_outcome) = os.batch_create_many_accounts_with_bdfs_with_derivation_outcome_then_save_once(3 * n as u16, NetworkID::Mainnet, "Acco".to_owned()).await.unwrap();
    assert_eq!(derivation_outcome.debug_was_derived.len(), 3 * n); // `n` missing + CACHE filling 2*n more.

    let all_accounts = os
        .profile()
        .unwrap()
        .accounts_on_all_networks_including_hidden()
        .items();

    assert_eq!(all_accounts.len(), 3 * n);

    let matrix_0 = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            1,
            [bdfs.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_0 =
        SecurityStructureOfFactorSources::new(DisplayName::sample(), matrix_0);

    let all_accounts = os
        .profile()
        .unwrap()
        .accounts_on_all_networks_including_hidden()
        .items();

    let first_half_of_accounts = all_accounts.clone()[0..n]
        .iter()
        .cloned()
        .collect::<IndexSet<Account>>();

    let second_half_of_accounts = all_accounts.clone()[n..3 * n]
        .iter()
        .cloned()
        .collect::<IndexSet<Account>>();

    assert_eq!(
        first_half_of_accounts.len() + second_half_of_accounts.len(),
        3 * n
    );

    let (security_structure_of_factor_instances_first_half, instances_in_cache_consumer, derivation_outcome) = os.make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
        first_half_of_accounts
                .clone()
                .into_iter()
                .map(|a| a.address())
                .collect(),
                shield_0.clone()).await.unwrap();

    // consume!
    instances_in_cache_consumer.consume().await.unwrap();

    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have used cache"
    );

    assert_eq!(
        security_structure_of_factor_instances_first_half
            .into_iter()
            .map(|a| a
                .1
                .matrix_of_factors
                .primary_role
                .all_hd_factors()
                .into_iter()
                .map(|f| f.derivation_entity_index())
                .next()
                .unwrap()) // single factor per role text
            .collect_vec(),
        (0..CACHE_FILLING_QUANTITY / 2)
            .map(|i| HDPathComponent::Securified(
                SecurifiedU30::try_from(i as u32).unwrap()
            ))
            .collect_vec()
    );

    let cache_before_fail = os.cache_snapshot().await;
    let profile = os.profile().unwrap();
    let mnemonic_with_passphrase = os
        .secure_storage
        .load_mnemonic_with_passphrase(bdfs.id_from_hash())
        .await
        .unwrap();

    let fail_interactor: Arc<dyn KeysDerivationInteractors> =
        Arc::new(TestDerivationInteractors::fail()); // <--- FAIL

    let os = SargonOS::fast_boot_bdfs_and_interactor(
        mnemonic_with_passphrase,
        fail_interactor,
        false,
    )
    .await;
    os.set_cache(cache_before_fail.serializable_snapshot())
        .await;
    os.import_profile(profile.clone()).await.unwrap();

    assert_eq!(
        os.profile()
            .unwrap()
            .accounts_on_all_networks_including_hidden(),
        profile.accounts_on_all_networks_including_hidden()
    );
    let c = os.cache_snapshot().await;
    assert_eq!(
        c.serializable_snapshot(),
        cache_before_fail.serializable_snapshot()
    );
    assert_eq!(os.bdfs().unwrap().id_from_hash(), bdfs.id_from_hash());

    let res = os
    .make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
        second_half_of_accounts
        .clone()
        .into_iter()
        .map(|a| a.address())
        .collect(),
        shield_0.clone()
    )
    .await;

    assert!(res.is_err());
    let cache_after_fail = os.cache_snapshot().await;
    assert_eq!(
        cache_after_fail.serializable_snapshot(),
        cache_before_fail.serializable_snapshot(),
        "Cache should not have changed when failing."
    );
}

#[actix_rt::test]
async fn test_assert_factor_instances_invalid() {
    let os = SargonOS::fast_boot().await;
    let alice = os
        .create_and_save_new_mainnet_account_with_bdfs(DisplayName::sample())
        .await
        .unwrap();
    let bob = os
        .create_and_save_new_mainnet_account_with_bdfs(
            DisplayName::sample_other(),
        )
        .await
        .unwrap();

    let bdfs = FactorSource::from(os.bdfs().unwrap());

    // This is NOT a valid Matrix! But for the purpose of this test, it's fine.
    // We are not testing valid matrices here... we are testing the factor
    // instances provider...
    let matrix_0 = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            1,
            [bdfs.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_0 =
        SecurityStructureOfFactorSources::new(DisplayName::sample(), matrix_0);
    let (security_structure_of_fis, _, _) = os.make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(IndexSet::from_iter([alice.address()]), shield_0.clone()).await.unwrap();

    let security_structure_of_fi =
        security_structure_of_fis.values().next().unwrap().clone();

    let securified_alice = os
        .__OFFLINE_ONLY_securify_account_without_saving(
            alice.address(),
            security_structure_of_fi.clone(),
        )
        .unwrap();

    os.update_entities(Accounts::just(securified_alice.clone()))
        .await
        .unwrap();

    // Since we did not call `consume` on the instances consumer, the same factor instances will be used => collision!
    let securified_bob = os
        .__OFFLINE_ONLY_securify_account_without_saving(
            bob.address(),
            security_structure_of_fi.clone(), // Oh noes! Same Structure as Alice!
        )
        .unwrap();

    let res = os
        .profile()
        .unwrap()
        .assert_new_factor_instances_not_already_used(Accounts::just(
            securified_bob.clone(),
        ));
    assert!(res.is_err());

    let err = CommonError::FactorInstancesDiscrepancy {
        address_of_entity1: alice.address().to_string(),
        address_of_entity2: bob.address().to_string(),
        factor_source_id: bdfs.factor_source_id().to_string(),
    };
    pretty_assertions::assert_eq!(res, Err(err));

    let mut fake_frank = securified_alice.clone();
    fake_frank.address = AccountAddress::sample_frank();

    let profile_snapshot_before_failing_op = os.profile().unwrap();
    let res = os.add_account(fake_frank.clone()).await;
    assert!(res.is_err());
    let err = CommonError::FactorInstancesDiscrepancy {
        address_of_entity1: alice.address().to_string(),
        address_of_entity2: fake_frank.address().to_string(),
        factor_source_id: bdfs.factor_source_id().to_string(),
    };
    pretty_assertions::assert_eq!(res, Err(err));
    let profile_snapshot_after_failing_op = os.profile().unwrap();
    assert_eq!(
        profile_snapshot_after_failing_op,
        profile_snapshot_before_failing_op
    );
}

#[actix_rt::test]
async fn add_account_and_personas_mixed() {
    let os = SargonOS::fast_boot().await;
    let profile = os.profile().unwrap();
    assert!(profile
        .personas_on_all_networks_including_hidden()
        .is_empty());
    assert!(profile
        .accounts_on_all_networks_including_hidden()
        .is_empty());

    let (batman, derivation_outcome) = os
        .create_and_save_new_mainnet_persona_with_derivation_outcome("Batman")
        .await
        .unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());

    let (alice, derivation_outcome) = os
        .create_and_save_new_mainnet_account_with_derivation_outcome("alice")
        .await
        .unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());

    let (satoshi, derivation_outcome) = os
        .create_and_save_new_mainnet_persona_with_derivation_outcome("Satoshi")
        .await
        .unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());

    assert_ne!(batman.address(), satoshi.address());

    let (bob, derivation_outcome) = os
        .create_and_save_new_mainnet_account_with_derivation_outcome("bob")
        .await
        .unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());
    assert_ne!(alice.address(), bob.address());

    let profile = os.profile().unwrap();
    assert_eq!(profile.personas_on_all_networks_including_hidden().len(), 2);
    assert_eq!(profile.accounts_on_all_networks_including_hidden().len(), 2);
}

#[actix_rt::test]
async fn adding_accounts_different_networks_different_factor_sources() {
    let os = SargonOS::fast_boot().await;
    let cache = os.cache_snapshot().await;
    assert_eq!(
        cache.total_number_of_factor_instances(),
        DerivationPreset::all().len() * CACHE_FILLING_QUANTITY
    );

    let fs_device = FactorSource::from(os.bdfs().unwrap());
    let fs_arculus = FactorSource::sample_arculus();
    let fs_ledger = FactorSource::sample_ledger();

    os.add_factor_source(fs_device.clone()).await.unwrap();
    os.add_factor_source(fs_arculus.clone()).await.unwrap();
    os.add_factor_source(fs_ledger.clone()).await.unwrap();

    let profile = os.profile().unwrap();
    let cache = os.cache_snapshot().await;
    assert_eq!(
        cache.total_number_of_factor_instances(),
        profile.factor_sources.len()
            * DerivationPreset::all().len()
            * CACHE_FILLING_QUANTITY
    );

    assert!(profile
        .accounts_on_all_networks_including_hidden()
        .is_empty());
    assert_eq!(profile.factor_sources.len(), 3);

    let (alice, derivation_outcome) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            fs_device.clone(),
            NetworkID::Mainnet,
            "Alice",
        )
        .await
        .unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());

    let (bob, derivation_outcome) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            fs_device.clone(),
            NetworkID::Mainnet,
            "Bob",
        )
        .await
        .unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());

    let (carol, derivation_outcome) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            fs_device.clone(),
            NetworkID::Stokenet,
            "Carol",
        )
        .await
        .unwrap();
    assert!(
        !derivation_outcome.debug_was_derived.is_empty(),
        "Should have derived more, since first time Stokenet is used!"
    );

    let (diana, derivation_outcome) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            fs_device.clone(),
            NetworkID::Stokenet,
            "Diana",
        )
        .await
        .unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());

    let (erin, derivation_outcome) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            fs_arculus.clone(),
            NetworkID::Mainnet,
            "Erin",
        )
        .await
        .unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());

    let (frank, derivation_outcome) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            fs_arculus.clone(),
            NetworkID::Mainnet,
            "Frank",
        )
        .await
        .unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());

    let (grace, derivation_outcome) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            fs_arculus.clone(),
            NetworkID::Stokenet,
            "Grace",
        )
        .await
        .unwrap();
    assert!(
        !derivation_outcome.debug_was_derived.is_empty(),
        "Should have derived more, since first time Stokenet is used with the Arculus!"
    );

    let (helena, derivation_outcome) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            fs_arculus.clone(),
            NetworkID::Stokenet,
            "Helena",
        )
        .await
        .unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());

    let (isabel, derivation_outcome) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            fs_ledger.clone(),
            NetworkID::Mainnet,
            "isabel",
        )
        .await
        .unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());

    let (jenny, derivation_outcome) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            fs_ledger.clone(),
            NetworkID::Mainnet,
            "Jenny",
        )
        .await
        .unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());

    let (klara, derivation_outcome) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            fs_ledger.clone(),
            NetworkID::Stokenet,
            "Klara",
        )
        .await
        .unwrap();
    assert!(
        !derivation_outcome.debug_was_derived.is_empty(),
        "Should have derived more, since first time Stokenet is used with the Ledger!"
    );

    let (lisa, derivation_outcome) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            fs_ledger.clone(),
            NetworkID::Stokenet,
            "Lisa",
        )
        .await
        .unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());

    let profile = os.profile().unwrap();
    assert_eq!(
        profile.accounts_on_all_networks_including_hidden().len(),
        12
    );

    let accounts = vec![
        alice, bob, carol, diana, erin, frank, grace, helena, isabel, jenny,
        klara, lisa,
    ];

    let factor_source_count = profile.factor_sources.len();
    let network_count = profile.networks.len();
    let cache = os.cache_snapshot().await;
    assert_eq!(
        cache.total_number_of_factor_instances(),
        network_count
            * factor_source_count
            * DerivationPreset::all().len()
            * CACHE_FILLING_QUANTITY
            - accounts.len()
            + factor_source_count // we do `+ factor_source_count` since every time a factor source is used on a new network for the first time, we derive `CACHE_FILLING_QUANTITY + 1`
    );

    assert_eq!(
        profile
            .accounts_on_all_networks_including_hidden()
            .into_iter()
            .map(|a| a.address())
            .collect::<HashSet<AccountAddress>>(),
        accounts
            .into_iter()
            .map(|a| a.address())
            .collect::<HashSet<AccountAddress>>()
    );
}

#[actix_rt::test]
async fn test_securified_accounts() {
    let os = SargonOS::fast_boot().await;
    let alice = os
        .create_and_save_new_mainnet_account("Alice")
        .await
        .unwrap();

    let bob = os.create_and_save_new_mainnet_account("Bob").await.unwrap();

    assert_ne!(alice.address(), bob.address());
    let bdfs = FactorSource::from(os.bdfs().unwrap());
    let ledger = FactorSource::sample_ledger();
    let arculus = FactorSource::sample_arculus();
    let password = FactorSource::sample_password();
    os.add_factor_source(ledger.clone()).await.unwrap();
    os.add_factor_source(arculus.clone()).await.unwrap();
    os.add_factor_source(password.clone()).await.unwrap();

    let matrix_0 = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            2,
            [bdfs.clone(), ledger.clone(), arculus.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone(), ledger.clone(), arculus.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone(), ledger.clone(), arculus.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_0 =
        SecurityStructureOfFactorSources::new(DisplayName::sample(), matrix_0);

    let (security_structures_of_fis, instances_in_cache_consumer, derivation_outcome) = os
        .make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
            IndexSet::from_iter([alice.address(), bob.address()]),
            shield_0,
        )
        .await
        .unwrap();

    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have used cache"
    );

    // Don't forget to consume!
    instances_in_cache_consumer.consume().await.unwrap();

    let alice_sec = security_structures_of_fis.get(&alice.address()).unwrap();

    let alice_matrix = alice_sec.matrix_of_factors.clone();
    assert_eq!(alice_matrix.primary().get_threshold(), 2);

    assert_eq!(
        alice_matrix
            .primary_role
            .all_factors()
            .into_iter()
            .map(|f| f.factor_source_id)
            .collect_vec(),
        [
            bdfs.factor_source_id(),
            ledger.factor_source_id(),
            arculus.factor_source_id()
        ]
    );

    for factors_for_role in [
        &alice_matrix.primary_role.all_hd_factors(),
        &alice_matrix.recovery_role.all_hd_factors(),
        &alice_matrix.confirmation_role.all_hd_factors(),
    ] {
        assert_eq!(
            factors_for_role
                .iter()
                .map(|f| f.derivation_entity_index())
                .collect_vec(),
            vec![
                HDPathComponent::Securified(SecurifiedU30::ZERO),
                HDPathComponent::Securified(SecurifiedU30::ZERO),
                HDPathComponent::Securified(SecurifiedU30::ZERO),
            ]
        );
    }

    // assert bob

    let bob_sec = security_structures_of_fis.get(&bob.address()).unwrap();

    let bob_matrix = bob_sec.matrix_of_factors.clone();
    assert_eq!(bob_matrix.primary().get_threshold(), 2);

    assert_eq!(
        bob_matrix
            .primary_role
            .all_factors()
            .into_iter()
            .map(|f| f.factor_source_id)
            .collect_vec(),
        [
            bdfs.factor_source_id(),
            ledger.factor_source_id(),
            arculus.factor_source_id()
        ]
    );

    for factors_for_role in [
        &bob_matrix.primary_role.all_hd_factors(),
        &bob_matrix.recovery_role.all_hd_factors(),
        &bob_matrix.confirmation_role.all_hd_factors(),
    ] {
        assert_eq!(
            factors_for_role
                .iter()
                .map(|f| f.derivation_entity_index())
                .collect_vec(),
            vec![
                HDPathComponent::Securified(SecurifiedU30::ONE),
                HDPathComponent::Securified(SecurifiedU30::ONE),
                HDPathComponent::Securified(SecurifiedU30::ONE),
            ]
        );
    }
    let carol = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            ledger.clone(),
            NetworkID::Mainnet,
            "Carol",
        )
        .await
        .unwrap()
        .0;

    assert_eq!(
            carol
                .try_get_unsecured_control()
                .unwrap()
                .transaction_signing
                .derivation_entity_index()
                .index_in_local_key_space(),
            U31::ZERO,
            "First account created with ledger, should have index 0, even though this ledger was used in the shield, since we are using two different KeySpaces for Securified and Unsecurified accounts."
        );

    let matrix_1 = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            1,
            [password.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [password.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [password.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_1 =
        SecurityStructureOfFactorSources::new(DisplayName::sample(), matrix_1);

    let (security_structures_of_fis, instances_in_cache_consumer, _) = os
        .make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
            IndexSet::from_iter([carol.address()]),
            shield_1.clone(),
        )
        .await
        .unwrap();

    // Don't forget to consume!
    instances_in_cache_consumer.consume().await.unwrap();

    let carol_sec = security_structures_of_fis.get(&carol.address()).unwrap();

    let carol_matrix = carol_sec.matrix_of_factors.clone();
    assert_eq!(carol_matrix.primary_role.get_threshold_factors().len(), 1);

    assert_eq!(
        carol_matrix
            .primary_role
            .all_factors()
            .into_iter()
            .map(|f| f.factor_source_id)
            .collect_vec(),
        [password.factor_source_id(),]
    );

    for factors_for_role in [
        &carol_matrix.primary_role.all_hd_factors(),
        &carol_matrix.recovery_role.all_hd_factors(),
        &carol_matrix.confirmation_role.all_hd_factors(),
    ] {
        assert_eq!(
            factors_for_role
                .iter()
                .map(|f| f.derivation_entity_index())
                .collect_vec(),
            vec![HDPathComponent::Securified(SecurifiedU30::ZERO),]
        );
    }

    // Update Alice's shield 1 -  only Passphrase as override factor
    let (security_structures_of_fis, instances_in_cache_consumer, derivation_outcome) = os
    .make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
        IndexSet::from_iter([alice.address()]),
        shield_1,
    )
    .await
    .unwrap();

    // Don't forget to consume!
    instances_in_cache_consumer.consume().await.unwrap();

    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have used cache"
    );

    let alice_sec = security_structures_of_fis.get(&alice.address()).unwrap();

    let alice_matrix = alice_sec.matrix_of_factors.clone();

    assert_eq!(
        alice_matrix
            .primary_role
            .all_factors()
            .into_iter()
            .map(|f| f.factor_source_id)
            .collect_vec(),
        [password.factor_source_id(),]
    );

    for factors_for_role in [
        &alice_matrix.primary_role.all_hd_factors(),
        &alice_matrix.recovery_role.all_hd_factors(),
        &alice_matrix.confirmation_role.all_hd_factors(),
    ] {
        assert_eq!(
            factors_for_role
                .iter()
                .map(|f| f.derivation_entity_index())
                .collect_vec(),
            vec![
                HDPathComponent::Securified(SecurifiedU30::ONE), // Carol used `0`.
            ]
        );
    }
}

#[actix_rt::test]
async fn securify_accounts_when_cache_is_half_full_single_factor_source() {
    let os = SargonOS::fast_boot().await;

    let profile = os.profile().unwrap();
    let bdfs = FactorSource::from(os.bdfs().unwrap());
    let factor_sources = profile.factor_sources.clone();
    assert_eq!(
        factor_sources.clone().into_iter().collect_vec(),
        vec![bdfs.clone(),]
    );

    let n = CACHE_FILLING_QUANTITY / 2;

    os.batch_create_many_accounts_with_bdfs_then_save_once(
        3 * n as u16,
        NetworkID::Mainnet,
        "Acco".to_owned(),
    )
    .await
    .unwrap();

    // This is NOT a valid Matrix! But for the purpose of this test, it's fine.
    // We are not testing valid matrices here... we are testing the factor
    // instances provider...
    let matrix_0 = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            1,
            [bdfs.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_0 =
        SecurityStructureOfFactorSources::new(DisplayName::sample(), matrix_0);
    let profile = os.profile().unwrap();
    let all_accounts = profile
        .accounts_on_all_networks_including_hidden()
        .into_iter()
        .collect_vec();

    let first_half_of_accounts = all_accounts.clone()[0..n]
        .iter()
        .cloned()
        .collect::<IndexSet<Account>>();

    let second_half_of_accounts = all_accounts.clone()[n..3 * n]
        .iter()
        .cloned()
        .collect::<IndexSet<Account>>();

    assert_eq!(
        first_half_of_accounts.len() + second_half_of_accounts.len(),
        3 * n
    );

    let (security_structures_of_fis, instances_in_cache_consumer, derivation_outcome) = os
    .make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
        first_half_of_accounts.clone().into_iter().map(|a| a.address()).collect(),
        shield_0.clone(),
    )
    .await
    .unwrap();

    // dont forget to consume!
    instances_in_cache_consumer.consume().await.unwrap();

    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have used cache"
    );

    assert_eq!(
        security_structures_of_fis
            .values()
            .map(|ss| ss
                .matrix_of_factors
                .primary_role
                .all_hd_factors()
                .into_iter()
                .map(|f| f.derivation_entity_index())
                .next()
                .unwrap()) // single factor per role text
            .collect_vec(),
        (0..CACHE_FILLING_QUANTITY / 2)
            .map(|i| HDPathComponent::from_global_key_space(
                i as u32 + GLOBAL_OFFSET_HARDENED_SECURIFIED
            )
            .unwrap())
            .collect_vec()
    );

    let (security_structures_of_fis, instances_in_cache_consumer, derivation_outcome) = os
    .make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
        second_half_of_accounts.clone().into_iter().map(|a| a.address()).collect(),
        shield_0.clone(),
    )
    .await
    .unwrap();

    // dont forget to consume!
    instances_in_cache_consumer.consume().await.unwrap();

    assert!(
        derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have derived more"
    );

    assert_eq!(
        security_structures_of_fis
            .values()
            .map(|ss| ss
                .matrix_of_factors
                .primary_role
                .all_hd_factors()
                .into_iter()
                .map(|f| f.derivation_entity_index())
                .next()
                .unwrap()) // single factor per role text
            .collect_vec(),
        (CACHE_FILLING_QUANTITY / 2
            ..(CACHE_FILLING_QUANTITY / 2 + CACHE_FILLING_QUANTITY))
            .map(|i| HDPathComponent::from_global_key_space(
                i as u32 + GLOBAL_OFFSET_HARDENED_SECURIFIED
            )
            .unwrap())
            .collect_vec()
    );
}

#[actix_rt::test]
async fn securify_accounts_when_cache_is_half_full_multiple_factor_sources() {
    let (os, bdfs) = SargonOS::with_bdfs().await;
    let ledger = FactorSource::sample_ledger();
    let arculus = FactorSource::sample_arculus();
    let password = FactorSource::sample_password();
    os.add_factor_source(ledger.clone()).await.unwrap();
    os.add_factor_source(arculus.clone()).await.unwrap();
    os.add_factor_source(password.clone()).await.unwrap();

    let profile = os.profile().unwrap();
    let factor_sources = profile.factor_sources.clone();
    assert_eq!(
        factor_sources.clone().into_iter().collect_vec(),
        vec![
            bdfs.clone(),
            ledger.clone(),
            arculus.clone(),
            password.clone(),
        ]
    );

    let n = CACHE_FILLING_QUANTITY / 2;

    let (_, derivation_outcome) = os.batch_create_many_accounts_with_bdfs_with_derivation_outcome_then_save_once(3 * n as u16, NetworkID::Mainnet, "Acco".to_owned()).await.unwrap();

    assert_eq!(derivation_outcome.debug_was_derived.len(), 3 * n); // `n` missing + CACHE filling 2*n more.

    // This is NOT a valid Matrix! But for the purpose of this test, it's fine.
    // We are not testing valid matrices here... we are testing the factor
    // instances provider...
    let matrix_0 = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            2,
            [bdfs.clone(), ledger.clone(), arculus.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone(), ledger.clone(), arculus.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone(), ledger.clone(), arculus.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_0 =
        SecurityStructureOfFactorSources::new(DisplayName::sample(), matrix_0);
    let all_accounts = os
        .profile()
        .unwrap()
        .accounts_on_all_networks_including_hidden()
        .into_iter()
        .collect_vec();

    let first_half_of_accounts = all_accounts.clone()[0..n]
        .iter()
        .cloned()
        .collect::<IndexSet<Account>>();

    let second_half_of_accounts = all_accounts.clone()[n..3 * n]
        .iter()
        .cloned()
        .collect::<IndexSet<Account>>();

    assert_eq!(
        first_half_of_accounts.len() + second_half_of_accounts.len(),
        3 * n
    );

    let (security_structures_of_fis, instances_in_cache_consumer, derivation_outcome) = os
    .make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
        first_half_of_accounts.clone().into_iter().map(|a| a.address()).collect(),
        shield_0.clone(),
    )
    .await
    .unwrap();

    // dont forget to consume!
    instances_in_cache_consumer.consume().await.unwrap();

    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have used cache"
    );
    let expected = vec![
        ["0^", "0^", "0^"],
        ["1^", "1^", "1^"],
        ["2^", "2^", "2^"],
        ["3^", "3^", "3^"],
        ["4^", "4^", "4^"],
        ["5^", "5^", "5^"],
        ["6^", "6^", "6^"],
        ["7^", "7^", "7^"],
        ["8^", "8^", "8^"],
        ["9^", "9^", "9^"],
        ["10^", "10^", "10^"],
        ["11^", "11^", "11^"],
        ["12^", "12^", "12^"],
        ["13^", "13^", "13^"],
        ["14^", "14^", "14^"],
    ];

    assert_eq!(
        security_structures_of_fis
            .values()
            .map(|ss| ss
                .matrix_of_factors
                .primary_role
                .all_hd_factors()
                .iter()
                .map(|f| f.derivation_entity_index())
                .map(|x| format!("{:?}", x))
                .collect_vec())
            .collect_vec(),
        expected.clone()
    );

    assert_eq!(
        security_structures_of_fis
            .values()
            .map(|ss| ss
                .matrix_of_factors
                .confirmation_role
                .all_hd_factors()
                .iter()
                .map(|f| f.derivation_entity_index())
                .map(|x| format!("{:?}", x))
                .collect_vec())
            .collect_vec(),
        expected.clone()
    );

    assert_eq!(
        security_structures_of_fis
            .values()
            .map(|ss| ss
                .matrix_of_factors
                .recovery_role
                .all_hd_factors()
                .iter()
                .map(|f| f.derivation_entity_index())
                .map(|x| format!("{:?}", x))
                .collect_vec())
            .collect_vec(),
        expected.clone()
    );

    let (security_structures_of_fis, instances_in_cache_consumer, derivation_outcome) = os
    .make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
        second_half_of_accounts.clone().into_iter().map(|a| a.address()).collect(),
        shield_0.clone(),
    )
    .await
    .unwrap();

    // dont forget to consume!
    instances_in_cache_consumer.consume().await.unwrap();

    assert!(
        derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have derived more"
    );

    assert!(
        derivation_outcome.found_any_instances_in_cache_for_any_factor_source(),
        "should have found some in cache"
    );

    let expected = vec![
        ["15^", "15^", "15^"],
        ["16^", "16^", "16^"],
        ["17^", "17^", "17^"],
        ["18^", "18^", "18^"],
        ["19^", "19^", "19^"],
        ["20^", "20^", "20^"],
        ["21^", "21^", "21^"],
        ["22^", "22^", "22^"],
        ["23^", "23^", "23^"],
        ["24^", "24^", "24^"],
        ["25^", "25^", "25^"],
        ["26^", "26^", "26^"],
        ["27^", "27^", "27^"],
        ["28^", "28^", "28^"],
        ["29^", "29^", "29^"],
        ["30^", "30^", "30^"],
        ["31^", "31^", "31^"],
        ["32^", "32^", "32^"],
        ["33^", "33^", "33^"],
        ["34^", "34^", "34^"],
        ["35^", "35^", "35^"],
        ["36^", "36^", "36^"],
        ["37^", "37^", "37^"],
        ["38^", "38^", "38^"],
        ["39^", "39^", "39^"],
        ["40^", "40^", "40^"],
        ["41^", "41^", "41^"],
        ["42^", "42^", "42^"],
        ["43^", "43^", "43^"],
        ["44^", "44^", "44^"],
    ];

    assert_eq!(
        security_structures_of_fis
            .values()
            .map(|ss| ss
                .matrix_of_factors
                .primary_role
                .all_hd_factors()
                .iter()
                .map(|f| f.derivation_entity_index())
                .map(|x| format!("{:?}", x))
                .collect_vec())
            .collect_vec(),
        expected
    );

    assert_eq!(
        security_structures_of_fis
            .values()
            .map(|ss| ss
                .matrix_of_factors
                .recovery_role
                .all_hd_factors()
                .iter()
                .map(|f| f.derivation_entity_index())
                .map(|x| format!("{:?}", x))
                .collect_vec())
            .collect_vec(),
        expected
    );

    assert_eq!(
        security_structures_of_fis
            .values()
            .map(|ss| ss
                .matrix_of_factors
                .confirmation_role
                .all_hd_factors()
                .iter()
                .map(|f| f.derivation_entity_index())
                .map(|x| format!("{:?}", x))
                .collect_vec())
            .collect_vec(),
        expected
    );
}

#[actix_rt::test]
async fn securify_personas_when_cache_is_half_full_single_factor_source() {
    let (os, bdfs) = SargonOS::with_bdfs().await;
    let factor_sources = os.profile().unwrap().factor_sources.clone();
    assert_eq!(
        factor_sources.clone().into_iter().collect_vec(),
        vec![bdfs.clone(),]
    );

    let n = CACHE_FILLING_QUANTITY / 2;

    let (_, _) = os.batch_create_many_personas_with_bdfs_with_derivation_outcome_then_save_once(3 * n as u16, NetworkID::Mainnet, "Persona".to_owned()).await.unwrap();

    // This is NOT a valid Matrix! But for the purpose of this test, it's fine.
    // We are not testing valid matrices here... we are testing the factor
    // instances provider...
    let matrix_0 = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            1,
            [bdfs.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_0 =
        SecurityStructureOfFactorSources::new(DisplayName::sample(), matrix_0);
    let all_personas = os
        .profile()
        .unwrap()
        .personas_on_all_networks_including_hidden()
        .into_iter()
        .collect_vec();

    let first_half_of_personas = all_personas.clone()[0..n]
        .iter()
        .cloned()
        .collect::<IndexSet<Persona>>();

    let second_half_of_personas = all_personas.clone()[n..3 * n]
        .iter()
        .cloned()
        .collect::<IndexSet<Persona>>();

    assert_eq!(
        first_half_of_personas.len() + second_half_of_personas.len(),
        3 * n
    );

    let (security_structures_of_fis, instances_in_cache_consumer, derivation_outcome) = os
    .make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
        first_half_of_personas.clone().into_iter().map(|a| a.address()).collect(),
        shield_0.clone(),
    )
    .await
    .unwrap();

    // dont forget to consume!
    instances_in_cache_consumer.consume().await.unwrap();

    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have used cache"
    );

    assert_eq!(
        security_structures_of_fis
            .values()
            .map(|ss| ss
                .matrix_of_factors
                .primary_role
                .all_hd_factors()
                .into_iter()
                .map(|f| f.derivation_entity_index())
                .map(|x| format!("{:?}", x))
                .next()
                .unwrap()) // single factor per role text
            .collect_vec(),
        [
            "0^", "1^", "2^", "3^", "4^", "5^", "6^", "7^", "8^", "9^", "10^",
            "11^", "12^", "13^", "14^"
        ]
    );

    let (security_structures_of_fis, instances_in_cache_consumer, derivation_outcome) = os
    .make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
        second_half_of_personas.clone().into_iter().map(|a| a.address()).collect(),
        shield_0.clone(),
    )
    .await
    .unwrap();

    // dont forget to consume!
    instances_in_cache_consumer.consume().await.unwrap();

    assert!(
        derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have derived more"
    );

    assert_eq!(
        security_structures_of_fis
            .values()
            .map(|ss| ss
                .matrix_of_factors
                .primary_role
                .all_hd_factors()
                .into_iter()
                .map(|f| f.derivation_entity_index())
                .map(|x| format!("{:?}", x))
                .next()
                .unwrap()) // single factor per role text
            .collect_vec(),
        [
            "15^", "16^", "17^", "18^", "19^", "20^", "21^", "22^", "23^",
            "24^", "25^", "26^", "27^", "28^", "29^", "30^", "31^", "32^",
            "33^", "34^", "35^", "36^", "37^", "38^", "39^", "40^", "41^",
            "42^", "43^", "44^"
        ]
    );
}

#[actix_rt::test]
async fn create_single_account() {
    let (os, bdfs) = SargonOS::with_bdfs().await;

    let (alice, derivation_outcome) = os
        .create_and_save_new_mainnet_account_with_derivation_outcome("alice")
        .await
        .unwrap();

    assert!(
        derivation_outcome.debug_was_derived.is_empty(),
        "should have used cache"
    );

    // This is NOT a valid Matrix! But for the purpose of this test, it's fine.
    // We are not testing valid matrices here... we are testing the factor
    // instances provider...
    let matrix_0 = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            1,
            [bdfs.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_0 =
        SecurityStructureOfFactorSources::new(DisplayName::sample(), matrix_0);

    let (security_structures_of_fis, instances_in_cache_consumer, derivation_outcome) = os
    .make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
       IndexSet::just(alice.address()),
        shield_0.clone(),
    )
    .await
    .unwrap();

    // Don't forget to consume!
    instances_in_cache_consumer.consume().await.unwrap();

    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have used cache"
    );

    let alice_sec = security_structures_of_fis.get(&alice.address()).unwrap();

    let alice_matrix = alice_sec.matrix_of_factors.clone();

    assert_eq!(
        alice_matrix
            .primary_role
            .all_factors()
            .into_iter()
            .map(|f| f.factor_source_id)
            .collect_vec(),
        [bdfs.factor_source_id(),]
    );

    for factors_for_role in [
        &alice_matrix.primary_role.all_hd_factors(),
        &alice_matrix.recovery_role.all_hd_factors(),
        &alice_matrix.confirmation_role.all_hd_factors(),
    ] {
        assert_eq!(
            factors_for_role
                .iter()
                .map(|f| f.derivation_entity_index())
                .collect_vec(),
            vec![HDPathComponent::Securified(SecurifiedU30::ZERO),]
        );
    }
}

#[actix_rt::test]
async fn securified_personas() {
    let os = SargonOS::fast_boot().await;
    let batman = os
        .create_and_save_new_mainnet_persona("Batman")
        .await
        .unwrap();

    let satoshi = os
        .create_and_save_new_mainnet_persona("Satoshi")
        .await
        .unwrap();

    assert_ne!(batman.address(), satoshi.address());
    let bdfs = FactorSource::from(os.bdfs().unwrap());
    let ledger = FactorSource::sample_ledger();
    let arculus = FactorSource::sample_arculus();
    let password = FactorSource::sample_password();
    os.add_factor_source(ledger.clone()).await.unwrap();
    os.add_factor_source(arculus.clone()).await.unwrap();
    os.add_factor_source(password.clone()).await.unwrap();

    let matrix_0 = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            2,
            [bdfs.clone(), ledger.clone(), arculus.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone(), ledger.clone(), arculus.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone(), ledger.clone(), arculus.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_0 =
        SecurityStructureOfFactorSources::new(DisplayName::sample(), matrix_0);

    let (security_structures_of_fis, instances_in_cache_consumer, derivation_outcome) = os
        .make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
            IndexSet::from_iter([batman.address(), satoshi.address()]),
            shield_0,
        )
        .await
        .unwrap();

    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have used cache"
    );

    // Don't forget to consume!
    instances_in_cache_consumer.consume().await.unwrap();

    let batman_sec = security_structures_of_fis.get(&batman.address()).unwrap();

    let batman_matrix = batman_sec.matrix_of_factors.clone();
    assert_eq!(batman_matrix.primary().get_threshold(), 2);

    assert_eq!(
        batman_matrix
            .primary_role
            .all_factors()
            .into_iter()
            .map(|f| f.factor_source_id)
            .collect_vec(),
        [
            bdfs.factor_source_id(),
            ledger.factor_source_id(),
            arculus.factor_source_id()
        ]
    );

    for factors_for_role in [
        &batman_matrix.primary_role.all_hd_factors(),
        &batman_matrix.recovery_role.all_hd_factors(),
        &batman_matrix.confirmation_role.all_hd_factors(),
    ] {
        assert_eq!(
            factors_for_role
                .iter()
                .map(|f| f.derivation_entity_index())
                .collect_vec(),
            vec![
                HDPathComponent::Securified(SecurifiedU30::ZERO),
                HDPathComponent::Securified(SecurifiedU30::ZERO),
                HDPathComponent::Securified(SecurifiedU30::ZERO),
            ]
        );
    }

    // assert satoshi

    let satoshi_sec =
        security_structures_of_fis.get(&satoshi.address()).unwrap();

    let satoshi_matrix = satoshi_sec.matrix_of_factors.clone();
    assert_eq!(satoshi_matrix.primary().get_threshold(), 2);

    assert_eq!(
        satoshi_matrix
            .primary_role
            .all_factors()
            .into_iter()
            .map(|f| f.factor_source_id)
            .collect_vec(),
        [
            bdfs.factor_source_id(),
            ledger.factor_source_id(),
            arculus.factor_source_id()
        ]
    );

    for factors_for_role in [
        &satoshi_matrix.primary_role.all_hd_factors(),
        &satoshi_matrix.recovery_role.all_hd_factors(),
        &satoshi_matrix.confirmation_role.all_hd_factors(),
    ] {
        assert_eq!(
            factors_for_role
                .iter()
                .map(|f| f.derivation_entity_index())
                .collect_vec(),
            vec![
                HDPathComponent::Securified(SecurifiedU30::ONE),
                HDPathComponent::Securified(SecurifiedU30::ONE),
                HDPathComponent::Securified(SecurifiedU30::ONE),
            ]
        );
    }
    let hyde = os
        .create_and_save_new_persona_with_factor_with_derivation_outcome(
            ledger.clone(),
            NetworkID::Mainnet,
            "Mr Hyde",
        )
        .await
        .unwrap()
        .0;

    assert_eq!(
            hyde
                .try_get_unsecured_control()
                .unwrap()
                .transaction_signing
                .derivation_entity_index()
                .index_in_local_key_space(),
            U31::ZERO,
            "First persona created with ledger, should have index 0, even though this ledger was used in the shield, since we are using two different KeySpaces for Securified and Unsecurified personas."
        );

    // This is NOT a valid Matrix! But for the purpose of this test, it's fine.
    // We are not testing valid matrices here... we are testing the factor
    // instances provider...
    let matrix_1 = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            1,
            [password.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [password.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [password.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_1 =
        SecurityStructureOfFactorSources::new(DisplayName::sample(), matrix_1);

    let (security_structures_of_fis, instances_in_cache_consumer, derivation_outcome) = os
        .make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
            IndexSet::from_iter([hyde.address()]),
            shield_1.clone(),
        )
        .await
        .unwrap();

    // Don't forget to consume!
    instances_in_cache_consumer.consume().await.unwrap();

    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have used cache"
    );

    let hyde_sec = security_structures_of_fis.get(&hyde.address()).unwrap();

    let hyde_matrix = hyde_sec.matrix_of_factors.clone();
    assert_eq!(hyde_matrix.primary_role.get_threshold_factors().len(), 1);

    assert_eq!(
        hyde_matrix
            .primary_role
            .all_factors()
            .into_iter()
            .map(|f| f.factor_source_id)
            .collect_vec(),
        [password.factor_source_id(),]
    );

    for factors_for_role in [
        &hyde_matrix.primary_role.all_hd_factors(),
        &hyde_matrix.recovery_role.all_hd_factors(),
        &hyde_matrix.confirmation_role.all_hd_factors(),
    ] {
        assert_eq!(
            factors_for_role
                .iter()
                .map(|f| f.derivation_entity_index())
                .collect_vec(),
            vec![HDPathComponent::Securified(SecurifiedU30::ZERO),]
        );
    }

    // Update Batman's shield 1 -  only Passphrase as override factor
    let (security_structures_of_fis, instances_in_cache_consumer, derivation_outcome) = os
    .make_security_structure_of_factor_instances_for_entities_without_consuming_cache_with_derivation_outcome(
        IndexSet::from_iter([batman.address()]),
        shield_1,
    )
    .await
    .unwrap();

    instances_in_cache_consumer.consume().await.unwrap();

    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have used cache"
    );

    let batman_sec = security_structures_of_fis.get(&batman.address()).unwrap();

    let batman_matrix = batman_sec.matrix_of_factors.clone();

    assert_eq!(
        batman_matrix
            .primary_role
            .all_factors()
            .into_iter()
            .map(|f| f.factor_source_id)
            .collect_vec(),
        [password.factor_source_id(),]
    );

    for factors_for_role in [
        &batman_matrix.primary_role.all_hd_factors(),
        &batman_matrix.recovery_role.all_hd_factors(),
        &batman_matrix.confirmation_role.all_hd_factors(),
    ] {
        assert_eq!(
            factors_for_role
                .iter()
                .map(|f| f.derivation_entity_index())
                .collect_vec(),
            vec![
                HDPathComponent::Securified(SecurifiedU30::ONE), // Hyde used `0`.
            ]
        );
    }
}

#[actix_rt::test]
async fn securified_all_accounts_next_veci_does_not_start_at_zero() {
    let os = SargonOS::fast_boot().await;
    let cache = os.cache_snapshot().await;
    assert_eq!(
        cache.total_number_of_factor_instances(),
        DerivationPreset::all().len() * CACHE_FILLING_QUANTITY
    );

    let bdfs = FactorSource::from(os.bdfs().unwrap());
    let arculus = FactorSource::sample_arculus();
    let ledger = FactorSource::sample_ledger();

    os.add_factor_source(arculus.clone()).await.unwrap();
    os.add_factor_source(ledger.clone()).await.unwrap();

    let factor_source_count = os.profile().unwrap().factor_sources.len();

    let cache = os.cache_snapshot().await;
    assert_eq!(
        cache.total_number_of_factor_instances(),
        factor_source_count
            * DerivationPreset::all().len()
            * CACHE_FILLING_QUANTITY
    );

    let profile = os.profile().unwrap();
    assert!(profile
        .accounts_on_all_networks_including_hidden()
        .is_empty());
    assert_eq!(profile.factor_sources.len(), 3);

    let network = NetworkID::Mainnet;

    // first create CACHE_FILLING_QUANTITY many "unnamed" accounts

    let (_, derivation_outcome) = os.batch_create_many_accounts_with_bdfs_with_derivation_outcome_then_save_once(CACHE_FILLING_QUANTITY as u16, network, "Acco".to_owned()).await.unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());

    let unnamed_accounts = os
        .profile()
        .unwrap()
        .accounts_on_all_networks_including_hidden()
        .into_iter()
        .collect_vec();

    // This is NOT a valid Matrix! But for the purpose of this test, it's fine.
    // We are not testing valid matrices here... we are testing the factor
    // instances provider...
    let matrix_0 = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            1,
            [bdfs.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_0 =
        SecurityStructureOfFactorSources::new(DisplayName::sample(), matrix_0);
    let (_, derivation_outcome) = os
        .__OFFLINE_ONLY_securify_accounts(
            unnamed_accounts
                .clone()
                .into_iter()
                .map(|a| a.address())
                .collect(),
            &shield_0,
        )
        .await
        .unwrap();

    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have used cache"
    );

    // assert correctness of next index assigner
    assert_eq!(
        os.accounts_on_current_network().unwrap().len(),
        CACHE_FILLING_QUANTITY
    );

    let next_index_profile_assigner =
        NextDerivationEntityIndexProfileAnalyzingAssigner::new(
            network,
            Arc::new(os.profile().unwrap()),
        );

    let next_index_veci = next_index_profile_assigner
        .next(
            bdfs.id_from_hash(),
            DerivationPreset::AccountVeci
                .index_agnostic_path_on_network(network),
        )
        .unwrap()
        .unwrap();

    assert_eq!(
        next_index_veci,
        HDPathComponent::Unsecurified(Unsecurified::Hardened(
            UnsecurifiedHardened::try_from(30u32).unwrap()
        ))
    );

    let next_index_mfa = next_index_profile_assigner
        .next(
            bdfs.id_from_hash(),
            DerivationPreset::AccountMfa
                .index_agnostic_path_on_network(network),
        )
        .unwrap()
        .unwrap();

    assert_eq!(
        next_index_mfa,
        HDPathComponent::Securified(SecurifiedU30::try_from(30u32).unwrap())
    );

    let (alice, derivation_outcome) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            bdfs.clone(),
            network,
            "Alice",
        )
        .await
        .unwrap();

    assert!(
        derivation_outcome.debug_found_in_cache.is_empty(),
        "Cache should have been empty"
    );

    assert!(
        !derivation_outcome.debug_was_cached.is_empty(),
        "should have filled cache"
    );

    assert!(
        !derivation_outcome.debug_was_derived.is_empty(),
        "should derived more since cache was empty"
    );

    assert_eq!(
        alice
            .try_get_unsecured_control()
            .unwrap()
            .transaction_signing
            .derivation_entity_index(),
        HDPathComponent::Unsecurified(Unsecurified::Hardened(
            UnsecurifiedHardened::try_from(30u32).unwrap()
        )) // <-- IMPORTANT this tests that we do not start at 0', asserts that the next index from profile analyzer
    );

    let (_, derivation_outcome) = os
        .__OFFLINE_ONLY_securify_accounts(
            IndexSet::just(alice.address()),
            &shield_0,
        )
        .await
        .unwrap();

    assert!(
        derivation_outcome.found_any_instances_in_cache_for_any_factor_source()
    );
    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source()
    );

    let alice = os
        .profile()
        .unwrap()
        .account_by_address(alice.address())
        .unwrap();
    let alice_sec = alice.try_get_secured_control().unwrap();
    let alice_matrix = alice_sec.security_structure.matrix_of_factors.clone();
    for factors_for_role in [
        &alice_matrix.primary_role.all_hd_factors(),
        &alice_matrix.recovery_role.all_hd_factors(),
        &alice_matrix.confirmation_role.all_hd_factors(),
    ] {
        assert_eq!(
            factors_for_role
                .iter()
                .map(|f| f.derivation_entity_index())
                .collect_vec(),
            vec![HDPathComponent::Securified(
                SecurifiedU30::try_from(30u32).unwrap()
            ),]
        );
    }
}

#[actix_rt::test]
async fn securified_accounts_asymmetric_indices() {
    let (os, bdfs) = SargonOS::with_bdfs().await;
    let cache = os.cache_snapshot().await;
    assert_eq!(
        cache.total_number_of_factor_instances(),
        DerivationPreset::all().len() * CACHE_FILLING_QUANTITY
    );

    let arculus = FactorSource::sample_arculus();
    let ledger = FactorSource::sample_ledger();

    os.add_factor_source(arculus.clone()).await.unwrap();
    os.add_factor_source(ledger.clone()).await.unwrap();

    let number_of_factor_sources = os.profile().unwrap().factor_sources.len();
    assert_eq!(number_of_factor_sources, 3);
    let cache = os.cache_snapshot().await;
    assert_eq!(
        cache.total_number_of_factor_instances(),
        number_of_factor_sources
            * DerivationPreset::all().len()
            * CACHE_FILLING_QUANTITY
    );

    let profile = os.profile().unwrap();
    assert!(profile
        .accounts_on_all_networks_including_hidden()
        .is_empty());

    let network = NetworkID::Mainnet;

    // first create CACHE_FILLING_QUANTITY many "unnamed" accounts

    let (_, derivation_outcome) = os.batch_create_many_accounts_with_factor_source_with_derivation_outcome_then_save_once(bdfs.clone(), CACHE_FILLING_QUANTITY as u16, network, "Acco".to_owned()).await.unwrap();
    assert!(derivation_outcome.debug_was_derived.is_empty());

    let unnamed_accounts = os
        .profile()
        .unwrap()
        .accounts_on_all_networks_including_hidden()
        .into_iter()
        .collect_vec();

    // This is NOT a valid Matrix! But for the purpose of this test, it's fine.
    // We are not testing valid matrices here... we are testing the factor
    // instances provider...
    let matrix_0 = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            1,
            [bdfs.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_0 =
        SecurityStructureOfFactorSources::new(DisplayName::sample(), matrix_0);
    let (_, derivation_outcome) = os
        .__OFFLINE_ONLY_securify_accounts(
            unnamed_accounts
                .clone()
                .iter()
                .map(|a| a.address())
                .collect(),
            &shield_0,
        )
        .await
        .unwrap();

    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "should have used cache"
    );

    let (alice, derivation_outcome) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            bdfs.clone(),
            network,
            "Alice",
        )
        .await
        .unwrap();
    assert!(
        derivation_outcome.debug_found_in_cache.is_empty(),
        "Cache should have been empty"
    );
    assert!(
        !derivation_outcome.debug_was_derived.is_empty(),
        "should have filled cache"
    );
    assert!(
        !derivation_outcome.debug_was_cached.is_empty(),
        "should have filled cache"
    );

    let (bob, _) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            bdfs.clone(),
            network,
            "Bob",
        )
        .await
        .unwrap();

    let (carol, _) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            bdfs.clone(),
            network,
            "Carol",
        )
        .await
        .unwrap();

    let (diana, _) = os
        .create_and_save_new_account_with_factor_with_derivation_outcome(
            bdfs.clone(),
            network,
            "Diana",
        )
        .await
        .unwrap();

    assert_eq!(
        diana
            .try_get_unsecured_control()
            .unwrap()
            .transaction_signing
            .derivation_entity_index(),
        HDPathComponent::from_local_key_space(
            33,
            KeySpace::Unsecurified { is_hardened: true }
        )
        .unwrap()
    );

    assert_eq!(
        HashSet::<_>::from_iter([
            alice.address(),
            bob.address(),
            carol.address(),
            diana.address()
        ])
        .len(),
        4
    );

    // This is NOT a valid Matrix! But for the purpose of this test, it's fine.
    // We are not testing valid matrices here... we are testing the factor
    // instances provider...
    let matrix_1 = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            2,
            [bdfs.clone(), arculus.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone(), arculus.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone(), arculus.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_1 =
        SecurityStructureOfFactorSources::new(DisplayName::sample(), matrix_1);

    let (securified_alice, derivation_outcome) = os
        .__OFFLINE_ONLY_securify_account(alice.address(), &shield_1)
        .await
        .unwrap();
    assert_eq!(securified_alice.address(), alice.address());

    assert!(
        derivation_outcome.found_any_instances_in_cache_for_any_factor_source()
    );
    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source()
    );

    assert_eq!(
        securified_alice
            .try_get_secured_control()
            .unwrap()
            .security_structure
            .matrix_of_factors
            .primary_role
            .all_hd_factors()
            .into_iter()
            .map(|f| (f.factor_source_id, f.derivation_entity_index()))
            .collect::<IndexMap<FactorSourceIDFromHash, HDPathComponent>>(),
        [
            (
                bdfs.id_from_hash(),
                HDPathComponent::from_local_key_space(30, KeySpace::Securified)
                    .unwrap()
            ),
            (
                arculus.id_from_hash(),
                HDPathComponent::from_local_key_space(0, KeySpace::Securified)
                    .unwrap()
            ),
        ]
        .into_iter()
        .collect::<IndexMap<FactorSourceIDFromHash, HDPathComponent>>()
    );

    // This is NOT a valid Matrix! But for the purpose of this test, it's fine.
    // We are not testing valid matrices here... we are testing the factor
    // instances provider...
    let matrix_2 = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            2,
            [bdfs.clone(), ledger.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone(), ledger.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone(), ledger.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_2 =
        SecurityStructureOfFactorSources::new(DisplayName::sample(), matrix_2);

    let (securified_bob, derivation_outcome) = os
        .__OFFLINE_ONLY_securify_account(bob.address(), &shield_2)
        .await
        .unwrap();
    assert_eq!(securified_bob.address(), bob.address());

    assert!(
        derivation_outcome.found_any_instances_in_cache_for_any_factor_source()
    );
    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source()
    );

    assert_eq!(
        securified_bob
            .try_get_secured_control()
            .unwrap()
            .security_structure
            .matrix_of_factors
            .primary_role
            .all_hd_factors()
            .into_iter()
            .map(|f| (f.factor_source_id, f.derivation_entity_index()))
            .collect::<IndexMap<FactorSourceIDFromHash, HDPathComponent>>(),
        [
            (
                bdfs.id_from_hash(),
                HDPathComponent::from_local_key_space(31, KeySpace::Securified) // Alice used 30
                    .unwrap()
            ),
            (
                ledger.id_from_hash(),
                HDPathComponent::from_local_key_space(0, KeySpace::Securified)
                    .unwrap()
            ),
        ]
        .into_iter()
        .collect::<IndexMap<FactorSourceIDFromHash, HDPathComponent>>()
    );

    let (securified_carol, derivation_outcome) = os
        .__OFFLINE_ONLY_securify_account(carol.address(), &shield_1)
        .await
        .unwrap();

    assert!(
        derivation_outcome.found_any_instances_in_cache_for_any_factor_source()
    );
    assert!(
        !derivation_outcome.derived_any_new_instance_for_any_factor_source()
    );

    assert_eq!(
        securified_carol
            .try_get_secured_control()
            .unwrap()
            .security_structure
            .matrix_of_factors
            .primary_role
            .all_hd_factors()
            .into_iter()
            .map(|f| (f.factor_source_id, f.derivation_entity_index()))
            .collect::<IndexMap<FactorSourceIDFromHash, HDPathComponent>>(),
        [
            (
                bdfs.id_from_hash(),
                HDPathComponent::from_local_key_space(32, KeySpace::Securified) // Alice used 30, Bob used 31
                    .unwrap()
            ),
            (
                arculus.id_from_hash(),
                HDPathComponent::from_local_key_space(1, KeySpace::Securified) // Alice used 0
                    .unwrap()
            ),
        ]
        .into_iter()
        .collect::<IndexMap<FactorSourceIDFromHash, HDPathComponent>>()
    );

    // CLEAR CACHE
    os.clear_cache().await;

    // This is NOT a valid Matrix! But for the purpose of this test, it's fine.
    // We are not testing valid matrices here... we are testing the factor
    // instances provider...
    let matrix_3fa = MatrixOfFactorSources {
        primary_role: PrimaryRoleWithFactorSources::with_factors(
            2,
            [bdfs.clone(), ledger.clone(), arculus.clone()],
            [],
        ),
        recovery_role: RecoveryRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone(), ledger.clone(), arculus.clone()],
        ),
        confirmation_role: ConfirmationRoleWithFactorSources::with_factors(
            0,
            [],
            [bdfs.clone(), ledger.clone(), arculus.clone()],
        ),
        number_of_days_until_auto_confirm: 1,
    };

    let shield_3fa = SecurityStructureOfFactorSources::new(
        DisplayName::sample(),
        matrix_3fa,
    );

    let (securified_diana, derivation_outcome) = os
        .__OFFLINE_ONLY_securify_account(diana.address(), &shield_3fa)
        .await
        .unwrap();

    assert!(!derivation_outcome
        .found_any_instances_in_cache_for_any_factor_source());
    assert!(derivation_outcome.derived_any_new_instance_for_any_factor_source());

    let diana_mfa_device = 33;
    let diana_mfa_arculus = 2;
    let diana_mfa_ledger = 1;

    assert_eq!(
        securified_diana
            .try_get_secured_control()
            .unwrap()
            .security_structure
            .matrix_of_factors
            .primary_role
            .all_hd_factors()
            .into_iter()
            .map(|f| (f.factor_source_id, f.derivation_entity_index()))
            .collect::<IndexMap<FactorSourceIDFromHash, HDPathComponent>>(),
        [
            (
                bdfs.id_from_hash(),
                HDPathComponent::from_local_key_space(
                    diana_mfa_device,
                    KeySpace::Securified
                )
                .unwrap()
            ),
            (
                arculus.id_from_hash(),
                HDPathComponent::from_local_key_space(
                    diana_mfa_arculus,
                    KeySpace::Securified
                )
                .unwrap()
            ),
            (
                ledger.id_from_hash(),
                HDPathComponent::from_local_key_space(
                    diana_mfa_ledger,
                    KeySpace::Securified
                )
                .unwrap()
            ),
        ]
        .into_iter()
        .collect::<IndexMap<FactorSourceIDFromHash, HDPathComponent>>()
    );

    // lets create 2 * CACHE_FILLING_QUANTITY many more accounts and securify them with
    // the same shield as Diana

    os.clear_cache().await; // CLEAR CACHE

    let (more_unnamed_accounts, _) = os.batch_create_many_accounts_with_bdfs_with_derivation_outcome_then_save_once(2 * CACHE_FILLING_QUANTITY as u16, network, "more".to_owned()).await.unwrap();

    let (many_securified_accounts, derivation_outcome) = os
        .__OFFLINE_ONLY_securify_accounts(
            more_unnamed_accounts
                .into_iter()
                .map(|a| a.address())
                .collect(),
            &shield_3fa,
        )
        .await
        .unwrap();

    assert!(
        derivation_outcome.derived_any_new_instance_for_any_factor_source(),
        "twice the cache size => derive more"
    );
    os.clear_cache().await; // CLEAR CACHE

    for index in 0..many_securified_accounts.len() {
        let securified_account = many_securified_accounts
            .clone()
            .into_iter()
            .nth(index)
            .unwrap();

        let offset = (index + 1) as u32;

        assert_eq!(
            securified_account
                .try_get_secured_control()
                .unwrap()
                .security_structure
                .matrix_of_factors
                .primary_role
                .all_hd_factors()
                .into_iter()
                .map(|f| (f.factor_source_id, f.derivation_entity_index()))
                .collect::<IndexMap<_, _>>(),
            [
                (
                    bdfs.id_from_hash(),
                    HDPathComponent::Securified(
                        SecurifiedU30::try_from(diana_mfa_device + offset)
                            .unwrap()
                    )
                ),
                (
                    arculus.id_from_hash(),
                    HDPathComponent::Securified(
                        SecurifiedU30::try_from(diana_mfa_arculus + offset)
                            .unwrap()
                    )
                ),
                (
                    ledger.id_from_hash(),
                    HDPathComponent::Securified(
                        SecurifiedU30::try_from(diana_mfa_ledger + offset)
                            .unwrap()
                    )
                ),
            ]
            .into_iter()
            .collect::<IndexMap<_, _>>()
        );
    }
}
