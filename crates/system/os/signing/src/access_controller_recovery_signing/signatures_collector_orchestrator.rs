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

                let post_processing_signatures = self.collect_post_processing_signatures(&intent_hash).await?;
               
                post_processing_signatures.into_iter().for_each(|signature| {
                    signatures.insert(signature);
                });

                let intent = self
                    .factory
                    .intent_for_hash(&intent_hash)
                    .unwrap();
                let intent_signatures = signatures
                    .into_iter()
                    .map(|hd| IntentSignature(hd.signature))
                    .collect_vec();

                SignedIntent::new(
                    intent,
                    IntentSignatures::new(intent_signatures),
                )
            }
            None => Err(CommonError::SigningFailedTooManyFactorSourcesNeglected),
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
        if let Some(collector) = self.factory.signature_collector_for_post_processing_signatures(intent_hash)? {
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use std::sync::Arc;

//     #[actix_rt::test]
//     async fn proper_signature_requests_are_made_while_user_skips_all_factors() {
//         let interactor = MockSignInteractor::<TransactionIntent>::new();

//         let profile = Profile::sample();

//         let base_intent = TransactionIntent::sample();
//         let lock_fee_data = LockFeeData::new_with_securified_fee_payer(AccountAddress::sample_frank(), AccessControllerAddress::sample_mainnet(), Decimal192::eight());
//         let existing_security_structure = SecurityStructureOfFactorInstances::sample_sim();
//         let proposed_security_structure = SecurityStructureOfFactorInstances::sample_other_sim();
//         let securified_entity = AnySecurifiedEntity::with_securified_entity_control(
//             Account::sample_sim().into(), 
//             SecuredEntityControl::new(
//             None, 
//             AccessControllerAddress::sample_mainnet(), 
//             existing_security_structure
//         )
//         .unwrap()
//     );

//         let fee_payer_account = None;

//         let recovery_intents = AccessControllerRecoveryIntentsBuilder::new(
//             base_intent,
//             lock_fee_data,
//             securified_entity.clone(),
//             proposed_security_structure.clone(),
//             fee_payer_account
//         ).build().unwrap();

//         let sign_request = SignRequest::<TransactionIntent>::new(
//             existing_security_structure.matrix_of_factors.recovery().all_factors().first().unwrap().factor_source_id.get_factor_source_kind(), 
//             IndexMap::from([
//                 (
//                     existing_security_structure.matrix_of_factors.recovery().all_factors().first().unwrap().factor_source_id,
//                     PerFactorSourceInput::<TransactionIntent>::new(
//                         existing_security_structure.matrix_of_factors.recovery().all_factors().first().unwrap().factor_source_id,,
//                         per_transaction,
//                         invalid_transactions_if_neglected
//                     )
//                 )
//             ])
//         );
//         interactor
//         .expect_sign(eq(SignRequest<TransactionIntent>::))
//         .with(eq())
//                 .once()
//                 .return_const(Some(request.clone()));

//         let factory = SignaturesCollectorFactory::new(
//             SigningFinishEarlyStrategy::default(),
//             FactorSource::sample_all(),
//             Arc::new(interactor),
//             recovery_intents,
//             securified_entity,
//             proposed_security_structure
//         );

//         let sut = SignaturesCollectorOrchestrator::new(factory);

//         sut.sign().await.unwrap();
//     }
// }
