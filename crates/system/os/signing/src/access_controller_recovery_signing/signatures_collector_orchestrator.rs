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

                let new_primary_signatures =
                    self.collect_new_primary_signatures(&intent_hash).await?;
                new_primary_signatures.into_iter().for_each(|signature| {
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

    async fn collect_new_primary_signatures(
        &self,
        intent_hash: &TransactionIntentHash,
    ) -> Result<IndexSet<HDSignature<TransactionIntentHash>>> {
        let Some(collector) =
            self.factory.sign_with_new_primary_if_needed(intent_hash)
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

    // #[actix_rt::test]
    // async fn proper_signature_requests_are_made_while_user_skips_all_factors() {
    //     let interactor = MockSignInteractor::<TransactionIntent>::new();

    //     let intents = AccessControllerRecoveryIntentsBuilder::new(
    //         base_intent,
    //         lock_fee_data,
    //         securified_entity,
    //         proposed_security_structure,
    //         fee_payer_account
    //     ).build().unwrap()
    //     // let interactor = SignInteractor
    //     let factory = SignaturesCollectorFactory::new(
    //         SigningFinishEarlyStrategy::default(),
    //         Profile::sample().factor_sources,
    //         Arc::new(interactor),
    //         recovery_intents
    //     );
    //     let sut = SignaturesCollectorOrchestrator::new(factory);
    // }
}
