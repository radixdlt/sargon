use crate::prelude::*;

pub struct StopTimedRecoverySignaturesCollectorOrchestrator {
    factory: StopTimedRecoverySignaturesCollectorFactory,
}

impl StopTimedRecoverySignaturesCollectorOrchestrator {
    pub fn new(factory: StopTimedRecoverySignaturesCollectorFactory) -> Self {
        Self { factory }
    }

    pub async fn sign(&self) -> Result<SignedIntent> {
        let outcome = {
            let recovery_attempt =
                self.try_stop_and_cancel_with_recovery().await?;
            match recovery_attempt {
                Some(signatures) => Some(signatures),
                None => self.try_stop_with_primary_or_confirmation().await?,
            }
        };

        match outcome {
            Some(mut signatures) => {
                let intent_hash = signatures
                    .first()
                    .map(|signature| signature.payload_id().clone())
                    .unwrap();

                let post_processing_signatures = self
                    .collect_post_processing_signatures(&intent_hash)
                    .await?;

                post_processing_signatures
                    .into_iter()
                    .for_each(|signature| {
                        signatures.insert(signature);
                    });

                let intent =
                    self.factory.intent_for_hash(&intent_hash).unwrap();
                let intent_signatures = signatures
                    .into_iter()
                    .map(|hd| IntentSignature(hd.signature))
                    .collect_vec();

                SignedIntent::new(
                    intent,
                    IntentSignatures::new(intent_signatures),
                )
            }
            None => {
                Err(CommonError::SigningFailedTooManyFactorSourcesNeglected)
            }
        }
    }

    async fn collect_post_processing_signatures(
        &self,
        intent_hash: &TransactionIntentHash,
    ) -> Result<IndexSet<HDSignature<TransactionIntentHash>>> {
        if let Some(collector) = self
            .factory
            .signature_collector_for_post_processing_signatures(intent_hash)?
        {
            let outcome = collector.collect_signatures().await?;
            if outcome.successful() {
                Ok(outcome.all_signatures())
            } else {
                Err(CommonError::SigningFailedTooManyFactorSourcesNeglected)
            }
        } else {
            Ok(IndexSet::new())
        }
    }

    async fn try_stop_and_cancel_with_recovery(
        &self,
    ) -> Result<Option<IndexSet<HDSignature<TransactionIntentHash>>>> {
        let outcome = self
            .factory
            .stop_and_cancel_with_recovery_role_collector()
            .collect_signatures()
            .await?;

        if outcome.successful() {
            Ok(Some(outcome.all_signatures()))
        } else {
            Ok(None)
        }
    }

    async fn try_stop_with_primary_or_confirmation(
        &self,
    ) -> Result<Option<IndexSet<HDSignature<TransactionIntentHash>>>> {
        let primary_outcome = self
            .factory
            .stop_with_primary_role_collector()
            .collect_signatures()
            .await?;

        if primary_outcome.successful() {
            return Ok(Some(primary_outcome.all_signatures()));
        }

        let confirmation_outcome = self
            .factory
            .stop_with_confirmation_role_collector()
            .collect_signatures()
            .await?;

        if confirmation_outcome.successful() {
            Ok(Some(confirmation_outcome.all_signatures()))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};

    type CollectorResult = Result<SignaturesOutcome<TransactionIntentHash>>;

    struct StubCollectorBuilder {
        outcomes: Mutex<VecDeque<CollectorResult>>,
    }

    impl StubCollectorBuilder {
        fn new(outcomes: Vec<CollectorResult>) -> Self {
            Self {
                outcomes: Mutex::new(VecDeque::from(outcomes)),
            }
        }
    }

    impl TransactionIntentSignaturesCollectorBuilder for StubCollectorBuilder {
        fn build(
            &self,
            _: IdentifiedVecOf<SignableWithEntities<TransactionIntent>>,
            _: SigningPurpose,
        ) -> Box<dyn TransactionIntentSignaturesCollector> {
            let outcome = self
                .outcomes
                .lock()
                .expect("collector outcomes mutex poisoned")
                .pop_front()
                .expect("unexpected build call");

            Box::new(StubCollector { outcome })
        }
    }

    struct StubCollector {
        outcome: CollectorResult,
    }

    #[async_trait::async_trait]
    impl TransactionIntentSignaturesCollector for StubCollector {
        async fn collect_signatures(
            self: Box<Self>,
        ) -> Result<SignaturesOutcome<TransactionIntentHash>> {
            let this = *self;
            this.outcome
        }
    }

    #[actix_rt::test]
    async fn skipping_all_factor_sources_results_in_error() {
        let failure_outcome = SignaturesOutcome::<TransactionIntentHash>::with_failed_transactions_due_to_skipped_factors(vec![
            FactorSourceIDFromHash::sample(),
        ]);

        let collector_builder: Arc<dyn TransactionIntentSignaturesCollectorBuilder> =
            Arc::new(StubCollectorBuilder::new(vec![
                Ok(failure_outcome.clone()),
                Ok(failure_outcome.clone()),
                Ok(failure_outcome),
            ]));

        let base_intent = TransactionIntent::sample();
        let lock_fee_data = LockFeeData::new_with_unsecurified_fee_payer(
            AccountAddress::sample(),
            Decimal192::one(),
        );
        let ac_state_details = AccessControllerStateDetails::new(
            AccessControllerAddress::sample_mainnet(),
            AccessControllerFieldStateValue::new(
                EntityReference::new(
                    CoreApiEntityType::GlobalAccessController,
                    false,
                    "".to_string(),
                ),
                None,
                None,
                ResourceAddress::sample_mainnet(),
                false,
                None,
                false,
                None,
                false,
            ),
            Decimal192::zero(),
        );
        let mut securified_entity = AnySecurifiedEntity::sample_account();
        securified_entity
            .securified_entity_control
            .set_provisional(Some(ProvisionalSecurifiedConfig::sample()));

        let factory = StopTimedRecoverySignaturesCollectorFactory::new(
            base_intent,
            securified_entity,
            lock_fee_data,
            ac_state_details,
            collector_builder,
        )
        .expect("failed to construct collector factory");

        let orchestrator =
            StopTimedRecoverySignaturesCollectorOrchestrator::new(factory);

        let error = orchestrator.sign().await.unwrap_err();
        assert_eq!(
            error,
            CommonError::SigningFailedTooManyFactorSourcesNeglected,
        );
    }
}
