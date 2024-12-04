use crate::prelude::*;
use std::ops::Index;

// ==================
// Sign Signables
// ==================
impl SargonOS {
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
    async fn test_sign_transaction_intent_success() {
        let profile = Profile::sample();
        let sut = boot_with_profile(&profile, None).await;

        let (signable, entities) = get_signable_with_entities::<
            TransactionIntent,
        >(&sut.profile().unwrap());

        let outcome = sut
            .sign_transaction(signable.clone(), RoleKind::Primary)
            .await;
        let signed = outcome.unwrap();

        assert_eq!(signable, signed.intent);
        assert_eq!(entities.len(), signed.intent_signatures.signatures.len());
    }

    #[actix_rt::test]
    async fn test_sign_subintent_success() {
        let profile = Profile::sample();
        let sut = boot_with_profile(&profile, None).await;

        let (signable, entities) =
            get_signable_with_entities::<Subintent>(&sut.profile().unwrap());

        let outcome = sut
            .sign_subintent(signable.clone(), RoleKind::Primary)
            .await;
        let signed = outcome.unwrap();

        assert_eq!(signable, signed.subintent);
        assert_eq!(
            entities.len(),
            signed.subintent_signatures.signatures.len()
        );
    }

    #[actix_rt::test]
    async fn test_sign_transaction_intent_rejected() {
        let profile = Profile::sample();
        let sut = boot_with_profile(
            &profile,
            Some(vec![profile.device_factor_sources().first().unwrap().id]),
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
    async fn test_sign_subintent_rejected() {
        let profile = Profile::sample();
        let sut = boot_with_profile(
            &profile,
            Some(vec![profile.device_factor_sources().first().unwrap().id]),
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
    async fn test_sign_fail_due_to_irrelevant_entity() {
        let profile = Profile::sample();
        let sut = boot_with_profile(
            &profile,
            Some(vec![profile.device_factor_sources().first().unwrap().id]),
        )
        .await;

        let irrelevant_account = Account::sample_mainnet_third();
        let transaction = TransactionIntent::sample_entities_requiring_auth(
            vec![&irrelevant_account],
            vec![],
        );

        let outcome =
            sut.sign_transaction(transaction, RoleKind::Primary).await;

        assert_eq!(outcome, Err(CommonError::UnknownAccount));
    }

    async fn boot_with_profile(
        profile: &Profile,
        maybe_failing_factor_sources: Option<Vec<FactorSourceIDFromHash>>,
    ) -> Arc<SUT> {
        let secure_storage_driver = EphemeralSecureStorage::new();
        let secure_storage_client =
            SecureStorageClient::new(secure_storage_driver.clone());
        secure_storage_client.save_profile(profile).await.unwrap();

        let test_drivers = Drivers::with_secure_storage(secure_storage_driver);
        let clients = Clients::new(Bios::new(test_drivers));
        let simulated_failures = SimulatedFailures::with_simulated_failures(
            maybe_failing_factor_sources.unwrap_or_default(),
        );

        let use_factor_sources_interactors =
            Arc::new(TestUseFactorSourcesInteractors::new(
                Arc::new(TestSignInteractor::<TransactionIntent>::new(
                    SimulatedUser::prudent_with_failures(
                        simulated_failures.clone(),
                    ),
                )),
                Arc::new(TestSignInteractor::<Subintent>::new(
                    SimulatedUser::prudent_with_failures(
                        simulated_failures.clone(),
                    ),
                )),
                Arc::new(TestDerivationInteractor::new(
                    false,
                    Arc::new(clients.secure_storage.clone()),
                )),
            ));
        let interactors = Interactors::new(use_factor_sources_interactors);
        SUT::boot_with_clients_and_interactor(clients, interactors).await
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
}
