use crate::prelude::*;

pub struct NotaryManager {
    keys_for_intents: IndexMap<TransactionIntentHash, Ed25519PrivateKey>,
}

impl NotaryManager {
    pub fn notarize(
        self,
        signed_intents: impl IntoIterator<Item = SignedIntent>,
    ) -> Result<Vec<NotarizedTransaction>> {
        let signed_intents = signed_intents.into_iter().collect_vec();
        let mut key_for_intent = self.keys_for_intents;
        signed_intents
            .into_iter()
            .map(|signed_intent| {
                let intent = signed_intent.intent();
                let private_key = key_for_intent
                    .swap_remove(&intent.transaction_intent_hash())
                    .ok_or_else(|| CommonError::Unknown)?;
                let notary_signature =
                    private_key.notarize_hash(&signed_intent.hash());
                NotarizedTransaction::new(signed_intent, notary_signature)
            })
            .collect::<Result<Vec<_>>>()
    }
}
