use crate::prelude::*;

pub struct SignaturesCollectorOrchestrator {
    factory: SignaturesCollectorFactory,
}

impl SignaturesCollectorOrchestrator {
    pub async fn sign(&self) -> Result<SignedIntent> {
        let outcome = self
            .iniate_with_recovery_flow()
            .await?
            .or(self.iniate_with_primary_flow().await?);

        match outcome {
            Some(signatures) => {
                let intent_hash = signatures.first().unwrap().payload_id();
                let intent = self.factory.intent_for_hash(intent_hash).unwrap();
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
            let confirming_role_outcome = self.factory.initiate_recovery_with_recovery_sign_with_confirmation_role_collector().collect_signatures().await?
        .success_or(
            self.factory.initiate_recovery_with_recovery_sign_with_primary_role_collector().collect_signatures().await?
        );
            if confirming_role_outcome.successful() {
                let mut confirming_role_signatures =
                    confirming_role_outcome.all_signatures();
                let intent_hash =
                    confirming_role_signatures.first().unwrap().payload_id();
                let recovery_signature = recovery_role_signatures
                    .iter()
                    .find(|sig| *sig.payload_id() == *intent_hash)
                    .unwrap()
                    .clone();
                confirming_role_signatures.insert(recovery_signature);
                return Ok(Some(confirming_role_signatures));
            } else {
                let signature = recovery_role_signatures
                    .into_iter()
                    .find(|sig| {
                        *sig.payload_id()
                            == self.factory.intent_hash_of_timed_recovery()
                    })
                    .unwrap();
                return Ok(Some(IndexSet::from_iter(vec![signature])));
            }
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
