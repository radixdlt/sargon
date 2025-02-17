use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedIntentSet {
    intents: Vec<EntitySignedFor>, // Want IndexSet but TransactionIntent is not `std::hash::Hash`
}
impl SignedIntentSet {
    pub fn get_best_signed_intent(self) -> Result<SignedIntentWithContext> {
        let first =
            self.intents.first().ok_or(CommonError::Unknown).cloned()?; // TODO specific error variant

        let from = |item: EntitySignedFor| -> Result<SignedIntentWithContext> {
            let intent = item.intent.clone();
            let signatures = item.intent_signatures();

            let signed_intent =
                SignedIntent::new(intent, IntentSignatures::new(signatures))?;

            Ok(SignedIntentWithContext {
                signed_intent,
                context: item.context,
            })
        };

        if self.intents.len() == 1 {
            from(first)
        } else {
            assert!(self.intents.iter().all(|i| i.variant().is_some()));

            let rated_by_tx_variant = self
                .intents
                .into_iter()
                .sorted_by_key(|i| i.variant().unwrap().rating())
                .collect_vec();
            let best = rated_by_tx_variant.first().unwrap().clone();
            from(best)
        }
    }
}
