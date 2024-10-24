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
                to,
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
                    FungibleResourceIndicator::guaranteed(amount)
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
        let address = AccountAddress::new(public_key, network_id);
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
        let intent_signature = private_key.sign_intent_hash(&intent_hash);

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

    /// Test failing due to Stokenet being down some time
    // #[actix_rt::test]
    // async fn test_dapp_metadata() {
    //     let gumball_address = AccountAddress::try_from_bech32(
    //         "account_tdx_2_129nx5lgkk3fz9gqf3clppeljkezeyyymqqejzp97tpk0r8els7hg3j",
    //     )
    //         .unwrap();
    //     let gateway_client = new_gateway_client(NetworkID::Stokenet);
    //     let sut = gateway_client.fetch_dapp_metadata(gumball_address);

    //     let response = timeout(MAX, sut).await.unwrap().unwrap();
    //     let icon_url = response.get_icon_url();
    //     assert_eq!(
    //         icon_url,
    //         Some(
    //             Url::parse(
    //                 "https://stokenet-gumball-club.radixdlt.com/assets/gumball-club.png"
    //             )
    //                 .unwrap()
    //         )
    //     );
    // }

    // #[actix_rt::test]
    // async fn get_transaction_status() {
    //     let network_id = NetworkID::Stokenet;
    //     let gateway_client = new_gateway_client(network_id);
    //     let private_key = Ed25519PrivateKey::generate();
    //     let (_, tx_id) =
    //         submit_tx_use_faucet(private_key, network_id).await.unwrap();

    //     let status_response =
    //         timeout(MAX, gateway_client.get_transaction_status(tx_id))
    //             .await
    //             .unwrap()
    //             .unwrap();

    //     assert_eq!(status_response.error_message, None);
    //     let status = status_response
    //         .known_payloads
    //         .first()
    //         .and_then(|payload| payload.payload_status.clone())
    //         .unwrap();
    //     assert_eq!(status, TransactionStatusResponsePayloadStatus::Pending);
    // }

    mod signing {
        use super::*;
        use radix_common::prelude::indexmap::IndexSet;
        use std::sync::Arc;

        struct TestLazySignMinimumInteractors;
        struct TestLazySignMinimumInteractor;

        #[async_trait::async_trait]
        impl PolyFactorSignInteractor for TestLazySignMinimumInteractor {
            async fn sign(
                &self,
                request: PolyFactorSignRequest,
            ) -> SignWithFactorsOutcome {
                let mut signatures = IndexSet::<HDSignature>::new();
                for (_, req) in request.per_factor_source.iter() {
                    let resp = <Self as MonoFactorSignInteractor>::sign(
                        self,
                        MonoFactorSignRequest::new(
                            req.clone(),
                            request.invalid_transactions_if_neglected.clone(),
                        ),
                    )
                    .await;

                    match resp {
                        SignWithFactorsOutcome::Signed {
                            produced_signatures,
                        } => {
                            signatures.extend(
                                produced_signatures
                                    .signatures
                                    .into_iter()
                                    .flat_map(|(_, xs)| xs)
                                    .collect::<IndexSet<_>>(),
                            );
                        }
                        SignWithFactorsOutcome::Neglected(_) => {
                            return SignWithFactorsOutcome::Neglected(
                                NeglectedFactors::new(
                                    NeglectFactorReason::UserExplicitlySkipped,
                                    request.factor_source_ids(),
                                ),
                            );
                        }
                    }
                }
                SignWithFactorsOutcome::signed(SignResponse::with_signatures(
                    signatures,
                ))
            }
        }

        #[async_trait::async_trait]
        impl MonoFactorSignInteractor for TestLazySignMinimumInteractor {
            async fn sign(
                &self,
                request: MonoFactorSignRequest,
            ) -> SignWithFactorsOutcome {
                if request.invalid_transactions_if_neglected.is_empty() {
                    return SignWithFactorsOutcome::Neglected(
                        NeglectedFactors::new(
                            NeglectFactorReason::UserExplicitlySkipped,
                            IndexSet::just(request.input.factor_source_id),
                        ),
                    );
                }
                let signatures = request
                    .input
                    .per_transaction
                    .into_iter()
                    .flat_map(|r| {
                        r.signature_inputs()
                            .iter()
                            .map(|x| HDSignature::fake_sign_by_looking_up_mnemonic_amongst_samples(x.clone()))
                            .collect::<IndexSet<_>>()
                    })
                    .collect::<IndexSet<HDSignature>>();
                SignWithFactorsOutcome::Signed {
                    produced_signatures: SignResponse::with_signatures(
                        signatures,
                    ),
                }
            }
        }

        impl SignInteractors for TestLazySignMinimumInteractors {
            fn interactor_for(&self, kind: FactorSourceKind) -> SignInteractor {
                match kind {
                    FactorSourceKind::Device => SignInteractor::mono(Arc::new(
                        TestLazySignMinimumInteractor,
                    )),
                    _ => SignInteractor::poly(Arc::new(
                        TestLazySignMinimumInteractor,
                    )),
                }
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
                AccountAddress::sample_at(0),
                || {
                    let i = HDPathComponent::from(0);
                    GeneralRoleWithHierarchicalDeterministicFactorInstances::threshold_only(
                        [
                            FI::sample_mainnet_tx_account(i, *f0.factor_source_id().as_hash().unwrap()), // SKIPPED
                            FI::sample_mainnet_tx_account(i, *f1.factor_source_id().as_hash().unwrap()),
                            FI::sample_mainnet_tx_account(i, *f2.factor_source_id().as_hash().unwrap()),
                        ],
                        2,
                    ).unwrap()
                },
            );

            let bob = Account::sample_securified_mainnet(
                "Bob",
                AccountAddress::sample_at(1),
                || {
                    let i = HDPathComponent::from(1);
                    GeneralRoleWithHierarchicalDeterministicFactorInstances::override_only([
                        FI::sample_mainnet_tx_account(
                            i,
                            *f3.factor_source_id().as_hash().unwrap(),
                        )
                    ])
                },
            );

            let carol = Account::sample_securified_mainnet(
                "Carol",
                AccountAddress::sample_at(2),
                || {
                    let i = HDPathComponent::from(2);
                    GeneralRoleWithHierarchicalDeterministicFactorInstances::new(
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
                    HDPathComponent::from(0),
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
                Arc::new(TestLazySignMinimumInteractors),
                &profile,
                RoleKind::Primary,
            )
            .unwrap();

            let outcome = collector.collect_signatures().await;

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
    }
}
