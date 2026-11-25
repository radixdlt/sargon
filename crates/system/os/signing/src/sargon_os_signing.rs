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

    /// Signs `subintent`. `execution_summary` is the one its review showed,
    /// if it was previewed: for the access controllers it calls, it decides
    /// which roles, and so which of the wallet's keys, the signatures are
    /// for. Calls on access controllers are signed only as the shared
    /// control it classifies, checked again here; without it, only the
    /// `create_proof`s the wallet adds, for their primary role.
    async fn sign_subintent(
        &self,
        subintent: Subintent,
        role_kind: RoleKind,
        execution_summary: Option<ExecutionSummary>,
    ) -> Result<SignedSubintent>;
}

/// How a transaction is signed, by its class.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SigningFlow {
    /// Updating a shield. The wallet reviews one role combination, a timed
    /// recovery, but signs whichever combination the user's factors can
    /// complete, which may confirm it at once.
    AccessControllerRecovery(AccessControllerAddress),
    /// Stopping a timed recovery, signed with whichever role is available,
    /// cancelling the proposal too when it's the recovery role.
    StopTimedRecovery(AccessControllerAddress),
    /// Anything else, securifying included: signed with the primary role,
    /// exactly as reviewed.
    AsReviewed,
}

impl SigningFlow {
    fn of(class: Option<&DetailedManifestClass>) -> Self {
        match class {
            Some(
                DetailedManifestClass::AccessControllerRecovery {
                    ac_addresses,
                    ..
                }
                | DetailedManifestClass::AccessControllerTimedRecovery {
                    ac_addresses,
                    ..
                },
            ) => ac_addresses.first().map_or(Self::AsReviewed, |ac| {
                Self::AccessControllerRecovery(*ac)
            }),
            // Only a stop, and cancelling the recovery role's proposal, which
            // the stop flow rebuilds: anything else would be dropped.
            Some(DetailedManifestClass::AccessControllerCancelRecovery {
                ac_addresses,
                cancellations,
            }) if cancellations.iter().all(|cancellation| {
                cancellation.stopped_timed_recovery.is_some()
                    && !cancellation.unlocks_primary_role
                    && cancellation
                        .cancelled_proposers
                        .iter()
                        .all(|proposer| *proposer == RecoveryProposer::Recovery)
            }) =>
            {
                ac_addresses
                    .first()
                    .map_or(Self::AsReviewed, |ac| Self::StopTimedRecovery(*ac))
            }
            _ => Self::AsReviewed,
        }
    }
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
        match SigningFlow::of(
            execution_summary.detailed_classification.as_ref(),
        ) {
            SigningFlow::AccessControllerRecovery(access_controller) => {
                sign_access_controller_recovery_transaction(
                    self,
                    transaction_intent,
                    access_controller,
                )
                .await
            }
            SigningFlow::StopTimedRecovery(access_controller) => {
                sign_access_controller_stop_timed_recovery_transaction(
                    self,
                    transaction_intent,
                    access_controller,
                )
                .await
            }
            SigningFlow::AsReviewed => {
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
        execution_summary: Option<ExecutionSummary>,
    ) -> Result<SignedSubintent> {
        info!(
            "External signing: begin for subintent {:?} (role: {:?})",
            subintent.get_id(),
            role_kind
        );
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

        let mut signatures = outcome.signatures_of_successful_transactions();

        let classification = execution_summary
            .as_ref()
            .and_then(|summary| summary.detailed_classification.as_ref());
        let external_accounts = resolve_external_accounts_for_subintent(
            self,
            &subintent,
            classification,
        )
        .await?;
        info!(
            "External signing: external accounts resolved: {}",
            external_accounts.len()
        );
        // Shared control whose calls need a role: the user must sign for it,
        // rather than hand back a pre-authorization without their signature.
        let needs_a_role = classification
            .filter(|class| is_allowed_in_pre_authorizations(class))
            .and_then(pre_authorization_access_controller_roles)
            .is_some_and(|roles| roles.values().any(|roles| !roles.is_empty()));
        if needs_a_role {
            let mfa_badges = self
                .mfa_signing_keys()?
                .into_iter()
                .map(|key| key.badge)
                .collect::<std::collections::HashSet<_>>();
            let has_a_key = external_accounts
                .iter()
                .flat_map(|account| account.required_nft_ids.iter())
                .any(|badge| mfa_badges.contains(badge));
            if !has_a_key {
                return Err(CommonError::NoKeysForAccessControllerRoles);
            }
        }
        let external_signatures = collect_external_signatures(
            subintent.clone(),
            external_accounts,
            self,
            self.sign_subintents_interactor(),
        )
        .await?;
        info!(
            "External signing: collected {} external signatures",
            external_signatures.len()
        );
        // The user had a key, but no signature came back, e.g. the factor
        // was skipped.
        if needs_a_role && external_signatures.is_empty() {
            return Err(
                CommonError::SigningFailedTooManyFactorSourcesNeglected,
            );
        }
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

#[cfg(test)]
mod test {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[test]
    fn shield_updates_go_through_the_recovery_flow() {
        let ac = AccessControllerAddress::sample();
        for class in [
            DetailedManifestClass::AccessControllerRecovery {
                ac_addresses: vec![ac],
                recovery_proposals: vec![],
            },
            // The reviewed shield update is a timed recovery.
            DetailedManifestClass::AccessControllerTimedRecovery {
                ac_addresses: vec![ac],
                timed_recoveries: vec![],
            },
        ] {
            assert_eq!(
                SigningFlow::of(Some(&class)),
                SigningFlow::AccessControllerRecovery(ac)
            );
        }
    }

    #[test]
    fn stopping_a_timed_recovery_goes_through_the_stop_flow() {
        let ac = AccessControllerAddress::sample();
        let stop = AccessControllerRecoveryCancellation {
            stopped_timed_recovery: Some(
                AccessControllerRuleSetForDisplay::sample(),
            ),
            cancelled_proposers: vec![RecoveryProposer::Recovery],
            unlocks_primary_role: false,
            ..AccessControllerRecoveryCancellation::sample()
        };
        let class = |cancellation| {
            DetailedManifestClass::AccessControllerCancelRecovery {
                ac_addresses: vec![ac],
                cancellations: vec![cancellation],
            }
        };

        assert_eq!(
            SigningFlow::of(Some(&class(stop.clone()))),
            SigningFlow::StopTimedRecovery(ac)
        );
        // The stop flow rebuilds only a stop and cancelling the recovery
        // role's proposal: anything else is signed as reviewed, not dropped.
        for cancellation in [
            AccessControllerRecoveryCancellation {
                stopped_timed_recovery: None,
                ..stop.clone()
            },
            AccessControllerRecoveryCancellation {
                unlocks_primary_role: true,
                ..stop.clone()
            },
            AccessControllerRecoveryCancellation {
                cancelled_proposers: vec![RecoveryProposer::Primary],
                ..stop.clone()
            },
        ] {
            assert_eq!(
                SigningFlow::of(Some(&class(cancellation))),
                SigningFlow::AsReviewed
            );
        }
    }

    /// Securifying is signed exactly as reviewed: nothing is added to it.
    #[test]
    fn anything_else_is_signed_as_reviewed() {
        for class in [
            Some(DetailedManifestClass::SecurifyEntity {
                entities: vec![AccountAddress::sample().into()],
            }),
            Some(DetailedManifestClass::General),
            Some(DetailedManifestClass::AccessControllerConfirmRecovery {
                ac_addresses: vec![AccessControllerAddress::sample()],
                confirmations: vec![],
            }),
            None,
        ] {
            assert_eq!(
                SigningFlow::of(class.as_ref()),
                SigningFlow::AsReviewed
            );
        }
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
            .sign_subintent(signable.clone(), RoleKind::Primary, None)
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
            .sign_subintent(signable.clone(), RoleKind::Primary, None)
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
            .sign_subintent(signable.clone(), RoleKind::Primary, None)
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
