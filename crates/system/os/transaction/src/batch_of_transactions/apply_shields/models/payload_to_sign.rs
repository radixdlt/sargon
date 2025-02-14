use crate::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ApplySecurityShieldPayloadToSign {
    pub applications_with_intents:
        Vec<SecurityShieldApplicationWithTransactionIntents>,
    pub notary_keys: IndexMap<TransactionIntentHash, Ed25519PrivateKey>,
}
