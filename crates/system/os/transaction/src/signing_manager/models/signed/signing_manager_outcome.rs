use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SigningManagerOutcome(pub Vec<SignedIntentWithOwners>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedIntentWithOwners {
    /// Used primaryly for testing
    pub(crate) intent_set_id: IntentSetID,
    pub intent: TransactionIntent,
    pub intent_signatures: Vec<OwnedIntentSignature>,
}

impl SignedIntentWithOwners {
    pub fn signed_intent(&self) -> Result<SignedIntent> {
        SignedIntent::new(
            self.intent.clone(),
            IntentSignatures::new(
                self.intent_signatures.clone().into_iter().map(|o| o.value),
            ),
        )
    }

    pub fn new(
        intent_set_id: IntentSetID,
        intent: TransactionIntent,
        intent_signatures: impl IntoIterator<Item = OwnedIntentSignature>,
    ) -> Self {
        Self {
            intent_set_id,
            intent,
            intent_signatures: intent_signatures.into_iter().collect(),
        }
    }
}
