#[cfg(test)]
mod integration_tests {
    use std::time::Duration;

    use actix_rt::time::timeout;
    use sargon::prelude::*;
    use std::collections::HashMap;

    const MAX: Duration = Duration::from_secs(5);

    #[cfg(test)]
    pub fn new_gateway_client(network_id: NetworkID) -> GatewayClient {
        let driver = RustNetworkingDriver::new();
        GatewayClient::new(driver, network_id)
    }

    #[actix_rt::test]
    async fn test_xrd_balance_of_account_or_zero() {
        let gateway_client = new_gateway_client(NetworkID::Mainnet);
        let sut = gateway_client
            .xrd_balance_of_account_or_zero(AccountAddress::sample_mainnet());

        let xrd_balance = timeout(MAX, sut).await.unwrap().unwrap();
        assert!(xrd_balance >= Decimal192::one())
    }

    #[actix_rt::test]
    async fn test_account_deleted() {
        let gateway_client = new_gateway_client(NetworkID::Stokenet);
        let account_address = AccountAddress::try_from_bech32(
            "account_tdx_2_12ywudmhgrlhvxsukpxn9pqr3dzv4la9upszfsms0pz0sh3lu6erxux"
        ).unwrap();

        let result = gateway_client
            .check_accounts_are_deleted(
                NetworkID::Stokenet,
                vec![account_address],
            )
            .await
            .unwrap();

        assert!(result.get(&account_address).unwrap())
    }

    #[actix_rt::test]
    async fn test_xrd_balance_of_account_or_zero_is_zero_for_unknown_mainnet() {
        let network_id = NetworkID::Mainnet;
        let gateway_client = new_gateway_client(network_id);
        let sut = gateway_client
            .xrd_balance_of_account_or_zero(AccountAddress::random(network_id));

        let xrd_balance = timeout(MAX, sut).await.unwrap().unwrap();
        assert_eq!(xrd_balance, Decimal192::zero());
    }

    #[actix_rt::test]
    async fn test_xrd_balance_of_account_or_zero_is_zero_for_unknown_stokenet()
    {
        let network_id = NetworkID::Stokenet;
        let gateway_client = new_gateway_client(network_id);
        let sut = gateway_client
            .xrd_balance_of_account_or_zero(AccountAddress::random(network_id));

        let xrd_balance = timeout(MAX, sut).await.unwrap().unwrap();
        assert_eq!(xrd_balance, Decimal192::zero());
    }

    #[actix_rt::test]
    async fn test_epoch() {
        let gateway_client = new_gateway_client(NetworkID::Mainnet);
        let sut = gateway_client.current_epoch();
        let epoch = timeout(MAX, sut).await.unwrap().unwrap();
        assert!(epoch > Epoch::from(0));
    }

    #[actix_rt::test]
    async fn dry_run_transaction() {
        // ARRANGE
        let network_id = NetworkID::Mainnet;
        let gateway_client = new_gateway_client(network_id);
        let start_epoch_inclusive =
            timeout(MAX, gateway_client.current_epoch())
                .await
                .unwrap()
                .unwrap();

        let from = AccountAddress::sample_mainnet();
        let to = AccountAddress::sample_mainnet_other();
        let resource = ResourceAddress::sample();
        let amount = Decimal192::one();
        let transfers = PerRecipientAssetTransfers::new(
            from,
            [PerRecipientAssetTransfer::new(
                AccountOrAddressOf::AddressOfExternalAccount { value: to },
                [PerRecipientFungibleTransfer::new(
                    resource, amount, true, None,
                )],
                [],
            )],
        );

        let manifest = TransactionManifest::per_recipient_transfers(transfers);

        let end_epoch_exclusive = Epoch::from(start_epoch_inclusive.0 + 10u64);
        let notary_public_key = Ed25519PublicKey::sample();
        let header = TransactionHeader::new(
            network_id,
            start_epoch_inclusive,
            end_epoch_exclusive,
            Nonce::random(),
            notary_public_key,
            true,
            0,
        );

        let intent =
            TransactionIntent::new(header, manifest.clone(), Message::None)
                .unwrap();

        let sut = gateway_client.dry_run_transaction(
            intent, vec![
                Ed25519PublicKey::from_hex(
                    "48d24f09b43d50f3acd58cf8509a57c8f306d94b945bd9b7e6ebcf6691eed3b6".to_owned()
                ).unwrap().into()
            ],
        );

        // ACT
        let engine_toolkit_receipt =
            timeout(MAX, sut).await.unwrap().unwrap().unwrap();
        let execution_summary =
            manifest.execution_summary(engine_toolkit_receipt).unwrap();

        // ASSERT
        assert_eq!(
            execution_summary.addresses_of_accounts_requiring_auth,
            vec![from]
        );
        assert_eq!(
            execution_summary.deposits,
            HashMap::<_, _>::from_iter([(
                to,
                vec![ResourceIndicator::fungible(
                    resource,
                    FungibleResourceIndicator::new_guaranteed(amount)
                )]
            )])
        );
    }

    async fn submit_tx_use_faucet(
        private_key: impl Into<PrivateKey>,
        network_id: NetworkID,
    ) -> Result<(AccountAddress, TransactionIntentHash)> {
        let private_key = private_key.into();
        // ARRANGE
        let gateway_client = new_gateway_client(network_id);

        let public_key = private_key.public_key();

        println!("✨ public_key: {}", &public_key);
        let address =
            AccountAddress::new_from_public_key(public_key, network_id);
        let manifest = TransactionManifest::faucet(true, &address);

        let start_epoch_inclusive =
            timeout(MAX, gateway_client.current_epoch())
                .await
                .unwrap()
                .unwrap();

        let end_epoch_exclusive = Epoch::from(start_epoch_inclusive.0 + 10u64);

        let header = TransactionHeader::new(
            network_id,
            start_epoch_inclusive,
            end_epoch_exclusive,
            Nonce::random(),
            public_key,
            false,
            0,
        );

        let intent =
            TransactionIntent::new(header, manifest.clone(), Message::None)
                .unwrap();

        let intent_hash = intent.transaction_intent_hash();
        println!("✨ intent hash: {}", &intent_hash);
        let intent_signature =
            private_key.sign_transaction_intent_hash(&intent_hash);

        let signed_intent = SignedIntent::new(
            intent,
            IntentSignatures::new([intent_signature]),
        )
        .unwrap();

        let notary_signature = private_key.notarize_hash(&signed_intent.hash());

        let notarized_transaction =
            NotarizedTransaction::new(signed_intent, notary_signature).unwrap();

        let tx_id = timeout(
            MAX,
            gateway_client.submit_notarized_transaction(notarized_transaction),
        )
        .await
        .unwrap()
        .unwrap();

        Ok((address, tx_id))
    }

    #[actix_rt::test]
    async fn submit_transaction_use_faucet() {
        let network_id = NetworkID::Stokenet;
        let private_key = Ed25519PrivateKey::generate();
        println!("🔮 private_key: {}", &private_key.to_hex());
        let (account_address, tx_id) =
            submit_tx_use_faucet(private_key, network_id).await.unwrap();
        println!("🔮 account_address: {}, tx_id: {}", account_address, tx_id);
    }

    #[actix_rt::test]
    async fn submit_transaction_use_faucet_secp256k1() {
        let network_id = NetworkID::Stokenet;
        let private_key = Secp256k1PrivateKey::generate();
        println!("🔮 private_key: {}", &private_key.to_hex());
        let (account_address, tx_id) =
            submit_tx_use_faucet(private_key, network_id).await.unwrap();
        println!("🔮 account_address: {}, tx_id: {}", account_address, tx_id);
        assert!(account_address.is_legacy_address())
    }

    #[actix_rt::test]
    async fn test_dapp_metadata() {
        let gumball_address = AccountAddress::try_from_bech32(
            "account_tdx_2_129nx5lgkk3fz9gqf3clppeljkezeyyymqqejzp97tpk0r8els7hg3j",
        )
            .unwrap();
        let gateway_client = new_gateway_client(NetworkID::Stokenet);
        let sut = gateway_client.fetch_dapp_metadata(gumball_address);

        let response = timeout(MAX, sut).await.unwrap().unwrap();
        let icon_url = response.get_icon_url();
        assert_eq!(
            icon_url,
            Some(
                Url::parse(
                    "https://stokenet-gumball-club.radixdlt.com/assets/gumball-club.png"
                )
                    .unwrap()
            )
        );
    }

    #[actix_rt::test]
    async fn get_transaction_status() {
        let network_id = NetworkID::Stokenet;
        let gateway_client = new_gateway_client(network_id);
        let private_key = Ed25519PrivateKey::generate();
        let (_, tx_id) =
            submit_tx_use_faucet(private_key, network_id).await.unwrap();

        let status_response =
            timeout(MAX, gateway_client.get_transaction_status(tx_id))
                .await
                .unwrap()
                .unwrap();

        assert_eq!(status_response.error_message, None);
        let status = status_response
            .known_payloads
            .first()
            .and_then(|payload| payload.payload_status.clone())
            .unwrap();
        assert_eq!(status, TransactionStatusResponsePayloadStatus::Pending);
    }

    mod signing {
        use super::*;
        use radix_common::prelude::indexmap::IndexSet;
        use radix_transactions::prelude::ManifestBuilder;
        use std::{collections::HashSet, sync::Arc};

        #[test]
        fn extract_when_account_is_unknown() {
            let profile = Profile::sample();

            let manifest_builder = ManifestBuilder::new();
            let mut manifest = TransactionManifest::sargon_built(
                manifest_builder,
                NetworkID::Mainnet,
            );
            manifest = manifest
                .modify_add_lock_fee(
                    &AccountAddress::sample_stokenet(),
                    Some(Decimal192::one()),
                )
                .unwrap();
            let manifest_summary = manifest.summary().unwrap();

            let result = ExtractorOfEntitiesRequiringAuth::extract(
                &profile,
                manifest_summary,
            );

            assert_eq!(result, Ok(IndexSet::new()));
        }

        #[test]
        fn extract_when_persona_is_unknown() {
            let profile = Profile::sample();

            let manifest = TransactionManifest::set_owner_keys_hashes(
                &Persona::sample_mainnet_third().address.into(),
                vec![PublicKeyHash::sample()],
            );
            let manifest_summary = manifest.summary().unwrap();

            let result = ExtractorOfEntitiesRequiringAuth::extract(
                &profile,
                manifest_summary,
            );

            assert!(matches!(result, Err(CommonError::UnknownPersona)));
        }

        #[test]
        fn extract_when_no_entities_require_auth() {
            let profile = Profile::sample();

            let manifest_builder = ManifestBuilder::new();
            let manifest = TransactionManifest::sargon_built(
                manifest_builder,
                NetworkID::Mainnet,
            );
            let manifest_summary = manifest.summary().unwrap();

            let result = ExtractorOfEntitiesRequiringAuth::extract(
                &profile,
                manifest_summary,
            );

            assert!(result.is_ok());
            assert!(result.unwrap().is_empty());
        }

        #[test]
        fn extract_entities_success() {
            let profile = Profile::sample();
            let account = Account::sample_mainnet();
            let persona = Persona::sample_mainnet();

            let manifest = TransactionManifest::set_owner_keys_hashes(
                &persona.address.into(),
                vec![PublicKeyHash::sample()],
            )
            .modify_add_lock_fee(&account.address, Some(Decimal192::one()))
            .unwrap();
            let manifest_summary = manifest.summary().unwrap();

            let result = ExtractorOfEntitiesRequiringAuth::extract(
                &profile,
                manifest_summary,
            );

            assert_eq!(
                result,
                Ok(IndexSet::from_iter(vec![
                    AccountOrPersona::from(account),
                    AccountOrPersona::from(persona),
                ]))
            );
        }

        pub struct TestTransactionSignInteractor;

        impl TestTransactionSignInteractor {
            async fn sign_mono(
                &self,
                factor_source_id: FactorSourceIDFromHash,
                request: &SignRequest<TransactionIntent>,
                transactions_to_sign: &IndexSet<
                    TransactionSignRequestInput<TransactionIntent>,
                >,
            ) -> Result<FactorOutcome<TransactionIntentHash>> {
                if request
                    .invalid_transactions_if_factor_neglected(&factor_source_id)
                    .is_empty()
                {
                    return Ok(FactorOutcome::skipped(factor_source_id));
                }

                let signatures = transactions_to_sign
                    .iter()
                    .flat_map(|per_transaction| {
                        per_transaction
                            .signature_inputs()
                            .iter()
                            .map(|x| HDSignature::fake_sign_by_looking_up_mnemonic_amongst_samples(x.clone()))
                            .collect::<IndexSet<_>>()
                    })
                    .collect::<IndexSet<HDSignature<TransactionIntentHash>>>();

                FactorOutcome::signed(signatures)
            }
        }

        #[test]
        fn profile_with_unknown_account() {
            let res = SignaturesCollector::new(
                SigningFinishEarlyStrategy::default(),
                [TransactionIntent::sample_entities_requiring_auth(
                    [&Account::sample_at(0)],
                    [],
                )],
                Arc::new(TestSignInteractor::new(
                    SimulatedUser::prudent_no_fail(),
                )),
                &Profile::sample_from(IndexSet::new(), [], []),
                SigningPurpose::sign_transaction_primary(),
            );
            assert!(res.is_ok());
        }

        #[test]
        fn invalid_profile_unknown_persona() {
            let res = SignaturesCollector::new(
                SigningFinishEarlyStrategy::default(),
                [TransactionIntent::sample_entities_requiring_auth(
                    [],
                    [&Persona::sample_at(0)],
                )],
                Arc::new(TestSignInteractor::new(
                    SimulatedUser::prudent_no_fail(),
                )),
                &Profile::sample_from(IndexSet::new(), [], []),
                SigningPurpose::sign_transaction_primary(),
            );
            assert!(matches!(res, Err(CommonError::UnknownPersona)));
        }

        #[actix_rt::test]
        async fn valid_profile() {
            let factors_sources = FactorSource::sample_all();
            let persona = Persona::sample_at(0);

            let collector = SignaturesCollector::new(
                SigningFinishEarlyStrategy::default(),
                [TransactionIntent::sample_entities_requiring_auth(
                    [],
                    [&persona],
                )],
                Arc::new(TestSignInteractor::new(
                    SimulatedUser::prudent_no_fail(),
                )),
                &Profile::sample_from(factors_sources, [], [&persona]),
                SigningPurpose::sign_transaction_primary(),
            )
            .unwrap();
            let outcome = collector.collect_signatures().await.unwrap();
            assert!(outcome.successful())
        }

        #[actix_rt::test]
        async fn continues_even_with_failed_tx_when_configured_to() {
            let factor_sources = &FactorSource::sample_all();
            let a0 = &Account::sample_at(0);
            let a1 = &Account::sample_at(1);

            let t0 =
                TransactionIntent::sample_entities_requiring_auth([a1], []);
            let t1 =
                TransactionIntent::sample_entities_requiring_auth([a0], []);

            let profile =
                Profile::sample_from(factor_sources.clone(), [a0, a1], []);

            let collector = SignaturesCollector::new(
                SigningFinishEarlyStrategy::new(
                    WhenAllTransactionsAreValid(
                        SignaturesCollectingContinuation::FinishEarly,
                    ),
                    WhenSomeTransactionIsInvalid(
                        SignaturesCollectingContinuation::Continue,
                    ),
                ),
                [t0.clone(), t1.clone()],
                Arc::new(TestSignInteractor::new(
                    SimulatedUser::prudent_with_failures(
                        SimulatedFailures::with_simulated_failures([
                            FactorSourceIDFromHash::sample_at(1),
                        ]),
                    ),
                )),
                &profile,
                SigningPurpose::sign_transaction_primary(),
            )
            .unwrap();

            let outcome = collector.collect_signatures().await.unwrap();
            assert!(!outcome.successful());
            assert_eq!(outcome.failed_transactions().len(), 1);
            assert_eq!(outcome.successful_transactions().len(), 1);
        }

        #[actix_rt::test]
        async fn continues_even_when_all_valid_if_configured_to() {
            let test =
                async move |when_all_valid: WhenAllTransactionsAreValid,
                            expected_sig_count: usize| {
                    let factor_sources = &FactorSource::sample_all();
                    let a5 = &Account::sample_at(5);

                    let t0 = TransactionIntent::sample_entities_requiring_auth(
                        [a5],
                        [],
                    );

                    let profile =
                        Profile::sample_from(factor_sources.clone(), [a5], []);

                    let collector = SignaturesCollector::new(
                        SigningFinishEarlyStrategy::new(
                            when_all_valid,
                            WhenSomeTransactionIsInvalid::default(),
                        ),
                        [t0.clone()],
                        Arc::new(TestSignInteractor::new(
                            SimulatedUser::prudent_no_fail(),
                        )),
                        &profile,
                        SigningPurpose::sign_transaction_primary(),
                    )
                    .unwrap();

                    let outcome = collector.collect_signatures().await.unwrap();
                    assert!(outcome.successful());
                    assert_eq!(
                        outcome.signatures_of_successful_transactions().len(),
                        expected_sig_count
                    );
                };

            test(
                WhenAllTransactionsAreValid(
                    SignaturesCollectingContinuation::FinishEarly,
                ),
                1,
            )
            .await;
            test(
                WhenAllTransactionsAreValid(
                    SignaturesCollectingContinuation::Continue,
                ),
                2,
            )
            .await;
        }

        #[test]
        fn test_profile() {
            let factor_sources = &FactorSource::sample_all();
            let a0 = &Account::sample_at(0);
            let a1 = &Account::sample_at(1);
            let a2 = &Account::sample_at(2);
            let a6 = &Account::sample_at(6);

            let p0 = &Persona::sample_at(0);
            let p1 = &Persona::sample_at(1);
            let p2 = &Persona::sample_at(2);
            let p6 = &Persona::sample_at(6);

            let t0 = TransactionIntent::sample_entities_requiring_auth(
                [a0, a1],
                [p0, p1],
            );
            let t1 = TransactionIntent::sample_entities_requiring_auth(
                [a0, a1, a2],
                [],
            );
            let t2 = TransactionIntent::sample_entities_requiring_auth(
                [],
                [p0, p1, p2],
            );
            let t3 =
                TransactionIntent::sample_entities_requiring_auth([a6], [p6]);

            let profile = Profile::sample_from(
                factor_sources.clone(),
                [a0, a1, a2, a6],
                [p0, p1, p2, p6],
            );

            let collector = SignaturesCollector::new(
                SigningFinishEarlyStrategy::default(),
                [t0.clone(), t1.clone(), t2.clone(), t3.clone()],
                Arc::new(TestSignInteractor::new(
                    SimulatedUser::prudent_no_fail(),
                )),
                &profile,
                SigningPurpose::sign_transaction_primary(),
            )
            .unwrap();

            let petitions = collector.petitions();

            assert_eq!(
                petitions
                    .txid_to_petition
                    .read()
                    .expect("Petitions lock should not have been poisoned")
                    .len(),
                4
            );

            {
                let petitions_ref = petitions
                    .txid_to_petition
                    .read()
                    .expect("Petitions lock should not have been poisoned");
                let petition =
                    petitions_ref.get(&t3.transaction_intent_hash()).unwrap();
                let for_entities = petition
                    .for_entities
                    .read()
                    .expect("PetitionForTransaction lock should not have been poisoned.")
                    .clone();
                let pet6 = for_entities.get(&a6.address.into()).unwrap();

                let paths6 = pet6
                    .all_factor_instances()
                    .iter()
                    .map(|f| f.factor_instance().derivation_path())
                    .collect_vec();

                pretty_assertions::assert_eq!(
                    paths6,
                    repeat_n(
                        DerivationPath::from(AccountPath::new(
                            NetworkID::Mainnet,
                            CAP26KeyKind::TransactionSigning,
                            Hardened::from_local_key_space(
                                U31::try_from(6u32).unwrap(),
                                IsSecurified(true)
                            )
                            .unwrap()
                        )),
                        5
                    )
                    .collect_vec()
                );
            }

            assert_petition(
                &petitions,
                &t0,
                HashMap::from_iter([
                    (
                        a0.address.into(),
                        HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                    ),
                    (
                        a1.address.into(),
                        HashSet::just(FactorSourceIDFromHash::sample_at(1)),
                    ),
                    (
                        p0.address.into(),
                        HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                    ),
                    (
                        p1.address.into(),
                        HashSet::just(FactorSourceIDFromHash::sample_at(1)),
                    ),
                ]),
                HashMap::new(),
            );

            assert_petition(
                &petitions,
                &t1,
                HashMap::from_iter([
                    (
                        a0.address.into(),
                        HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                    ),
                    (
                        a1.address.into(),
                        HashSet::just(FactorSourceIDFromHash::sample_at(1)),
                    ),
                    (
                        a2.address.into(),
                        HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                    ),
                ]),
                HashMap::new(),
            );

            assert_petition(
                &petitions,
                &t2,
                HashMap::from_iter([
                    (
                        p0.address.into(),
                        HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                    ),
                    (
                        p1.address.into(),
                        HashSet::just(FactorSourceIDFromHash::sample_at(1)),
                    ),
                    (
                        p2.address.into(),
                        HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                    ),
                ]),
                HashMap::new(),
            );

            assert_petition(
                &petitions,
                &t3,
                HashMap::from_iter([
                    (
                        a6.address.into(),
                        HashSet::from_iter([
                            FactorSourceIDFromHash::sample_at(0),
                            FactorSourceIDFromHash::sample_at(3),
                            FactorSourceIDFromHash::sample_at(5),
                        ]),
                    ),
                    (
                        p6.address.into(),
                        HashSet::from_iter([
                            FactorSourceIDFromHash::sample_at(0),
                            FactorSourceIDFromHash::sample_at(3),
                            FactorSourceIDFromHash::sample_at(5),
                        ]),
                    ),
                ]),
                HashMap::from_iter([
                    (
                        a6.address.into(),
                        HashSet::from_iter([
                            FactorSourceIDFromHash::sample_at(1),
                            FactorSourceIDFromHash::sample_at(4),
                        ]),
                    ),
                    (
                        p6.address.into(),
                        HashSet::from_iter([
                            FactorSourceIDFromHash::sample_at(1),
                            FactorSourceIDFromHash::sample_at(4),
                        ]),
                    ),
                ]),
            );
        }

        #[async_trait::async_trait]
        impl SignInteractor<TransactionIntent> for TestTransactionSignInteractor {
            async fn sign(
                &self,
                request: SignRequest<TransactionIntent>,
            ) -> Result<SignResponse<TransactionIntentHash>> {
                let mut per_factor_outcome = IndexMap::<
                    FactorSourceIDFromHash,
                    FactorOutcome<TransactionIntentHash>,
                >::new();

                for (factor_source_id, inputs) in
                    request.per_factor_source.iter()
                {
                    let outcome = self
                        .sign_mono(
                            *factor_source_id,
                            &request,
                            &inputs.per_transaction,
                        )
                        .await?;

                    per_factor_outcome.insert(*factor_source_id, outcome);
                }

                SignResponse::new_from_outcomes(per_factor_outcome)
            }
        }

        #[actix_rt::test]
        async fn valid() {
            type FI = HierarchicalDeterministicFactorInstance;

            let f0 = FactorSource::sample_ledger();
            let f1 = FactorSource::sample_device_babylon();
            let f2 = FactorSource::sample_device_babylon_other();
            let f3 = FactorSource::sample_arculus();
            let f4 = FactorSource::sample_off_device();

            let alice = Account::sample_securified_mainnet(
                "Alice",
                0,
                HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(0),
                || {
                    let i = Hardened::from_local_key_space(0u32, IsSecurified(true))
                        .unwrap();
                    GeneralRoleWithHierarchicalDeterministicFactorInstances::with_factors_and_role(
                        RoleKind::Primary,
                        [
                            FI::sample_mainnet_tx_account(i, *f0.factor_source_id().as_hash().unwrap()), // SKIPPED
                            FI::sample_mainnet_tx_account(i, *f1.factor_source_id().as_hash().unwrap()),
                            FI::sample_mainnet_tx_account(i, *f2.factor_source_id().as_hash().unwrap()),
                        ],
                        2,
                        []
                    ).unwrap()
                },
            );

            let bob = Account::sample_securified_mainnet(
                "Bob",
                1,
                HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(1),
                || {
                    let i = Hardened::from_local_key_space(1u32, IsSecurified(true))
                        .unwrap();
                    GeneralRoleWithHierarchicalDeterministicFactorInstances::with_factors_and_role(
                        RoleKind::Primary,
                        [], 0,
                        [
                        FI::sample_mainnet_tx_account(
                            i,
                            *f3.factor_source_id().as_hash().unwrap(),
                        )
                    ]).unwrap()
                },
            );

            let carol = Account::sample_securified_mainnet(
                "Carol",
                3,
                HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(2),
                || {
                    let i = Hardened::from_local_key_space(2u32, IsSecurified(true))
                        .unwrap();
                    GeneralRoleWithHierarchicalDeterministicFactorInstances::with_factors_and_role(
                        RoleKind::Primary,
                        [FI::sample_mainnet_tx_account(
                            i,
                            *f2.factor_source_id().as_hash().unwrap(),
                        )],
                        1,
                        [FI::sample_mainnet_tx_account(
                            i,
                            *f4.factor_source_id().as_hash().unwrap(),
                        )],
                    ).unwrap()
                },
            );

            let satoshi = Persona::sample_unsecurified_mainnet(
                "Satoshi",
                HierarchicalDeterministicFactorInstance::sample_mainnet_tx_identity(
                    Hardened::from_local_key_space(0u32, IsSecurified(true)).unwrap(),
                    *f4.factor_source_id().as_hash().unwrap(),
                ),
            );

            let tx0 = TransactionIntent::sample_entity_addresses_requiring_auth(
                [alice.address],
                [],
            );
            let tx1 = TransactionIntent::sample_entity_addresses_requiring_auth(
                [alice.address, bob.address, carol.address],
                [satoshi.address],
            );
            let tx2 = TransactionIntent::sample_entity_addresses_requiring_auth(
                [bob.address],
                [satoshi.address],
            );

            let transactions = [tx0, tx1, tx2];

            let profile = Profile::sample_from(
                [f0.clone(), f1, f2, f3, f4],
                [&alice, &bob, &carol],
                [&satoshi],
            );

            let collector = SignaturesCollector::new(
                SigningFinishEarlyStrategy::default(),
                transactions,
                Arc::new(TestTransactionSignInteractor),
                &profile,
                SigningPurpose::sign_transaction_primary(),
            )
            .unwrap();

            let outcome = collector.collect_signatures().await.unwrap();

            assert!(outcome.successful());
            assert_eq!(
                outcome.signatures_of_successful_transactions().len(),
                10
            );
            assert_eq!(
                outcome.ids_of_neglected_factor_sources(),
                IndexSet::<FactorSourceIDFromHash>::just(
                    *f0.factor_source_id().as_hash().unwrap()
                )
            );
        }

        mod multi_tx {
            use std::collections::HashSet;

            use super::*;

            async fn multi_accounts_multi_personas_all_single_factor_controlled_with_sim_user(
                sim: SimulatedUser<TransactionIntent>,
            ) {
                let factor_sources = &FactorSource::sample_all();
                let a0 = Account::sample_at(0);
                let a1 = Account::sample_at(1);
                let a2 = Account::sample_at(2);

                let p0 = Persona::sample_at(0);
                let p1 = Persona::sample_at(1);
                let p2 = Persona::sample_at(2);

                let t0 = TransactionIntent::sample_entities_requiring_auth(
                    [&a0, &a1],
                    [&p0, &p1],
                );
                let t1 = TransactionIntent::sample_entities_requiring_auth(
                    [&a0, &a1, &a2],
                    [],
                );
                let t2 = TransactionIntent::sample_entities_requiring_auth(
                    [],
                    [&p0, &p1, &p2],
                );

                let profile = Profile::sample_from(
                    factor_sources.clone(),
                    [&a0, &a1, &a2],
                    [&p0, &p1, &p2],
                );

                let collector = SignaturesCollector::new(
                    SigningFinishEarlyStrategy::default(),
                    [t0.clone(), t1.clone(), t2.clone()],
                    Arc::new(TestSignInteractor::new(sim)),
                    &profile,
                    SigningPurpose::sign_transaction_primary(),
                )
                .unwrap();

                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                assert!(outcome.failed_transactions().is_empty());
                assert_eq!(
                    outcome.signatures_of_successful_transactions().len(),
                    10
                );
                assert_eq!(
                    outcome
                        .successful_transactions()
                        .into_iter()
                        .map(|t| t.signable_id)
                        .collect::<HashSet<_>>(),
                    HashSet::from_iter([
                        t0.clone().transaction_intent_hash(),
                        t1.clone().transaction_intent_hash(),
                        t2.clone().transaction_intent_hash(),
                    ])
                );
                let st0 = outcome
                    .successful_transactions()
                    .into_iter()
                    .find(|st| st.signable_id == t0.transaction_intent_hash())
                    .unwrap();

                assert_eq!(
                    st0.signatures
                        .clone()
                        .into_iter()
                        .map(|s| s.owned_factor_instance().owner)
                        .collect::<HashSet<_>>(),
                    HashSet::from_iter([
                        AddressOfAccountOrPersona::from(a0.address),
                        AddressOfAccountOrPersona::from(a1.address),
                        AddressOfAccountOrPersona::from(p0.address),
                        AddressOfAccountOrPersona::from(p1.address),
                    ])
                );

                let st1 = outcome
                    .successful_transactions()
                    .into_iter()
                    .find(|st| st.signable_id == t1.transaction_intent_hash())
                    .unwrap();

                assert_eq!(
                    st1.signatures
                        .clone()
                        .into_iter()
                        .map(|s| s.owned_factor_instance().owner)
                        .collect::<HashSet<_>>(),
                    HashSet::from_iter([
                        AddressOfAccountOrPersona::from(a0.address),
                        AddressOfAccountOrPersona::from(a1.address),
                        AddressOfAccountOrPersona::from(a2.address),
                    ])
                );

                let st2 = outcome
                    .successful_transactions()
                    .into_iter()
                    .find(|st| st.signable_id == t2.transaction_intent_hash())
                    .unwrap();

                assert_eq!(
                    st2.signatures
                        .clone()
                        .into_iter()
                        .map(|s| s.owned_factor_instance().owner)
                        .collect::<HashSet<_>>(),
                    HashSet::from_iter([
                        AddressOfAccountOrPersona::from(p0.address),
                        AddressOfAccountOrPersona::from(p1.address),
                        AddressOfAccountOrPersona::from(p2.address),
                    ])
                );

                // Assert sorted in increasing "friction order".
                assert_eq!(
                    outcome
                        .signatures_of_successful_transactions()
                        .iter()
                        .map(|f| { f.factor_source_id().kind })
                        .collect::<IndexSet::<FactorSourceKind>>(),
                    IndexSet::<FactorSourceKind>::from_iter([
                        FactorSourceKind::Device,
                        FactorSourceKind::LedgerHQHardwareWallet
                    ])
                );
            }

            #[derive(Clone, Debug)]
            struct Vector {
                simulated_user: SimulatedUser<TransactionIntent>,
                expected: Expected,
            }
            #[derive(Clone, Debug, PartialEq, Eq)]
            struct Expected {
                successful_txs_signature_count: usize,
                signed_factor_source_kinds: IndexSet<FactorSourceKind>,
                expected_neglected_factor_source_count: usize,
            }
            async fn multi_securified_entities_with_sim_user(vector: Vector) {
                let factor_sources = &FactorSource::sample_all();

                let a4 = &Account::sample_at(4);
                let a5 = &Account::sample_at(5);
                let a6 = &Account::sample_at(6);

                let p4 = &Persona::sample_at(4);
                let p5 = &Persona::sample_at(5);
                let p6 = &Persona::sample_at(6);

                let t0 = TransactionIntent::sample_entities_requiring_auth(
                    [a5],
                    [p5],
                );
                let t1 = TransactionIntent::sample_entities_requiring_auth(
                    [a4, a5, a6],
                    [],
                );
                let t2 = TransactionIntent::sample_entities_requiring_auth(
                    [a4, a6],
                    [p4, p6],
                );
                let t3 = TransactionIntent::sample_entities_requiring_auth(
                    [],
                    [p4, p5, p6],
                );

                let profile = Profile::sample_from(
                    factor_sources.clone(),
                    [a4, a5, a6],
                    [p4, p5, p6],
                );

                let collector = SignaturesCollector::new(
                    SigningFinishEarlyStrategy::default(),
                    [t0.clone(), t1.clone(), t2.clone(), t3.clone()],
                    Arc::new(TestSignInteractor::new(vector.simulated_user)),
                    &profile,
                    SigningPurpose::sign_transaction_primary(),
                )
                .unwrap();

                let outcome = collector.collect_signatures().await.unwrap();

                assert_eq!(
                    outcome.neglected_factor_sources().len(),
                    vector.expected.expected_neglected_factor_source_count
                );

                assert!(outcome.successful());
                assert!(outcome.failed_transactions().is_empty());
                assert_eq!(
                    outcome.signatures_of_successful_transactions().len(),
                    vector.expected.successful_txs_signature_count
                );
                assert_eq!(
                    outcome
                        .successful_transactions()
                        .into_iter()
                        .map(|t| t.signable_id)
                        .collect::<HashSet<_>>(),
                    HashSet::from_iter([
                        t0.clone().transaction_intent_hash(),
                        t1.clone().transaction_intent_hash(),
                        t2.clone().transaction_intent_hash(),
                        t3.clone().transaction_intent_hash(),
                    ])
                );

                // Assert sorted in increasing "friction order".
                assert_eq!(
                    outcome
                        .signatures_of_successful_transactions()
                        .iter()
                        .map(|f| { f.factor_source_id().kind })
                        .collect::<IndexSet::<FactorSourceKind>>(),
                    vector.expected.signed_factor_source_kinds
                );
            }

            mod with_failure {
                use std::{cell::RefCell, rc::Rc};

                use super::*;

                #[actix_rt::test]
                async fn multi_securified_entities() {
                    multi_securified_entities_with_sim_user(Vector {
                        simulated_user: SimulatedUser::prudent_with_failures(
                            SimulatedFailures::with_simulated_failures([
                                FactorSourceIDFromHash::sample_at(1),
                            ]),
                        ),
                        expected: Expected {
                            successful_txs_signature_count: 24,
                            // We always end early
                            // `Device` FactorSourceKind never got used since it
                            // we are done after Passphrase.
                            signed_factor_source_kinds: IndexSet::<
                                FactorSourceKind,
                            >::from_iter(
                                [
                                FactorSourceKind::ArculusCard,
                                FactorSourceKind::Password,
                            ]
                            ),
                            expected_neglected_factor_source_count: 1,
                        },
                    })
                    .await;
                }

                #[actix_rt::test]
                async fn failed_threshold_successful_override() {
                    let factor_sources = &FactorSource::sample_all();
                    let a9 = &Account::sample_at(9);
                    let tx0 = TransactionIntent::sample_entities_requiring_auth(
                        [a9],
                        [],
                    );

                    let all_transactions = [tx0.clone()];

                    let profile =
                        Profile::sample_from(factor_sources.clone(), [a9], []);

                    let collector = SignaturesCollector::new(
                        SigningFinishEarlyStrategy::default(),
                        all_transactions,
                        Arc::new(TestSignInteractor::new(
                            SimulatedUser::prudent_with_failures(
                                SimulatedFailures::with_simulated_failures([
                                    FactorSourceIDFromHash::sample_at(1),
                                ]),
                            ),
                        )),
                        &profile,
                        SigningPurpose::sign_transaction_primary(),
                    )
                    .unwrap();

                    let outcome = collector.collect_signatures().await.unwrap();
                    assert!(outcome.successful());
                    assert_eq!(
                        outcome
                            .successful_transactions()
                            .into_iter()
                            .map(|t| t.signable_id.clone())
                            .collect_vec(),
                        vec![tx0.clone().transaction_intent_hash()]
                    );
                    assert_eq!(
                        outcome
                            .all_signatures()
                            .into_iter()
                            .map(|s| s.factor_source_id())
                            .collect_vec(),
                        vec![FactorSourceIDFromHash::sample_at(8)]
                    );
                }

                #[actix_rt::test]
                async fn many_failing_tx() {
                    let factor_sources = &FactorSource::sample_all();
                    let a0 = &Account::sample_at(0);
                    let p3 = &Persona::sample_at(3);
                    let tx = TransactionIntent::sample_entities_requiring_auth(
                        [],
                        [p3],
                    );

                    // In need of different PublicKeyHashes so the IntentHash of each transaction is different
                    let make_random_pk_hash = || {
                        let private_key = Ed25519PrivateKey::generate();
                        PublicKeyHash::hash(private_key.public_key())
                    };
                    let failing_transactions = (0..100)
                        .map(|_| {
                            TransactionIntent::sample_entity_addresses_with_pub_key_hashes_requiring_auth(
                                [(a0.address, make_random_pk_hash())],
                                [],
                            )
                        })
                        .collect::<Vec<_>>();
                    let mut all_transactions = failing_transactions.clone();
                    all_transactions.push(tx.clone());

                    let profile = Profile::sample_from(
                        factor_sources.clone(),
                        [a0],
                        [p3],
                    );

                    let collector = SignaturesCollector::new(
                        SigningFinishEarlyStrategy::default(),
                        all_transactions,
                        Arc::new(TestSignInteractor::new(
                            SimulatedUser::prudent_with_failures(
                                SimulatedFailures::with_simulated_failures([
                                    FactorSourceIDFromHash::sample_at(0),
                                ]),
                            ),
                        )),
                        &profile,
                        SigningPurpose::sign_transaction_primary(),
                    )
                    .unwrap();

                    let outcome = collector.collect_signatures().await.unwrap();
                    assert!(!outcome.successful());
                    assert_eq!(
                        outcome
                            .failed_transactions()
                            .iter()
                            .map(|t| t.signable_id.clone())
                            .collect_vec(),
                        failing_transactions
                            .iter()
                            .map(|t| t.transaction_intent_hash().clone())
                            .collect_vec()
                    );

                    assert_eq!(
                        outcome
                            .ids_of_neglected_factor_sources_failed()
                            .into_iter()
                            .collect_vec(),
                        vec![FactorSourceIDFromHash::sample_at(0)]
                    );

                    assert!(outcome
                        .ids_of_neglected_factor_sources_skipped_by_user()
                        .is_empty());

                    assert_eq!(
                        outcome
                            .successful_transactions()
                            .into_iter()
                            .map(|t| t.signable_id)
                            .collect_vec(),
                        vec![tx.transaction_intent_hash()]
                    )
                }

                #[actix_rt::test]
                async fn same_tx_is_not_shown_to_user_in_case_of_already_failure(
                ) {
                    let factor_sources = FactorSource::sample_all();

                    let a7 = Account::sample_at(7);
                    let a0 = Account::sample_at(0);

                    let tx0 = TransactionIntent::sample_entities_requiring_auth(
                        [&a7, &a0],
                        [],
                    );
                    let tx1 = TransactionIntent::sample_entities_requiring_auth(
                        [&a0],
                        [],
                    );

                    let profile = Profile::sample_from(
                        factor_sources.clone(),
                        [&a7, &a0],
                        [],
                    );

                    type Tuple = (
                        FactorSourceKind,
                        IndexSet<
                            InvalidTransactionIfNeglected<
                                TransactionIntentHash,
                            >,
                        >,
                    );
                    type Tuples = Vec<Tuple>;
                    let tuples = Rc::<RefCell<Tuples>>::new(RefCell::new(
                        Tuples::default(),
                    ));
                    let tuples_clone = tuples.clone();
                    let collector = SignaturesCollector::new(
                        SigningFinishEarlyStrategy::default(),
                        [tx0.clone(), tx1.clone()],
                        Arc::new(TestSignInteractor::new(
                            SimulatedUser::with_spy(
                                move |kind, invalid| {
                                    let tuple = (kind, invalid);
                                    let mut x =
                                        RefCell::borrow_mut(&tuples_clone);
                                    x.push(tuple)
                                },
                                SimulatedUserMode::Prudent,
                                SimulatedFailures::with_simulated_failures([
                                    FactorSourceIDFromHash::sample_at(2), // will cause any TX with a7 to fail
                                ]),
                            ),
                        )),
                        &profile,
                        SigningPurpose::sign_transaction_primary(),
                    )
                    .unwrap();

                    let outcome = collector.collect_signatures().await.unwrap();

                    let tuples = tuples.borrow().clone();
                    assert_eq!(
                        tuples,
                        vec![
                            (
                                FactorSourceKind::LedgerHQHardwareWallet,
                                IndexSet::just(
                                    InvalidTransactionIfNeglected::new(
                                        tx0.clone().transaction_intent_hash(),
                                        [a7.address.into()]
                                    )
                                )
                            ),
                            // Important that we do NOT display any mentioning of `tx0` here again!
                            (
                                FactorSourceKind::Device,
                                IndexSet::just(
                                    InvalidTransactionIfNeglected::new(
                                        tx1.clone().transaction_intent_hash(),
                                        [a0.address.into()]
                                    )
                                )
                            ),
                        ]
                    );

                    assert!(!outcome.successful());
                    assert_eq!(
                        outcome.ids_of_neglected_factor_sources_failed(),
                        IndexSet::<FactorSourceIDFromHash>::just(
                            FactorSourceIDFromHash::sample_at(2)
                        )
                    );
                    assert_eq!(
                        outcome.ids_of_neglected_factor_sources_irrelevant(),
                        IndexSet::<FactorSourceIDFromHash>::from_iter([
                            FactorSourceIDFromHash::sample_at(6),
                            FactorSourceIDFromHash::sample_at(7),
                            FactorSourceIDFromHash::sample_at(8),
                            FactorSourceIDFromHash::sample_at(9)
                        ])
                    );
                    assert_eq!(
                        outcome
                            .successful_transactions()
                            .into_iter()
                            .map(|t| t.signable_id)
                            .collect_vec(),
                        vec![tx1.transaction_intent_hash().clone()]
                    );

                    assert_eq!(
                        outcome
                            .failed_transactions()
                            .into_iter()
                            .map(|t| t.signable_id)
                            .collect_vec(),
                        vec![tx0.transaction_intent_hash().clone()]
                    );

                    assert_eq!(outcome.all_signatures().len(), 1);

                    assert!(outcome
                        .all_signatures()
                        .into_iter()
                        .map(|s| s.payload_id().clone())
                        .all(|i| i == tx1.transaction_intent_hash()));

                    assert_eq!(
                        outcome
                            .all_signatures()
                            .into_iter()
                            .map(|s| s.derivation_path())
                            .collect_vec(),
                        vec![DerivationPath::from(AccountPath::new(
                            NetworkID::Mainnet,
                            CAP26KeyKind::TransactionSigning,
                            Hardened::from_local_key_space(
                                U31::ZERO,
                                IsSecurified(false)
                            ) // unsecurified account at `0`.
                            .unwrap()
                        ))]
                    )
                }
            }

            mod no_fail {
                use super::*;

                #[actix_rt::test]
                async fn multi_accounts_multi_personas_all_single_factor_controlled(
                ) {
                    multi_accounts_multi_personas_all_single_factor_controlled_with_sim_user(
                        SimulatedUser::prudent_no_fail(),
                    )
                        .await;

                    // Same result with lazy user, not able to skip without failures.
                    multi_accounts_multi_personas_all_single_factor_controlled_with_sim_user(
                        SimulatedUser::lazy_sign_minimum([]),
                    )
                        .await
                }

                #[actix_rt::test]
                async fn multi_securified_entities() {
                    multi_securified_entities_with_sim_user(Vector {
                        simulated_user: SimulatedUser::prudent_no_fail(),
                        expected: Expected {
                            successful_txs_signature_count: 32,
                            // We always end early
                            // `Device` FactorSourceKind never got used since it
                            // we are done after YubiKey.
                            signed_factor_source_kinds: IndexSet::<
                                FactorSourceKind,
                            >::from_iter(
                                [
                                FactorSourceKind::LedgerHQHardwareWallet,
                                FactorSourceKind::ArculusCard,
                                FactorSourceKind::Password,
                            ]
                            ),
                            expected_neglected_factor_source_count: 0,
                        },
                    })
                    .await;

                    multi_securified_entities_with_sim_user(Vector {
                        simulated_user: SimulatedUser::lazy_sign_minimum([]),
                        expected: Expected {
                            successful_txs_signature_count: 24,
                            // We always end early, this lazy user was able to skip
                            // Ledger.
                            signed_factor_source_kinds: IndexSet::<
                                FactorSourceKind,
                            >::from_iter(
                                [
                                FactorSourceKind::ArculusCard,
                                FactorSourceKind::Password,
                                FactorSourceKind::Device,
                            ]
                            ),
                            expected_neglected_factor_source_count: 2,
                        },
                    })
                    .await;
                }
            }
        }

        mod rola {
            use std::collections::HashSet;

            use super::*;

            #[actix_rt::test]
            async fn test_petitions_for() {
                let factor_sources = &FactorSource::sample_all();

                let a0 = &Account::sample_at(0);
                let a1 = &Account::sample_at(1);
                let a6 = &Account::sample_at(6);

                let p0 = &Persona::sample_at(0);
                let p1 = &Persona::sample_at(1);
                let p6 = &Persona::sample_at(6);

                let entities_to_sign = vec![
                    AddressOfAccountOrPersona::Account(a0.address),
                    AddressOfAccountOrPersona::Account(a1.address),
                    AddressOfAccountOrPersona::Account(a6.address),
                    AddressOfAccountOrPersona::Identity(p0.address),
                    AddressOfAccountOrPersona::Identity(p1.address),
                    AddressOfAccountOrPersona::Identity(p6.address),
                ];

                let auth_intent = AuthIntent::new_from_request(
                    DappToWalletInteractionAuthChallengeNonce::sample(),
                    DappToWalletInteractionMetadata::sample(),
                    entities_to_sign,
                )
                .unwrap();

                let profile = Profile::sample_from(
                    factor_sources.clone(),
                    [a0, a1, a6],
                    [p0, p1, p6],
                );

                let collector = SignaturesCollector::new(
                    SigningFinishEarlyStrategy::default(),
                    [auth_intent.clone()],
                    Arc::new(TestSignInteractor::new(
                        SimulatedUser::prudent_no_fail(),
                    )),
                    &profile,
                    SigningPurpose::ROLA,
                )
                .unwrap();

                let petitions = collector.petitions();

                assert_eq!(
                    petitions
                        .txid_to_petition
                        .read()
                        .expect("Petitions lock should not have been poisoned")
                        .len(),
                    1
                );

                assert_petition(
                    &petitions,
                    &auth_intent,
                    HashMap::from_iter([
                        (
                            a0.address.into(),
                            HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                        ),
                        (
                            a1.address.into(),
                            HashSet::just(FactorSourceIDFromHash::sample_at(1)),
                        ),
                        (
                            a6.address.into(),
                            HashSet::from_iter([
                                // Only device factor source is used for signing auth for securified entity
                                FactorSourceIDFromHash::sample_at(0),
                            ]),
                        ),
                        (
                            p0.address.into(),
                            HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                        ),
                        (
                            p1.address.into(),
                            HashSet::just(FactorSourceIDFromHash::sample_at(1)),
                        ),
                        (
                            p6.address.into(),
                            HashSet::from_iter([
                                // Only device factor source is used for signing auth for securified entity
                                FactorSourceIDFromHash::sample_at(0),
                            ]),
                        ),
                    ]),
                    HashMap::from_iter([]),
                );
            }
        }
    }
}
