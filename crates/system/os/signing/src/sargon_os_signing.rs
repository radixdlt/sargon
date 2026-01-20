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
            Some(DetailedManifestClass::SecurifyEntity { entities }) => {
                let entity_address = entities.first().unwrap();
                return sign_entity_securify(
                    self,
                    transaction_intent,
                    *entity_address,
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
        let profile = &self.profile()?;

        let collector = SignaturesCollector::new(
            SigningFinishEarlyStrategy::default(),
            vec![subintent.clone()],
            self.sign_subintents_interactor(),
            profile,
            SigningPurpose::sign_transaction(role_kind),
        )?;

        let outcome = collector.collect_signatures().await?;
        if !outcome.successful() {
            return Err(
                CommonError::SigningFailedTooManyFactorSourcesNeglected,
            );
        }

        let mut signatures =
            outcome.signatures_of_successful_transactions();

        let external_accounts =
            self.resolve_external_accounts_for_subintent(&subintent).await?;
        let external_signatures = collect_external_signatures(
            subintent.clone(),
            external_accounts,
            self,
            self.sign_subintents_interactor(),
        )
        .await?;
        signatures.extend(external_signatures);

        subintent.signed(signatures)
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

#[async_trait::async_trait]
impl FactorInstanceLookupByNftIds for SargonOS {
    async fn factor_instances_for_nfts(
        &self,
        _nft_ids: Vec<NonFungibleGlobalId>,
    ) -> Result<Vec<HierarchicalDeterministicFactorInstance>> {
        // TODO: wire the real lookup for NFT -> factor instance mapping.
        Ok(Vec::new())
    }
}

impl SargonOS {
    async fn resolve_external_accounts_for_subintent(
        &self,
        subintent: &Subintent,
    ) -> Result<Vec<ExternalAccountAccessRule>> {
        let summary = subintent.manifest.summary()?;
        let profile = self.profile()?;

        let external_account_addresses = summary
            .addresses_of_accounts_requiring_auth
            .into_iter()
            .filter(|address| profile.account_by_address(*address).is_err())
            .collect_vec();

        if external_account_addresses.is_empty() {
            return Ok(Vec::new());
        }

        let (gateway_client, network_id) = self.gateway_client_on()?;
        let badge_owners = gateway_client
            .fetch_entities_badge_owners(
                network_id,
                external_account_addresses
                    .iter()
                    .map(|address| {
                        AddressOfAccountOrPersona::Account(*address)
                    })
                    .collect_vec(),
            )
            .await?;

        let mut external_accounts = Vec::new();
        for address in external_account_addresses {
            let owner_address =
                AddressOfAccountOrPersona::Account(address);
            let maybe_badge_owner =
                badge_owners.get(&owner_address).unwrap_or(&None);
            let Some(access_controller_address) =
                maybe_badge_owner.and_then(|a| a.as_access_controller().cloned())
            else {
                continue;
            };

            let access_rule = self
                .access_rule_for_access_controller(access_controller_address)
                .await?;
            external_accounts.push(ExternalAccountAccessRule {
                owner: owner_address,
                access_rule,
            });
        }

        Ok(external_accounts)
    }
}

impl SargonOS {
    async fn access_rule_for_access_controller(
        &self,
        access_controller_address: AccessControllerAddress,
    ) -> Result<gateway_models::prelude::AccessRule> {
        let gateway_client = self.gateway_client()?;
        let details = gateway_client
            .fetch_access_controller_details(access_controller_address)
            .await?;

        if let Some(attempt) = details.state.primary_role_recovery_attempt {
            return Ok(attempt.recovery_proposal.primary_role);
        }
        if let Some(attempt) = details.state.recovery_role_recovery_attempt {
            return Ok(attempt.recovery_proposal.primary_role);
        }

        Err(CommonError::Unknown {
            error_message: "Access controller access rule unavailable; TODO: fetch configured rule set from access controller or account entity.".to_string(),
        })
    }
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
                MFAFactorInstances::new(),
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
