use crate::prelude::*;
use std::ops::Index;

// ==================
// Sign Signables
// ==================
impl SargonOS {
    pub async fn sign_auth(
        &self,
        address_of_entity: AddressOfAccountOrPersona,
        challenge_nonce: DappToWalletInteractionAuthChallengeNonce,
        metadata: DappToWalletInteractionMetadata,
    ) -> Result<WalletToDappInteractionAuthProof> {
        let profile = &self.profile_state_holder.profile()?;

        let auth_signer = AuthenticationSigner::new(
            self.auth_signing_interactor(),
            profile,
            address_of_entity,
            challenge_nonce,
            metadata,
        )?;

        auth_signer.sign().await
    }

    pub async fn sign_transaction(
        &self,
        transaction_intent: TransactionIntent,
        role_kind: RoleKind,
    ) -> Result<SignedIntent> {
        self.sign(
            transaction_intent.clone(),
            self.sign_transactions_interactor(),
            role_kind,
        )
        .await
    }

    pub async fn sign_subintent(
        &self,
        subintent: Subintent,
        role_kind: RoleKind,
    ) -> Result<SignedSubintent> {
        self.sign(
            subintent.clone(),
            self.sign_subintents_interactor(),
            role_kind,
        )
        .await
    }

    async fn sign<S: Signable>(
        &self,
        signable: S,
        sign_interactor: Arc<dyn SignInteractor<S>>,
        role_kind: RoleKind,
    ) -> Result<S::Signed> {
        let profile = &self.profile_state_holder.profile()?;

        let collector = SignaturesCollector::new(
            SigningFinishEarlyStrategy::default(),
            vec![signable.clone()],
            sign_interactor,
            profile,
            role_kind,
        )?;

        let outcome = collector.collect_signatures().await?;
        let payload_id = signable.get_id();

        if outcome.successful() {
            let intent_signatures = IndexSet::<IntentSignature>::from_iter(
                outcome
                    .signatures_of_successful_transactions()
                    .iter()
                    .filter(|hd| hd.input.payload_id == payload_id)
                    .map(|hd| IntentSignature(hd.signature)),
            );

            signable.signed(IntentSignatures::new(intent_signatures))
        } else {
            Err(CommonError::SigningRejected)
        }
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
        let sut = boot_with_profile(&profile, None).await;

        let all_accounts = profile.accounts_on_current_network().unwrap();
        let account = all_accounts.first().unwrap();
        let nonce = DappToWalletInteractionAuthChallengeNonce::sample();
        let metadata = DappToWalletInteractionMetadata::new(
            WalletInteractionVersion::current(),
            NetworkID::Mainnet,
            "https://example.com",
            DappDefinitionAddress::sample(),
        );

        let expected_challenge =
            RolaChallenge::from_request(nonce.clone(), metadata.clone())
                .unwrap();

        let signed = sut
            .sign_auth(
                AddressOfAccountOrPersona::Account(account.address),
                nonce,
                metadata,
            )
            .await
            .unwrap();

        let signature_with_public_key = SignatureWithPublicKey::from((
            *signed.public_key.as_ed25519().unwrap(),
            *signed.signature.as_ed25519().unwrap(),
        ));

        assert!(signature_with_public_key
            .is_valid_for_hash(&expected_challenge.hash()))
    }

    #[actix_rt::test]
    async fn test_sign_auth_failure() {
        let profile = Profile::sample();

        let sut =
            boot_with_profile(&profile, Some(SigningFailure::UserRejected))
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

        let result = sut
            .sign_auth(
                AddressOfAccountOrPersona::Account(account.address),
                nonce,
                metadata,
            )
            .await;

        assert_eq!(result, Err(CommonError::SigningRejected))
    }

    #[actix_rt::test]
    async fn test_sign_transaction_intent_success() {
        let profile = Profile::sample();
        let sut = boot_with_profile(&profile, None).await;

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
        let sut = boot_with_profile(&profile, None).await;

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
        let sut = boot_with_profile(&profile, None).await;

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
        let sut = boot_with_profile(&profile, None).await;

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
        let sut = boot_with_profile(
            &profile,
            Some(SigningFailure::NeglectedFactorSources(vec![
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

        assert_eq!(outcome, Err(CommonError::SigningRejected));
    }

    #[actix_rt::test]
    async fn test_sign_transaction_subintent_rejected_due_to_all_factors_neglected(
    ) {
        let profile = Profile::sample();
        let sut = boot_with_profile(
            &profile,
            Some(SigningFailure::NeglectedFactorSources(vec![
                profile.device_factor_sources().first().unwrap().id,
            ])),
        )
        .await;

        let (signable, _) =
            get_signable_with_entities::<Subintent>(&sut.profile().unwrap());

        let outcome = sut
            .sign_subintent(signable.clone(), RoleKind::Primary)
            .await;

        assert_eq!(outcome, Err(CommonError::SigningRejected));
    }

    #[actix_rt::test]
    async fn test_sign_fail_due_to_profile() {
        let test_drivers = Drivers::test();
        let clients = Clients::new(Bios::new(test_drivers));
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
            boot_with_profile(&profile, Some(SigningFailure::UserRejected))
                .await;

        let (signable, _) =
            get_signable_with_entities::<Subintent>(&sut.profile().unwrap());

        let outcome = sut
            .sign_subintent(signable.clone(), RoleKind::Primary)
            .await;

        assert_eq!(outcome, Err(CommonError::SigningRejected));
    }

    async fn boot_with_profile(
        profile: &Profile,
        maybe_signing_failure: Option<SigningFailure>,
    ) -> Arc<SUT> {
        let secure_storage_driver = EphemeralSecureStorage::new();
        let secure_storage_client =
            SecureStorageClient::new(secure_storage_driver.clone());
        secure_storage_client.save_profile(profile).await.unwrap();

        let test_drivers = Drivers::with_secure_storage(secure_storage_driver);
        let clients = Clients::new(Bios::new(test_drivers));

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
                get_test_auth_interactor(&maybe_signing_failure),
            ));
        let interactors = Interactors::new(use_factor_sources_interactors);
        SUT::boot_with_clients_and_interactor(clients, interactors).await
    }

    fn get_simulated_user<S: Signable>(
        maybe_signing_failure: &Option<SigningFailure>,
    ) -> SimulatedUser<S> {
        match maybe_signing_failure {
            None => SimulatedUser::<S>::prudent_no_fail(),
            Some(failure) => match failure {
                SigningFailure::NeglectedFactorSources(factor_sources) => {
                    SimulatedUser::<S>::prudent_with_failures(
                        SimulatedFailures::with_simulated_failures(
                            factor_sources.clone(),
                        ),
                    )
                }
                SigningFailure::UserRejected => SimulatedUser::<S>::rejecting(),
            },
        }
    }

    fn get_test_auth_interactor(
        maybe_signing_failure: &Option<SigningFailure>,
    ) -> Arc<dyn AuthenticationSigningInteractor> {
        match maybe_signing_failure {
            None => Arc::new(TestAuthenticationInteractor::new_succeeding()),
            Some(_) => Arc::new(TestAuthenticationInteractor::new_failing()),
        }
    }

    fn get_signable_with_entities<S: Signable>(
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
        NeglectedFactorSources(Vec<FactorSourceIDFromHash>),
        UserRejected,
    }
}
