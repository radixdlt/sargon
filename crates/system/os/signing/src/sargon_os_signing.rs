use manifests::{
    RolesExercisableInTransactionManifestCombination,
    TransactionManifestSecurifySecurifiedEntity,
};

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
    ) -> Result<SignedSubintent>;

    async fn sign_ac_recovery_transaction(
        &self,
        transaction_intent: TransactionIntent,
        access_controller: AccessControllerAddress,
    ) -> Result<SignedIntent>;
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
                ac_addresses: ac_addresses,
            }) => {
                let access_controller = ac_addresses.first().unwrap();
                self.sign_ac_recovery_transaction(
                    transaction_intent,
                    access_controller.clone(),
                )
                .await
            }
            Some(
                DetailedManifestClass::AccessControllerStopTimedRecovery {
                    ac_addresses: _,
                },
            ) => {
                // MFA - actually based on AC specs, the tx needs to be signed by the factor that proposed the recovery
                self.sign(
                    transaction_intent.clone(),
                    self.sign_transactions_interactor(),
                    SigningPurpose::sign_transaction(RoleKind::Primary),
                )
                .await
            }
            Some(_) | None => {
                // Transactions exercising Primary role
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
    ) -> Result<SignedSubintent> {
        self.sign(
            subintent.clone(),
            self.sign_subintents_interactor(),
            SigningPurpose::sign_transaction(RoleKind::Primary),
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

    async fn sign_ac_recovery_transaction(
        &self,
        recovery_plus_confirmation_transaction_intent: TransactionIntent,
        access_controller: AccessControllerAddress,
    ) -> Result<SignedIntent> {
        // Preparation:
        // - Extract the entity from ac address.
        // - Extract entity's secured control.
        // - Extract the proposed security structure for recovery.
        // - Prepare the transaction intents candidates.
        let entity =
            self.entity_by_access_controller_address(access_controller)?;
        let security_control = entity.security_state().into_securified().unwrap();
        let proposed_security_structure = security_control
            .provisional_securified_config
            .ok_or(CommonError::EntityHasNoProvisionalSecurityConfigSet)?;
        let controlling_security_structure =
            security_control.security_structure;
        let profile = &self.profile()?;
        let signing_factor_sources: SecurityStructureOfFactorSources = controlling_security_structure.try_into()?;

        // Need to prepare the intents to sign with R role, we do already have the R+C intent that was originally build.
        let securified_entity =
            AnySecurifiedEntity::with_securified_entity_control(
                entity.clone(),
                security_control,
            );

        let recovery_intents = AccessControllerRecoveryIntents::new(
            recovery_plus_confirmation_transaction_intent.clone(), 
            securified_entity.clone(),
            proposed_security_structure.get_security_structure_of_factor_instances().clone(),
        )?;
       
       let mut signatures: IndexSet<HDSignature<TransactionIntentHash>> = IndexSet::new();

       // 1. Try signing with recovery role
       let collector = SignaturesCollector::with(
        SigningFinishEarlyStrategy::default(),
         profile.factor_sources(),
         recovery_intents.iniate_with_recovery_role_signable_intents(&profile),
          self.sign_transactions_interactor(),
          SigningPurpose::sign_transaction(RoleKind::Recovery)
        );

        // The intent that 
        let mut per_intent_signatures: IndexMap<TransactionIntentHash, IndexSet<HDSignature<TransactionIntentHash>>> = IndexMap::new();

        let outcome = collector.collect_signatures().await?;

        if outcome.successful() {
            // successfully signed with recovery
            let produced_signatures = outcome.signatures_of_successful_transactions();
            for signature in produced_signatures {
                per_intent_signatures.append_or_insert_element_to(signature.input.payload_id, signature);
            }

            // Sign with confirmation
            let collector = SignaturesCollector::with(
                SigningFinishEarlyStrategy::default(),
                 profile.factor_sources(),
                 recovery_intents.iniate_with_recovery_role_signable_intents(&profile),
                  self.sign_transactions_interactor(),
                  SigningPurpose::sign_transaction(RoleKind::Confirmation)
                );
            let outcome = collector.collect_signatures().await?;

            if outcome.successful() {
                let produced_signatures = outcome.signatures_of_successful_transactions();
                for signature in produced_signatures {
                    per_intent_signatures.append_or_insert_element_to(signature.input.payload_id, signature);
                }
            } else {

            }

        } else {
           // iniate with primary
        }

        signable.signed(signatures);

        Err(CommonError::Unknown)
    }
}

enum RecoverySignatureFSM {
    SignInitiateWithRecoveryIntents,
    SignInitiateWithRecoveryConfirmWithConfirmationIntent,
    SignInitiateWithRecoveryConfirmWithPrimaryIntent,
    SignInitiateWithPrimaryConfirmWithConfirmationIntent
}

struct ACRecoverySignaturesCollector {
    factory: ACRecoverySignatureCollectorFactory
}

impl ACRecoverySignaturesCollector {
    async fn sign(&self) -> Result<SignedIntent> {
        let sign_with_recovery_outcome = self.factory.iniate_with_recovery_sign_with_recovery().collect_signatures().await?;
        if sign_with_recovery_outcome.successful() {
            let recovery_signatures = sign_with_recovery_outcome.all_signatures();

            let sign_with_confirmation = self.factory.iniate_with_recovery_sign_with_confirmation().collect_signatures().await?;
            if sign_with_confirmation.successful() {
                let intent = self.factory.ac_recovery_intents.initiate_with_recovery_complete_with_confirmation.signable.clone();
                let mut confirmation_signatures = sign_with_confirmation.all_signatures();
                let r_plus_c_signature = recovery_signatures.iter().find(|s| s.payload_id() == &intent.get_id()).cloned().unwrap();
                confirmation_signatures.insert(r_plus_c_signature.clone());
                return intent.signed(confirmation_signatures)
            } else {
                let sign_with_primary = self.factory.iniate_with_recovery_sign_with_primary().collect_signatures().await?;
                if sign_with_primary.successful() {
                    let intent = self.factory.ac_recovery_intents.initiate_with_recovery_complete_with_primary.signable.clone();
                    let mut primary_signatures = sign_with_primary.all_signatures();
                    let r_plus_p_signature = recovery_signatures.iter().find(|s| s.payload_id() == &intent.get_id()).cloned().unwrap();
                    primary_signatures.insert(r_plus_p_signature.clone());
                    return intent.signed(primary_signatures)
                } else {
                    let intent = self.factory.ac_recovery_intents.initiate_with_recovery_delayed_completion.signable.clone();
                    let r_plus_t_signature = recovery_signatures.iter().find(|s| s.payload_id() == &intent.get_id()).cloned().unwrap();
                    return intent.signed(IndexSet::from([r_plus_t_signature]))
                }
            }
        } else {
            let sign_iniate_with_primary = self.factory.iniate_with_primary_sign_with_primary().collect_signatures().await?;

            if sign_iniate_with_primary.successful() {
                let sign_with_confirmation = self.factory.iniate_with_primary_sign_with_confirmation().collect_signatures().await?;
                if sign_with_confirmation.successful() {
                    let mut primary_signatures = sign_iniate_with_primary.all_signatures();
                    let mut confirmation_signatures = sign_with_confirmation.all_signatures();

                    primary_signatures.append(&mut confirmation_signatures);
                    let intent = self.factory.ac_recovery_intents.initiate_with_primary_complete_with_confirmation.signable.clone();
                    return intent.signed(primary_signatures)
                } 
            }
        }

        Err(CommonError::Unknown)
    }
}

struct ACRecoverySignatureCollectorFactory {
    profile: Profile,
    signing_interactor: Arc<dyn SignInteractor<TransactionIntent>>,
    ac_recovery_intents: AccessControllerRecoveryIntents
}

impl ACRecoverySignatureCollectorFactory {
    fn iniate_with_recovery_sign_with_recovery(&self) -> SignaturesCollector<TransactionIntent> {
        let intents = vec![
            self.ac_recovery_intents.initiate_with_recovery_complete_with_confirmation.clone(),
            self.ac_recovery_intents.initiate_with_recovery_complete_with_primary.clone(),
            self.ac_recovery_intents.initiate_with_recovery_delayed_completion.clone(),
        ];

        self.signature_collector_for(intents, RoleKind::Recovery)
    }

    fn iniate_with_recovery_sign_with_confirmation(&self) -> SignaturesCollector<TransactionIntent> {
        let intents = vec![
            self.ac_recovery_intents.initiate_with_recovery_complete_with_confirmation.clone(),
        ];

        self.signature_collector_for(intents, RoleKind::Confirmation)
    }

    fn iniate_with_recovery_sign_with_primary(&self) -> SignaturesCollector<TransactionIntent> {
        let intents = vec![
            self.ac_recovery_intents.initiate_with_recovery_complete_with_primary.clone(),
        ];

        self.signature_collector_for(intents, RoleKind::Primary)
    }

    fn iniate_with_primary_sign_with_primary(&self) -> SignaturesCollector<TransactionIntent> {
        let intents = vec![
            self.ac_recovery_intents.initiate_with_primary_complete_with_confirmation.clone(),
        ];

        self.signature_collector_for(intents, RoleKind::Primary)
    }

    fn iniate_with_primary_sign_with_confirmation(&self) -> SignaturesCollector<TransactionIntent> {
        let intents = vec![
            self.ac_recovery_intents.initiate_with_primary_complete_with_confirmation.clone(),
        ];

       self.signature_collector_for(intents, RoleKind::Confirmation)
    }

    fn signature_collector_for(&self, intents: Vec<SignableWithEntities<TransactionIntent>>, role: RoleKind) -> SignaturesCollector<TransactionIntent> {
        SignaturesCollector::with(
            SigningFinishEarlyStrategy::default(),
            self.profile.factor_sources().clone(),
            intents.into(),
            self.signing_interactor.clone(),
              SigningPurpose::sign_transaction(role)
            )
    }
}

struct AccessControllerRecoveryIntents {
    initiate_with_recovery_complete_with_confirmation: SignableWithEntities<TransactionIntent>,
    initiate_with_recovery_complete_with_primary: SignableWithEntities<TransactionIntent>,
    initiate_with_recovery_delayed_completion: SignableWithEntities<TransactionIntent>,
    initiate_with_primary_complete_with_confirmation: SignableWithEntities<TransactionIntent>,
}

impl AccessControllerRecoveryIntents {
    fn new(
        intent: TransactionIntent,
        securified_entity: AnySecurifiedEntity,
        proposed_security_structure: SecurityStructureOfFactorInstances,
    ) -> Result<Self> {
        let r_plus_c_manifest =
            TransactionManifest::apply_security_shield_for_securified_entity(
                securified_entity.clone(),
                proposed_security_structure.clone(),
                RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation,
            );
        let r_plus_p_manifest =
            TransactionManifest::apply_security_shield_for_securified_entity(
                securified_entity.clone(),
                proposed_security_structure.clone(),
                RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary,
            );
        let r_plus_t_manifest =
            TransactionManifest::apply_security_shield_for_securified_entity(
                securified_entity.clone(),
                proposed_security_structure.clone(),
                RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion,
            );
        let p_plus_c_manifest =
            TransactionManifest::apply_security_shield_for_securified_entity(
                securified_entity.clone(),
                proposed_security_structure.clone(),
                RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation,
            );

        let r_plus_c_intent = TransactionIntent::new(intent.header, r_plus_c_manifest, intent.message.clone())?;
        let r_plus_p_intent = TransactionIntent::new(intent.header, r_plus_p_manifest, intent.message.clone())?;
        let r_plus_t_intent = TransactionIntent::new(intent.header, r_plus_t_manifest, intent.message.clone())?;
        let p_plus_c_intent = TransactionIntent::new(intent.header, p_plus_c_manifest, intent.message.clone())?;

        Ok(Self {
            initiate_with_recovery_complete_with_confirmation: SignableWithEntities::with(r_plus_c_intent, vec![securified_entity.entity.clone()]),
            initiate_with_recovery_complete_with_primary: SignableWithEntities::with(r_plus_p_intent, vec![securified_entity.entity.clone()]),
            initiate_with_recovery_delayed_completion: SignableWithEntities::with(r_plus_t_intent, vec![securified_entity.entity.clone()]),
            initiate_with_primary_complete_with_confirmation: SignableWithEntities::with(p_plus_c_intent, vec![securified_entity.entity.clone()]),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

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
            .sign_transaction(signable.clone(), RoleKind::Primary)
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
            .sign_transaction(transaction, RoleKind::Primary)
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
            .sign_transaction(transaction, RoleKind::Primary)
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
            .sign_transaction(signable.clone(), RoleKind::Primary)
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
            .sign_transaction(signable.clone(), RoleKind::Primary)
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

        let outcome =
            sut.sign_transaction(transaction, RoleKind::Primary).await;

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
