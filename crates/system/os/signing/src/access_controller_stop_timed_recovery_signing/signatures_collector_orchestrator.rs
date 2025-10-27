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
