use crate::prelude::*;

pub struct SignaturesCollectorOrchestrator {
    factory: SignaturesCollectorFactory,
}

impl SignaturesCollectorOrchestrator {
    pub fn new(factory: SignaturesCollectorFactory) -> Self {
        Self { factory }
    }

    pub async fn sign(&self) -> Result<SignedIntent> {
        let outcome = {
            let iniate_with_recovery_result =
                self.iniate_with_recovery_flow().await?;
            match iniate_with_recovery_result {
                Some(value) => Some(value),
                None => self.iniate_with_primary_flow().await?,
            }
        };

        match outcome {
            Some(mut signatures) => {
                let intent_hash = signatures
                    .first()
                    .map(|sig| sig.payload_id().clone())
                    .ok_or(CommonError::TooFewFactorInstancesDerived)?;

                let fee_payer_signatures =
                    self.collect_fee_payer_signatures(&intent_hash).await?;

                fee_payer_signatures.into_iter().for_each(|signature| {
                    signatures.insert(signature);
                });

                let intent = self
                    .factory
                    .intent_for_hash(&intent_hash)
                    .ok_or(CommonError::TooFewFactorInstancesDerived)?;
                let intent_signatures = signatures
                    .into_iter()
                    .map(|hd| IntentSignature(hd.signature))
                    .collect_vec();

                SignedIntent::new(
                    intent,
                    IntentSignatures::new(intent_signatures),
                )
            }
            None => Err(CommonError::TooFewFactorInstancesDerived),
        }
    }

    async fn collect_fee_payer_signatures(
        &self,
        intent_hash: &TransactionIntentHash,
    ) -> Result<IndexSet<HDSignature<TransactionIntentHash>>> {
        let Some(collector) = self
            .factory
            .fee_payer_sign_with_primary_role_collector_for(intent_hash)
        else {
            return Ok(IndexSet::new());
        };

        let outcome = collector.collect_signatures().await?;

        if !outcome.successful() {
            return Err(CommonError::TooFewFactorInstancesDerived);
        }

        let signatures = outcome.signatures_of_successful_transactions();
        if signatures.is_empty() {
            return Err(CommonError::TooFewFactorInstancesDerived);
        }

        Ok(signatures)
    }

    async fn iniate_with_recovery_flow(
        &self,
    ) -> Result<Option<IndexSet<HDSignature<TransactionIntentHash>>>> {
        let recovery_role_signatures_outcome = self
            .factory
            .initiate_recovery_with_recovery_sign_with_recovery_role_collector()
            .collect_signatures()
            .await?;

        if recovery_role_signatures_outcome.successful() {
            let recovery_role_signatures =
                recovery_role_signatures_outcome.all_signatures();

            let confirmation_role_outcome = self.factory.initiate_recovery_with_recovery_sign_with_confirmation_role_collector().collect_signatures().await?;
            if confirmation_role_outcome.successful() {
                let mut confirmation_role_signatures =
                    confirmation_role_outcome.all_signatures();

                let intent_hash =
                    confirmation_role_signatures.first().unwrap().payload_id();
                let recovery_signature = recovery_role_signatures
                    .iter()
                    .find(|sig| *sig.payload_id() == *intent_hash)
                    .unwrap()
                    .clone();
                confirmation_role_signatures.insert(recovery_signature);
                return Ok(Some(confirmation_role_signatures));
            }

            let primary_role_signatures_outcome = self.factory.initiate_recovery_with_recovery_sign_with_primary_role_collector().collect_signatures().await?;
            if primary_role_signatures_outcome.successful() {
                let mut primary_role_signatures =
                    primary_role_signatures_outcome.all_signatures();

                let intent_hash =
                    primary_role_signatures.first().unwrap().payload_id();
                let recovery_signature = recovery_role_signatures
                    .iter()
                    .find(|sig| *sig.payload_id() == *intent_hash)
                    .unwrap()
                    .clone();
                primary_role_signatures.insert(recovery_signature);
                return Ok(Some(primary_role_signatures));
            }

            let signature = recovery_role_signatures
                .into_iter()
                .find(|sig| {
                    *sig.payload_id()
                        == self.factory.intent_hash_of_timed_recovery()
                })
                .unwrap();
            return Ok(Some(IndexSet::from_iter(vec![signature])));
        }

        Ok(None)
    }

    async fn iniate_with_primary_flow(
        &self,
    ) -> Result<Option<IndexSet<HDSignature<TransactionIntentHash>>>> {
        let primary_role_signatures_outcome = self
            .factory
            .initiate_recovery_with_primary_sign_with_primary_role_collector()
            .collect_signatures()
            .await?;

        if primary_role_signatures_outcome.successful() {
            let confirmation_role_signatures_outcome = self.factory.initiate_recovery_with_primary_sign_with_confirmation_role_collector().collect_signatures().await?;
            if confirmation_role_signatures_outcome.successful() {
                let mut primary_role_signatures =
                    primary_role_signatures_outcome.all_signatures();
                let mut confirmation_role_signatures =
                    confirmation_role_signatures_outcome.all_signatures();

                // merge signatures
                primary_role_signatures
                    .append(&mut confirmation_role_signatures);

                return Ok(Some(primary_role_signatures));
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    fn factor_sources_for_security_structure(
        security_structure: &SecurityStructureOfFactorInstances,
    ) -> IndexSet<FactorSource> {
        let required_ids = security_structure
            .unique_all_factor_instances()
            .into_iter()
            .filter_map(|factor_instance| {
                factor_instance
                    .try_as_hd_factor_instances()
                    .ok()
                    .map(|hd| hd.factor_source_id())
            })
            .collect::<IndexSet<_>>();

        FactorSource::sample_values_all()
            .into_iter()
            .filter(|factor_source| {
                factor_source
                    .factor_source_id()
                    .as_hash()
                    .map(|id| required_ids.contains(id))
                    .unwrap_or(false)
            })
            .collect()
    }

    struct FixtureMetadata {
        profile_factor_sources: IndexSet<FactorSource>,
        recovery_confirmation_hash: TransactionIntentHash,
        recovery_primary_hash: TransactionIntentHash,
        recovery_delayed_hash: TransactionIntentHash,
        primary_confirmation_hash: TransactionIntentHash,
    }

    fn build_fixture() -> (FixtureMetadata, AccessControllerRecoveryIntents) {
        let mut account = Account::sample_mainnet_alice();
        let security_structure = SecurityStructureOfFactorInstances::sample();
        let securified_control = SecuredEntityControl::new(
            None,
            AccessControllerAddress::sample(),
            security_structure.clone(),
        )
        .expect("fixture: secured control");
        account.set_security_state_unchecked(EntitySecurityState::from(
            securified_control.clone(),
        ));

        let securified_account = SecurifiedAccount::try_from(account)
            .expect("fixture: securified account");
        let securified_entity: AnySecurifiedEntity = securified_account.into();

        let base_intent = TransactionIntent::sample();
        let fee_payer_account = Account::sample_mainnet_other();
        let lock_fee_data = LockFeeData::new_with_unsecurified_fee_payer(
            fee_payer_account.address,
            Decimal192::from(0),
        );

        let recovery_intents = AccessControllerRecoveryIntentsBuilder::new(
            base_intent,
            lock_fee_data,
            securified_entity,
            security_structure.clone(),
            Some(fee_payer_account),
        )
        .build()
        .expect("fixture: build intents");

        let profile_factor_sources =
            factor_sources_for_security_structure(&security_structure);

        let metadata = FixtureMetadata {
            profile_factor_sources,
            recovery_confirmation_hash: recovery_intents
                .initiate_with_recovery_complete_with_confirmation
                .id
                .clone(),
            recovery_primary_hash: recovery_intents
                .initiate_with_recovery_complete_with_primary
                .id
                .clone(),
            recovery_delayed_hash: recovery_intents
                .initiate_with_recovery_delayed_completion
                .id
                .clone(),
            primary_confirmation_hash: recovery_intents
                .initiate_with_primary_complete_with_confirmation
                .id
                .clone(),
        };

        (metadata, recovery_intents)
    }

    fn make_orchestrator(
        metadata: &FixtureMetadata,
        recovery_intents: AccessControllerRecoveryIntents,
        interactor: Arc<dyn SignInteractor<TransactionIntent>>,
    ) -> SignaturesCollectorOrchestrator {
        let factory = SignaturesCollectorFactory::new(
            SigningFinishEarlyStrategy::default(),
            metadata.profile_factor_sources.clone(),
            interactor,
            recovery_intents,
        );

        SignaturesCollectorOrchestrator { factory }
    }

    fn hashes_in_request(
        request: &SignRequest<TransactionIntent>,
    ) -> IndexSet<TransactionIntentHash> {
        request
            .per_factor_source
            .values()
            .flat_map(|input| input.per_transaction.iter())
            .map(|tx_input| tx_input.payload.clone().into())
            .collect()
    }

    struct SelectiveSignInteractor {
        delegate: TestSignInteractor<TransactionIntent>,
        skip_hashes: IndexSet<TransactionIntentHash>,
        fail_hashes: IndexSet<TransactionIntentHash>,
    }

    impl SelectiveSignInteractor {
        fn signing_only(
            simulated_user: SimulatedUser<TransactionIntent>,
        ) -> Self {
            Self {
                delegate: TestSignInteractor::new(simulated_user),
                skip_hashes: IndexSet::new(),
                fail_hashes: IndexSet::new(),
            }
        }

        fn with_behaviour(
            simulated_user: SimulatedUser<TransactionIntent>,
            skip_hashes: IndexSet<TransactionIntentHash>,
            fail_hashes: IndexSet<TransactionIntentHash>,
        ) -> Self {
            Self {
                delegate: TestSignInteractor::new(simulated_user),
                skip_hashes,
                fail_hashes,
            }
        }
    }

    #[async_trait::async_trait]
    impl SignInteractor<TransactionIntent> for SelectiveSignInteractor {
        async fn sign(
            &self,
            request: SignRequest<TransactionIntent>,
        ) -> Result<SignResponse<TransactionIntentHash>> {
            let hashes = hashes_in_request(&request);
            let factor_ids = request.factor_source_ids();

            if !hashes.is_empty()
                && hashes.iter().all(|hash| self.fail_hashes.contains(hash))
            {
                let outcomes = IndexMap::from_iter(
                    factor_ids
                        .iter()
                        .map(|id| (*id, FactorOutcome::failure(*id))),
                );

                return SignResponse::new_from_outcomes(outcomes);
            }

            if !hashes.is_empty()
                && hashes.iter().all(|hash| self.skip_hashes.contains(hash))
            {
                return Ok(SignResponse::user_skipped_factors(factor_ids));
            }

            self.delegate.sign(request).await
        }
    }

    #[actix_rt::test]
    async fn sign_uses_recovery_flow_when_all_roles_succeed() {
        let (metadata, recovery_intents) = build_fixture();
        let interactor =
            Arc::new(SelectiveSignInteractor::signing_only(SimulatedUser::<
                TransactionIntent,
            >::prudent_no_fail(
            )));

        let orchestrator =
            make_orchestrator(&metadata, recovery_intents, interactor);

        let signed = orchestrator.sign().await.expect("recovery flow success");

        assert_eq!(
            signed.intent.transaction_intent_hash(),
            metadata.recovery_confirmation_hash
        );
        assert!(
            signed.intent_signatures.signatures.len() >= 3,
            "should capture signatures from recovery, confirmation, and fee payer primary roles"
        );
    }

    #[actix_rt::test]
    async fn sign_falls_back_to_primary_flow_when_recovery_skipped() {
        let (metadata, recovery_intents) = build_fixture();

        let skip_hashes = IndexSet::from_iter([
            metadata.recovery_confirmation_hash.clone(),
            metadata.recovery_primary_hash.clone(),
            metadata.recovery_delayed_hash.clone(),
        ]);

        let interactor = Arc::new(SelectiveSignInteractor::with_behaviour(
            SimulatedUser::<TransactionIntent>::prudent_no_fail(),
            skip_hashes,
            IndexSet::new(),
        ));

        let orchestrator =
            make_orchestrator(&metadata, recovery_intents, interactor);

        let signed = orchestrator.sign().await.expect("fallback to primary");

        assert_eq!(
            signed.intent.transaction_intent_hash(),
            metadata.primary_confirmation_hash
        );
        assert_ne!(
            signed.intent.transaction_intent_hash(),
            metadata.recovery_confirmation_hash
        );
        assert!(
            signed.intent_signatures.signatures.len() >= 3,
            "should include fee payer signature even on primary flow"
        );
    }

    #[actix_rt::test]
    async fn sign_returns_timed_recovery_signature_when_confirmation_and_primary_fail(
    ) {
        let (metadata, recovery_intents) = build_fixture();

        let fail_hashes = IndexSet::from_iter([
            metadata.recovery_confirmation_hash.clone(),
            metadata.recovery_primary_hash.clone(),
        ]);

        let interactor = Arc::new(SelectiveSignInteractor::with_behaviour(
            SimulatedUser::<TransactionIntent>::prudent_no_fail(),
            IndexSet::new(),
            fail_hashes,
        ));

        let orchestrator =
            make_orchestrator(&metadata, recovery_intents, interactor);

        let signed = orchestrator
            .sign()
            .await
            .expect("timed recovery fallback to succeed");

        assert_eq!(
            signed.intent.transaction_intent_hash(),
            metadata.recovery_delayed_hash
        );
        assert!(
            signed.intent_signatures.signatures.len() >= 2,
            "should combine timed recovery and fee payer signatures"
        );
    }

    #[actix_rt::test]
    async fn sign_fails_when_all_flows_neglected() {
        let (metadata, recovery_intents) = build_fixture();

        let interactor = Arc::new(SelectiveSignInteractor::with_behaviour(
            SimulatedUser::<TransactionIntent>::lazy_always_skip_no_fail(),
            IndexSet::new(),
            IndexSet::new(),
        ));

        let orchestrator =
            make_orchestrator(&metadata, recovery_intents, interactor);

        let error = orchestrator.sign().await.expect_err("all flows neglected");

        assert_eq!(error, CommonError::TooFewFactorInstancesDerived);
    }
}
