use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
pub(crate) struct EntitySignedForWithVariant {
    intent_set_id: IntentSetID,
    pub(crate) intent: TransactionIntent,
    pub(crate) entity: AccountOrPersona,
    pub(crate) signatures: IndexSet<SignatureWithPublicKey>,
    pub(crate) variant:
        Option<RolesExercisableInTransactionManifestCombination>,
}
impl EntitySignedForWithVariant {
    pub(crate) fn intent_signatures(&self) -> Vec<IntentSignature> {
        self.signatures
            .iter()
            .map(|s| IntentSignature::from(s.clone()))
            .collect()
    }
    pub(crate) fn new(
        intent_set_id: IntentSetID,
        intent: TransactionIntent,
        entity: AccountOrPersona,
        signatures: IndexSet<SignatureWithPublicKey>,
        variant: Option<RolesExercisableInTransactionManifestCombination>,
    ) -> Self {
        Self {
            intent_set_id,
            intent,
            entity,
            signatures,
            variant,
        }
    }
    pub(crate) fn intent_set_id(&self) -> IntentSetID {
        self.intent_set_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SignedIntentSet {
    intent_set_id: IntentSetID,
    intents: Vec<EntitySignedForWithVariant>, // Want IndexSet but TransactionIntent is not `std::hash::Hash`
}
impl SignedIntentSet {
    /// # Panics
    /// Panics if any `intent: EntitySignedFor`'s intent set ID is not equal to `intent_set_id`
    pub(crate) fn new(
        intent_set_id: IntentSetID,
        intents: Vec<EntitySignedForWithVariant>,
    ) -> Self {
        assert!(
            intents.iter().all(|i| i.intent_set_id() == intent_set_id),
            "Discrepancy between intent set ID and intent set ID of intents"
        );
        Self {
            intent_set_id,
            intents,
        }
    }

    pub fn get_best_signed_intent(self) -> Result<SignedIntentWithContext> {
        let first =
            self.intents.first().ok_or(CommonError::Unknown).cloned()?; // TODO specific error variant

        let from = |with_variant: EntitySignedForWithVariant| -> Result<SignedIntentWithContext> {
            let intent = with_variant.intent.clone();
            let signatures = with_variant.intent_signatures();

            let signed_intent =
                SignedIntent::new(intent, IntentSignatures::new(signatures))?;

            Ok(SignedIntentWithContext::new(
                with_variant.intent_set_id(),
                signed_intent,
                with_variant.entity,
                with_variant.variant,
            ))
        };

        if self.intents.len() == 1 {
            assert!(first.variant.is_none());
            from(first)
        } else {
            assert!(self.intents.iter().all(|i| i.variant.is_some()));

            let rated_by_tx_variant = self
                .intents
                .into_iter()
                .sorted_by_key(|i| i.variant.unwrap().rating())
                .collect_vec();
            let best = rated_by_tx_variant.first().unwrap().clone();
            from(best)
        }
    }
}
