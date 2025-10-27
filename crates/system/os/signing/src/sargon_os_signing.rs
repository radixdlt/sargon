use crate::prelude::*;

#[async_trait::async_trait]
pub trait OsSigning {
    async fn sign<S: Signable>(
        &self,
        signable: S,
        sign_interactor: Arc<dyn SignInteractor<S>>,
        purpose: SigningPurpose,
    ) -> Result<S::Signed>;

    async fn sign_auth(
        &self,
        auth_intent: AuthIntent,
    ) -> Result<SignedAuthIntent>;

    async fn sign_transaction(
        &self,
        transaction_intent: TransactionIntent,
        execution_summary: ExecutionSummary,
    ) -> Result<SignedIntent>;

    async fn sign_subintent(
        &self,
        subintent: Subintent,
        role_kind: RoleKind,
    ) -> Result<SignedSubintent>;
}

// ==================
// Sign Signables
// ==================
#[async_trait::async_trait]
impl OsSigning for SargonOS {
    async fn sign_auth(
        &self,
        auth_intent: AuthIntent,
    ) -> Result<SignedAuthIntent> {
        self.sign(
            auth_intent.clone(),
            self.sign_auth_interactor(),
            SigningPurpose::ROLA,
        )
        .await
    }

    async fn sign_transaction(
        &self,
        transaction_intent: TransactionIntent,
        execution_summary: ExecutionSummary,
    ) -> Result<SignedIntent> {
        match execution_summary.detailed_classification {
            Some(DetailedManifestClass::AccessControllerRecovery {
                ac_addresses,
            }) => {
                return sign_access_controller_recovery_transaction(
                    self,
                    transaction_intent,
                    ac_addresses[0],
                )
                .await;
            }
            Some(
                DetailedManifestClass::AccessControllerStopTimedRecovery {
                    ac_addresses,
                },
            ) => {
                return sign_access_controller_stop_timed_recovery_transaction(
                    self,
                    transaction_intent,
                    ac_addresses[0],
                )
                .await;
            }
            _ => {
                self.sign(
                    transaction_intent.clone(),
                    self.sign_transactions_interactor(),
                    SigningPurpose::sign_transaction(RoleKind::Primary),
                )
                .await
            }
        }
    }

    async fn sign_subintent(
        &self,
        subintent: Subintent,
        role_kind: RoleKind,
    ) -> Result<SignedSubintent> {
        self.sign(
            subintent.clone(),
            self.sign_subintents_interactor(),
            SigningPurpose::sign_transaction(role_kind),
        )
        .await
    }

    async fn sign<S: Signable>(
        &self,
        signable: S,
        sign_interactor: Arc<dyn SignInteractor<S>>,
        purpose: SigningPurpose,
    ) -> Result<S::Signed> {
        let profile = &self.profile()?;

        let collector = SignaturesCollector::new(
            SigningFinishEarlyStrategy::default(),
            vec![signable.clone()],
            sign_interactor,
            profile,
            purpose,
        )?;

        let outcome = collector.collect_signatures().await?;

        if outcome.successful() {
            let signatures = outcome.signatures_of_successful_transactions();
            signable.signed(signatures)
        } else {
            Err(CommonError::SigningFailedTooManyFactorSourcesNeglected)
        }
    }
}

async fn sign_access_controller_recovery_transaction(
    os: &SargonOS,
    base_transaction_intent: TransactionIntent,
    ac_address: AccessControllerAddress,
) -> Result<SignedIntent> {
    let profile = os.profile()?;
    let gw_client = os.gateway_client()?;

    let (fee_paying_account_address, fee) = base_transaction_intent
        .extract_fee_payer_info()
        .expect("Shoud have a fee payer configured");

    let fee_payer_xrd_balance = gw_client
        .xrd_balance_of_account_or_zero(fee_paying_account_address)
        .await?;
    let fee_payer_account =
        profile.account_by_address(fee_paying_account_address)?;
    let lock_fee_data = LockFeeData::new_with_account(
        fee_payer_account,
        fee,
        fee_payer_xrd_balance,
    );

    let ac_state_details = gw_client
        .fetch_access_controller_details(ac_address)
        .await?;

    let factory = SignaturesCollectorFactory::new(
        base_transaction_intent,
        os.sign_transactions_interactor(),
        profile,
        lock_fee_data,
        ac_state_details,
    )?;

    SignaturesCollectorOrchestrator::new(factory).sign().await
}

async fn sign_access_controller_stop_timed_recovery_transaction(
    os: &SargonOS,
    base_transaction_intent: TransactionIntent,
    ac_address: AccessControllerAddress,
) -> Result<SignedIntent> {
    let profile = os.profile()?;
    let gw_client = os.gateway_client()?;

    let (fee_paying_account_address, fee) = base_transaction_intent
        .extract_fee_payer_info()
        .expect("Shoud have a fee payer configured");

    let fee_payer_xrd_balance = gw_client
        .xrd_balance_of_account_or_zero(fee_paying_account_address)
        .await?;
    let fee_payer_account =
        profile.account_by_address(fee_paying_account_address)?;
    let lock_fee_data = LockFeeData::new_with_account(
        fee_payer_account,
        fee,
        fee_payer_xrd_balance,
    );

    let ac_state_details = gw_client
        .fetch_access_controller_details(ac_address)
        .await?;

    let factory = StopTimedRecoverySignaturesCollectorFactory::new(
        base_transaction_intent,
        os.sign_transactions_interactor(),
        profile,
        lock_fee_data,
        ac_state_details,
    )?;

    StopTimedRecoverySignaturesCollectorOrchestrator::new(factory)
        .sign()
        .await
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    fn default_sign_transaction_args() -> (ExecutionSummary, LockFeeData) {
        (
            ExecutionSummary::sample(),
            LockFeeData::new_with_unsecurified_fee_payer(
                AccountAddress::sample(),
                Decimal192::one(),
            ),
        )
    }

    #[actix_rt::test]
    async fn test_sign_auth_success() {
        let profile = Profile::sample();
        let sut = boot(Some(profile.clone()), None).await;
        let all_accounts = profile.accounts_on_current_network().unwrap();
        let account = all_accounts.first().unwrap();
        let nonce = DappToWalletInteractionAuthChallengeNonce::sample();
        let metadata = DappToWalletInteractionMetadata::new(
            WalletInteractionVersion::current(),
            NetworkID::Mainnet,
            "https://example.com",
            DappDefinitionAddress::sample(),
        );
        let auth_intent = AuthIntent::new_from_request(
            nonce,
            metadata,
            [AddressOfAccountOrPersona::Account(account.address)],
        )
        .unwrap();

        let signed = sut.sign_auth(auth_intent.clone()).await.unwrap();

        let signature_with_public_key = signed
            .intent_signatures_per_owner
            .values()
            .collect_vec()
            .first()
            .unwrap()
            .0;

        assert!(signature_with_public_key
            .is_valid_for_hash(&auth_intent.auth_intent_hash().hash()))
    }

    #[actix_rt::test]
    async fn test_sign_auth_failure() {
        let profile = Profile::sample();

        let sut =
            boot(Some(profile.clone()), Some(SigningFailure::UserRejected))
                .await;

        let all_accounts = profile.accounts_on_current_network().unwrap();
        let account = all_accounts.first().unwrap();
        let nonce = DappToWalletInteractionAuthChallengeNonce::sample();
        let metadata = DappToWalletInteractionMetadata::new(
            WalletInteractionVersion::current(),
            NetworkID::Mainnet,
            "https://example.com",
            DappDefinitionAddress::sample(),
        );

        let auth_intent = AuthIntent::new_from_request(
            nonce,
            metadata,
            vec![AddressOfAccountOrPersona::Account(account.address)],
        )
        .unwrap();

        let result = sut.sign_auth(auth_intent).await;

        assert_eq!(result, Err(CommonError::HostInteractionAborted))
    }

    #[actix_rt::test]
    async fn test_sign_transaction_intent_success() {
        let profile = Profile::sample();
        let sut = boot(Some(profile.clone()), None).await;

        let (signable, entities) = get_signable_with_entities::<
            TransactionIntent,
        >(&sut.profile().unwrap());

        let signed = sut
            .sign_transaction(signable.clone(), ExecutionSummary::sample())
            .await
            .unwrap();

        assert_eq!(signable, signed.intent);
        assert_eq!(entities.len(), signed.intent_signatures.signatures.len());
    }

    #[actix_rt::test]
    async fn test_sign_subintent_success() {
        let profile = Profile::sample();
        let sut = boot(Some(profile.clone()), None).await;

        let (signable, entities) =
            get_signable_with_entities::<Subintent>(&sut.profile().unwrap());

        let signed = sut
            .sign_subintent(signable.clone(), RoleKind::Primary)
            .await
            .unwrap();

        assert_eq!(signable, signed.subintent);
        assert_eq!(
            entities.len(),
            signed.subintent_signatures.signatures.len()
        );
    }

    #[actix_rt::test]
    async fn test_sign_transaction_intent_only_with_irrelevant_entity() {
        let profile = Profile::sample();
        let sut = boot(Some(profile.clone()), None).await;

        let irrelevant_account = Account::sample_mainnet_third();
        let transaction = TransactionIntent::sample_entities_requiring_auth(
            vec![&irrelevant_account],
            vec![],
        );

        let outcome = sut
            .sign_transaction(transaction, ExecutionSummary::sample())
            .await
            .unwrap();

        assert_eq!(outcome.intent_signatures.signatures.len(), 0);
    }

    #[actix_rt::test]
    async fn test_sign_transaction_intent_containing_irrelevant_entity() {
        let profile = Profile::sample();
        let sut = boot(Some(profile.clone()), None).await;

        let irrelevant_account = Account::sample_mainnet_third();
        let relevant_account = Account::sample_mainnet();
        let transaction = TransactionIntent::sample_entities_requiring_auth(
            vec![&irrelevant_account, &relevant_account],
            vec![],
        );

        let outcome = sut
            .sign_transaction(transaction, ExecutionSummary::sample())
            .await
            .unwrap();

        assert_eq!(outcome.intent_signatures.signatures.len(), 1);
    }

    #[actix_rt::test]
    async fn test_sign_transaction_intent_rejected_due_to_all_factors_neglected(
    ) {
        let profile = Profile::sample();
        let sut = boot(
            Some(profile.clone()),
            Some(SigningFailure::FailingFactorSources(vec![
                profile.device_factor_sources().first().unwrap().id,
            ])),
        )
        .await;

        let (signable, _) = get_signable_with_entities::<TransactionIntent>(
            &sut.profile().unwrap(),
        );

        let outcome = sut
            .sign_transaction(signable.clone(), ExecutionSummary::sample())
            .await;

        assert_eq!(
            outcome,
            Err(CommonError::SigningFailedTooManyFactorSourcesNeglected)
        );
    }

    #[actix_rt::test]
    async fn test_sign_transaction_prudent_user_skips_factor() {
        let device = FactorSource::sample_at(0);
        let account_device = Account::sample_unsecurified_mainnet(
            "Device",
            HierarchicalDeterministicFactorInstance::new_for_entity(
                device.clone().as_device().unwrap().id,
                CAP26EntityKind::Account,
                Hardened::from_local_key_space_unsecurified(0u32).unwrap(),
            ),
        );
        let ledger1 = FactorSource::sample_at(1);
        let account_ledger1 = Account::sample_unsecurified_mainnet(
            "Ledger1",
            HierarchicalDeterministicFactorInstance::new_for_entity(
                ledger1.clone().as_ledger().unwrap().id,
                CAP26EntityKind::Account,
                Hardened::from_local_key_space_unsecurified(0u32).unwrap(),
            ),
        );
        let ledger2 = FactorSource::sample_at(2);
        let account_ledger2 = Account::sample_unsecurified_mainnet(
            "Ledger2",
            HierarchicalDeterministicFactorInstance::new_for_entity(
                ledger2.clone().as_ledger().unwrap().id,
                CAP26EntityKind::Account,
                Hardened::from_local_key_space_unsecurified(0u32).unwrap(),
            ),
        );

        let profile = Profile::with(
            Header::sample(),
            FactorSources::from_iter([
                device.clone(),
                ledger1.clone(),
                ledger2.clone(),
            ]),
            AppPreferences::default(),
            ProfileNetworks::just(ProfileNetwork::new(
                NetworkID::Mainnet,
                [
                    account_device.clone(),
                    account_ledger1.clone(),
                    account_ledger2.clone(),
                ],
                [],
                AuthorizedDapps::new(),
                ResourcePreferences::new(),
            )),
        );

        let sut = boot(
            Some(profile.clone()),
            Some(SigningFailure::SkippingFactorSources(vec![
                ledger1.as_ledger().unwrap().id,
            ])),
        )
        .await;

        let signable =
            TransactionIntent::sample_entity_addresses_requiring_auth(
                [
                    account_device.address,
                    account_ledger1.address,
                    account_ledger2.address,
                ],
                [],
            );

        let outcome = sut
            .sign_transaction(signable.clone(), ExecutionSummary::sample())
            .await;

        assert_eq!(
            outcome,
            Err(CommonError::SigningFailedTooManyFactorSourcesNeglected)
        );
    }

    #[actix_rt::test]
    async fn test_sign_transaction_subintent_rejected_due_to_all_factors_neglected(
    ) {
        let profile = Profile::sample();
        let sut = boot(
            Some(profile.clone()),
            Some(SigningFailure::FailingFactorSources(vec![
                profile.device_factor_sources().first().unwrap().id,
            ])),
        )
        .await;

        let (signable, _) =
            get_signable_with_entities::<Subintent>(&sut.profile().unwrap());

        let outcome = sut
            .sign_subintent(signable.clone(), RoleKind::Primary)
            .await;

        assert_eq!(
            outcome,
            Err(CommonError::SigningFailedTooManyFactorSourcesNeglected)
        );
    }

    #[actix_rt::test]
    async fn test_sign_fail_due_to_profile() {
        let test_drivers = Drivers::test();
        let mut clients = Clients::new(Bios::new(test_drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();
        let interactors = Interactors::new_from_clients(&clients);
        let sut =
            SUT::boot_with_clients_and_interactor(clients, interactors).await;

        let transaction =
            TransactionIntent::sample_entity_addresses_requiring_auth(
                vec![AccountAddress::sample_mainnet()],
                vec![],
            );

        let (execution_summary, lock_fee_data) =
            default_sign_transaction_args();

        let outcome = sut
            .sign_transaction(transaction, ExecutionSummary::sample())
            .await;

        assert_eq!(
            outcome,
            Err(CommonError::ProfileStateNotLoaded {
                current_state: ProfileState::None.to_string()
            })
        );
    }

    #[actix_rt::test]
    async fn test_sign_fail_due_to_user_rejecting() {
        let profile = Profile::sample();
        let sut =
            boot(Some(profile.clone()), Some(SigningFailure::UserRejected))
                .await;

        let (signable, _) =
            get_signable_with_entities::<Subintent>(&sut.profile().unwrap());

        let outcome = sut
            .sign_subintent(signable.clone(), RoleKind::Primary)
            .await;

        assert_eq!(outcome, Err(CommonError::HostInteractionAborted));
    }

    async fn boot(
        profile: Option<Profile>,
        maybe_signing_failure: Option<SigningFailure>,
    ) -> Arc<SUT> {
        let secure_storage_driver = EphemeralSecureStorage::new();

        if let Some(profile) = profile {
            let secure_storage_client =
                SecureStorageClient::new(secure_storage_driver.clone());
            secure_storage_client.save_profile(&profile).await.unwrap();
        }

        let test_drivers = Drivers::with_secure_storage(secure_storage_driver);
        let mut clients = Clients::new(Bios::new(test_drivers));
        clients.factor_instances_cache =
            FactorInstancesCacheClient::in_memory();

        let use_factor_sources_interactors =
            Arc::new(TestUseFactorSourcesInteractors::new(
                Arc::new(TestSignInteractor::<TransactionIntent>::new(
                    get_simulated_user::<TransactionIntent>(
                        &maybe_signing_failure,
                    ),
                )),
                Arc::new(TestSignInteractor::<Subintent>::new(
                    get_simulated_user::<Subintent>(&maybe_signing_failure),
                )),
                Arc::new(TestDerivationInteractor::new(
                    false,
                    Arc::new(clients.secure_storage.clone()),
                )),
                Arc::new(TestSignInteractor::<AuthIntent>::new(
                    get_simulated_user::<AuthIntent>(&maybe_signing_failure),
                )),
            ));
        let interactors = Interactors::new(
            use_factor_sources_interactors,
            Arc::new(TestAuthorizationInteractor::stubborn_authorizing()),
            Arc::new(TestSpotCheckInteractor::new_succeeded()),
        );
        SUT::boot_with_clients_and_interactor(clients, interactors).await
    }

    fn get_simulated_user<S: Signable>(
        maybe_signing_failure: &Option<SigningFailure>,
    ) -> SimulatedUser<S> {
        match maybe_signing_failure {
            None => SimulatedUser::<S>::prudent_no_fail(),
            Some(failure) => match failure {
                SigningFailure::FailingFactorSources(factor_sources) => {
                    SimulatedUser::<S>::prudent_with_failures(
                        SimulatedFailures::with_simulated_failures(
                            factor_sources.clone(),
                        ),
                    )
                }
                SigningFailure::UserRejected => SimulatedUser::<S>::rejecting(),
                SigningFailure::SkippingFactorSources(factor_sources) => {
                    SimulatedUser::<S>::skipping_specific(
                        factor_sources.iter().cloned().collect(),
                    )
                }
            },
        }
    }

    fn get_signable_with_entities<
        S: Signable + ProvidesSamplesByBuildingManifest,
    >(
        profile: &Profile,
    ) -> (S, Vec<impl IsEntityAddress>) {
        let accounts_addresses_involved = profile
            .accounts_on_current_network()
            .unwrap()
            .iter()
            .map(|a| a.address)
            .collect_vec();

        (
            S::sample_entity_addresses_requiring_auth(
                accounts_addresses_involved.clone(),
                [],
            ),
            accounts_addresses_involved,
        )
    }

    enum SigningFailure {
        FailingFactorSources(Vec<FactorSourceIDFromHash>),
        SkippingFactorSources(Vec<FactorSourceIDFromHash>),
        UserRejected,
    }
}
