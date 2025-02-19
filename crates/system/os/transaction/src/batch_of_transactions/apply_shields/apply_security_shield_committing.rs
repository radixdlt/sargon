use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplySecurityShieldCommitting: Send + Sync {
    /// Host has previously called the function
    ///     `make_interaction_for_applying_security_shield`
    /// and specified the `security_shield_id` and `addresses` of the entities
    /// for which they want to apply the security shield. Which returns a Vec
    /// of TransactionManifests, one for each entity. If the entity is securified
    /// already the "variant" `RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithRecovery` is used.
    ///
    /// Host presents batch TX review UI, and user needs to select payer for each manifest,
    /// MUST be done for Personas and in case of entity being an Account, the payer might
    /// be the same account as the entity applying the shield. That information is passed
    /// when user slides to sign back to Sargon via the tuples of `ManifestWithPayerByAddress`.
    ///
    /// We will map from `Vec<Manifest>` -> `Vec<Vec<Manifest>>` where for each entity
    /// being unsecurified the inner Vec will be unchanged - one single manifest. But
    /// for each securified entity - which has a manifest which was create with `InitiateWithRecoveryCompleteWithConfirmation` variant, we will map to all variants of the [`RolesExercisableInTransactionManifestCombination`] enum.
    ///
    /// Then we will inner map of the `Vec<Vec<Manifest>>` to
    /// perform look up of all `payer` address and get the Account from
    /// Profile - and depending on if that payer account is already securified or not
    /// we will use `modify_add_lock_fee` for Unsecurified accounts and for securified
    /// accounts we will use `modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller`.
    ///
    /// Then we will build TransactionIntent for all of these - with broad enough
    /// an epoch window so that we can submit these with delay in between.
    ///
    /// We will compile them and we will start the process of signing them. Which will be the job of `SigningManager` - many instances of `SignaturesCollector` using one Role at a time.
    ///
    /// Can work with single transaction of course...
    async fn sign_and_enqueue_batch_of_transactions_applying_security_shield(
        &self,
        network_id: NetworkID,
        manifest_and_payer_tuples: Vec<ManifestWithPayerByAddress>, // TODO: Want IndexSet but not Hash
    ) -> Result<IndexSet<TransactionIntentHash>>;
}

#[async_trait::async_trait]
impl ApplySecurityShieldCommitting for SargonOS {
    async fn sign_and_enqueue_batch_of_transactions_applying_security_shield(
        &self,
        network_id: NetworkID,
        manifest_and_payer_tuples: Vec<ManifestWithPayerByAddress>, // TODO: Want IndexSet but not Hash
    ) -> Result<IndexSet<TransactionIntentHash>> {
        let committer = ApplyShieldTransactionsCommitterImpl::new(self)?;
        committer
            .commit(network_id, manifest_and_payer_tuples)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::{future::Future, pin::Pin};

    use super::*;

    #[actix_rt::test]
    async fn test() {
        let network_id = NetworkID::Mainnet;
        let mock_networking_driver =
            MockNetworkingDriver::everyones_rich(network_id);

        let bdfs_mnemonic = MnemonicWithPassphrase::sample_device();
        let os = SargonOS::boot_test_with_networking_driver_and_bdfs(
            mock_networking_driver,
            Some(bdfs_mnemonic),
        )
        .await
        .unwrap();

        let bdfs = os.main_bdfs().unwrap();
        println!("🔮 bdfs: {:?}", bdfs.factor_source_id());
        let ledger = FactorSource::sample_at(1);
        let password = FactorSource::sample_at(5);
        let off_device_mnemonic = FactorSource::sample_at(7);

        for fs in FactorSource::sample_all().into_iter() {
            if fs.factor_source_id() == bdfs.factor_source_id() {
                continue;
            }
            os.add_factor_source(fs).await.unwrap();
        }
        os.set_main_factor_source(bdfs.factor_source_id())
            .await
            .unwrap();

        let shield_builder = SecurityShieldBuilder::lenient();

        let shield = shield_builder
            .add_factor_source_to_primary_threshold(bdfs.factor_source_id())
            .add_factor_source_to_primary_threshold(password.factor_source_id())
            .add_factor_source_to_recovery_override(ledger.factor_source_id())
            .add_factor_source_to_confirmation_override(
                off_device_mnemonic.factor_source_id(),
            )
            .set_authentication_signing_factor(bdfs.factor_source_id())
            .build()
            .unwrap();

        let shield_id = shield.id();

        os.add_security_structure_of_factor_source_ids(&shield)
            .await
            .unwrap();

        // Unsecurified Account
        let alice = os
            .create_and_save_new_mainnet_account_with_main_bdfs(
                DisplayName::new("Alice").unwrap(),
            )
            .await
            .unwrap();

        // Unsecurified Account
        let bob = os
            .create_and_save_new_mainnet_account_with_main_bdfs(
                DisplayName::new("Bob").unwrap(),
            )
            .await
            .unwrap();

        // Unsecurified Account 2
        let carla = os
            .create_and_save_new_mainnet_account_with_main_bdfs(
                DisplayName::new("Carla").unwrap(),
            )
            .await
            .unwrap();

        // Unsecurified account 3
        let peter = os
            .create_and_save_new_mainnet_account_with_main_bdfs(
                DisplayName::new("Peter").unwrap(),
            )
            .await
            .unwrap();

        // Securified Account 2
        let david = Account::sample_at(3);
        os.add_account(david.clone()).await.unwrap();

        // Securified Account 3
        let emily = Account::sample_at(4);
        os.add_account(emily.clone()).await.unwrap();

        // Securified Account 4
        let frank = Account::sample_at(5);
        os.add_account(frank.clone()).await.unwrap();

        // Securified Account 5
        let mut paige = Account::sample_at(6);
        paige.display_name = DisplayName::new("Paige").unwrap();
        os.add_account(paige.clone()).await.unwrap();

        // Securified Persona
        let ziggy = Persona::sample_at(2);
        os.add_persona(ziggy.clone()).await.unwrap();

        // Securified Persona 2
        let superman = Persona::sample_at(4);
        os.add_persona(superman.clone()).await.unwrap();

        // Unsecurified Persona
        let satoshi = os
            .create_and_save_new_mainnet_persona_with_main_bdfs(
                DisplayName::new("satoshi").unwrap(),
            )
            .await
            .unwrap();

        // Unsecurified Persona 2
        let batman = os
            .create_and_save_new_mainnet_persona_with_main_bdfs(
                DisplayName::new("Batman").unwrap(),
            )
            .await
            .unwrap();

        // does not include paige and peter which are payers,
        // and payers cannot be in the list of entities to apply the shield for.
        let addresses = IndexSet::from_iter([
            alice.address_erased(),
            bob.address_erased(),
            carla.address_erased(),
            satoshi.address_erased(),
            batman.address_erased(),
            david.address_erased(),
            emily.address_erased(),
            frank.address_erased(),
            ziggy.address_erased(),
            superman.address_erased(),
        ]);

        let manifests = os
            .make_interaction_for_applying_security_shield(
                shield_id,
                addresses.clone(),
            )
            .await
            .unwrap()
            .transactions;

        // let mut manifests_iter = manifests.iter();
        let lookup_map = hacky_tmp_get_entities_applying_shield();
        let _get = |entity: AccountOrPersona,
                    account: AccountAddress|
         -> ManifestWithPayerByAddress {
            let key = EntityApplyingShieldAddress::from(entity);
            let manifest = lookup_map.get(&key).unwrap();
            ManifestWithPayerByAddress::new(
                manifest.clone(),
                Decimal192::ten(),
                account,
            )
        };

        let get_with_payer = |entity: AccountOrPersona,
                              account: &Account|
         -> ManifestWithPayerByAddress {
            _get(entity, account.address)
        };

        let get_entity_is_payer =
            |entity: Account| -> ManifestWithPayerByAddress {
                _get(entity.clone().into(), entity.address)
            };

        // ============================================
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // User reviews TXs in Radix Wallet app and
        // selects fee payer (optional) and slides to
        // sign.
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ============================================
        let manifest_and_payer_tuples = vec![
            // ~~~~~ UNSECURIFIED ENTITIES ~~~~~
            // Scenario: Alice is an Unsecurified Account paying for herself
            get_entity_is_payer(alice),
            // Scenario: Bob is an Unsecurified Account paid by Unsecurified Payer "Paige"
            get_with_payer(bob.into(), &paige),
            // Scenario: Carla is an Unsecurified Account paid by Securified Payer "Peter"
            get_with_payer(carla.into(), &peter),
            // Scenario: Satoshi is an Unsecurified Persona paid by Unsecurified Payer "Paige"
            get_with_payer(satoshi.into(), &paige),
            // Scenario: Batman is an Unsecurified Persona paid by Securified Payer "Peter"
            get_with_payer(batman.into(), &peter),
            // ~~~~~ SECURIFIED ENTITIES ~~~~~
            // Scenario: David is a Securified Account paying for himself
            get_entity_is_payer(david),
            // Scenario: Emily is a Securified Account paid by Unsecurified Payer "Paige"
            get_with_payer(emily.into(), &paige),
            // Scenario: Frank is a Securified Account paid by Securified Payer "Peter"
            get_with_payer(frank.into(), &peter),
            // Scenario: Ziggy is a Securified Persona paid by Unsecurified Payer "Paige"
            get_with_payer(ziggy.into(), &paige),
            // Scenario: Superman is a Securified Persona paid by Securified Payer "Peter"
            get_with_payer(superman.into(), &peter),
        ];

        assert_eq!(manifest_and_payer_tuples.len(), manifests.len());

        let committer = ApplyShieldTransactionsCommitterImpl::new(&os).unwrap();

        let txids = committer
            .commit(network_id, manifest_and_payer_tuples)
            .await
            .unwrap();

        assert_eq!(txids.len(), addresses.len());
    }

    #[actix_rt::test]
    async fn not_enough_xrd_for_unsecurified_account() {
        not_enough_xrd_for_unsecurified(
            EntityApplyingShield::unsecurified_account(
                UnsecurifiedAccount::sample(),
            ),
            None,
        )
        .await
    }

    #[actix_rt::test]
    async fn not_enough_xrd_for_securified_account() {
        not_enough_xrd_for_unsecurified(
            EntityApplyingShield::securified_account(
                SecurifiedAccount::sample(),
            ),
            None,
        )
        .await
    }

    #[actix_rt::test]
    async fn not_enough_xrd_for_unsecurified_persona() {
        not_enough_xrd_for_unsecurified(
            EntityApplyingShield::unsecurified_persona(
                UnsecurifiedPersona::sample(),
            ),
            Account::sample(),
        )
        .await
    }

    async fn not_enough_xrd_for_unsecurified(
        entity: EntityApplyingShield,
        fee_payer: impl Into<Option<Account>>,
    ) {
        let fee_payer = fee_payer.into();
        let fee_payer = match fee_payer {
            Some(fp) => fp,
            None => entity
                .clone()
                .into_account()
                .expect("Must specify fee payer"),
        };
        let account_to_add = fee_payer.clone();
        not_enough_xrd_for(
            |network_id| { MockNetworkingDriver::everyones_broke(network_id) },
            |os: Arc<SargonOS>| {
                Box::pin(async move {
                    match entity.entity() {
                        AccountOrPersona::AccountEntity(ref account) => {
                            os.add_account(account.clone()).await.unwrap();
                        }
                        AccountOrPersona::PersonaEntity(ref persona) => {
                            os.add_persona(persona.clone()).await.unwrap();
                        }
                    }
                       let _ = os.add_account(account_to_add.clone()).await;
                IndexSet::from_iter([entity.address()])
            })
            },
            |_, manifests| {
                let manifest_and_payer_tuples =
                    vec![ManifestWithPayerByAddress::new(
                        manifests.first().unwrap().clone(),
                        Decimal192::ten(),
                        fee_payer.clone().address,
                    )];
                manifest_and_payer_tuples
            },
            |addresses, result| {
                assert_eq!(result, Err(CommonError::UnableContributeToAcXrdVaultInsufficientBalanceOfPayer {
                    payer: fee_payer.address.to_string(),
                    vault_of_entity: addresses[0].to_string(),
                    payer_balance: "0".to_owned(),
                    needed_balance: "110".to_owned(),
                }));
            },
        )
        .await;
    }

    async fn not_enough_xrd_for(
        arrange_networking: impl FnOnce(NetworkID) -> Arc<dyn NetworkingDriver>,
        arrange_entities: impl FnOnce(
            Arc<SargonOS>,
        ) -> Pin<
            Box<dyn Future<Output = IndexSet<AddressOfAccountOrPersona>>>,
        >,
        arrange_input: impl FnOnce(
            IndexSet<AddressOfAccountOrPersona>,
            Vec<TransactionManifest>,
        ) -> Vec<ManifestWithPayerByAddress>,
        assert: impl FnOnce(
            IndexSet<AddressOfAccountOrPersona>,
            Result<IndexSet<TransactionIntentHash>>,
        ),
    ) {
        let network_id = NetworkID::Mainnet;
        let mock_networking_driver = arrange_networking(network_id);

        let os =
            SargonOS::boot_test_with_networking_driver(mock_networking_driver)
                .await
                .unwrap();

        let addresses = arrange_entities(os.clone()).await;

        let bdfs = os.main_bdfs().unwrap();

        let shield_builder = SecurityShieldBuilder::lenient();

        let shield = shield_builder
            .add_factor_source_to_primary_threshold(bdfs.factor_source_id())
            // .add_factor_source_to_primary_threshold(password.factor_source_id())
            .add_factor_source_to_recovery_override(bdfs.factor_source_id())
            .add_factor_source_to_confirmation_override(bdfs.factor_source_id())
            .set_authentication_signing_factor(bdfs.factor_source_id())
            .build()
            .unwrap();

        os.add_security_structure_of_factor_source_ids(&shield)
            .await
            .unwrap();

        let shield_id = shield.id();

        let manifests = os
            .make_interaction_for_applying_security_shield(
                shield_id,
                addresses.clone(),
            )
            .await
            .unwrap()
            .transactions;
        let manifests = manifests
            .into_iter()
            .map(|uvm| uvm.manifest(network_id).unwrap())
            .collect_vec();
        let manifests_and_payer_tuples =
            arrange_input(addresses.clone(), manifests);

        let committer = ApplyShieldTransactionsCommitterImpl::new(&os).unwrap();

        let result = committer
            .commit(network_id, manifests_and_payer_tuples)
            .await;

        assert(addresses, result)
    }
}
