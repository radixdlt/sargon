use crate::prelude::*;

pub(crate) struct SigningManagerIntermediaryOutcome {
    successfully_signed_intent_sets: Vec<SignedIntentSet>,
    failed_intent_sets: Vec<SignedIntentSet>,
}

impl SigningManagerIntermediaryOutcome {
    pub(crate) fn get_best_signed_intents(
        self,
    ) -> Result<Vec<SignedIntentWithContext>> {
        // TODO: Implement support for handling of failed transactions, i.e. submit the successful ones even if some failed and do SOMETHING with the failed ones
        let signed_sets = self.validate_all_intent_sets_signed()?;

        // We are not going to submit multiple manifest variants for each "manifest set",
        // we only want the "best one" for each set.
        signed_sets
            .into_iter()
            .map(|signed_set| signed_set.get_best_signed_intent())
            .collect::<Result<Vec<_>>>()
    }

    // TODO: Implement support for handling of failed transactions, i.e. submit the successful ones even if some failed and do SOMETHING with the failed ones
    fn validate_all_intent_sets_signed(self) -> Result<Vec<SignedIntentSet>> {
        if self.failed_intent_sets.is_empty() {
            Ok(self.successfully_signed_intent_sets)
        } else {
            Err(CommonError::Unknown) // TODO specific error variant
        }
    }
}
