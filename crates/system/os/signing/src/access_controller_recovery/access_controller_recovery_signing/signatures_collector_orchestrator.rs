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

    /// Collects additional signatures if needed after the final intent candidate was determined.
    /// Additional signatures could be required for:
    /// - Fee payer.
    /// - Rola key configuration.
    ///
    /// These will need to exercise the Primary role.
    async fn collect_post_processing_signatures(
        &self,
        intent_hash: &TransactionIntentHash,
    ) -> Result<IndexSet<HDSignature<TransactionIntentHash>>> {
        if let Some(collector) = self
            .factory
            .signature_collector_for_post_processing_signatures(intent_hash)?
        {
            let signatures_otucome = collector.collect_signatures().await?;
            if signatures_otucome.successful() {
                Ok(signatures_otucome.all_signatures())
            } else {
                Err(CommonError::SigningFailedTooManyFactorSourcesNeglected)
            }
        } else {
            Ok(IndexSet::new())
        }
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