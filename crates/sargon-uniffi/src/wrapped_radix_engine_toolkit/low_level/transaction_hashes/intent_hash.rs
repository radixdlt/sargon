use crate::prelude::*;

decl_tx_hash!(
    /// `IntentHash` used to identify transactions.
    /// Representation is bech32 encoded string starting with `txid_` e.g.:
    /// `"txid_rdx19rpveua6xuhvz0axu0mwpqk8fywr83atv8mkrugchvw6uuslgppqh9cnj4"`
    TransactionIntentHash,
);

#[uniffi::export]
pub fn new_transaction_intent_hash_sample() -> TransactionIntentHash {
    InternalTransactionIntentHash::sample().into()
}

#[uniffi::export]
pub fn new_transaction_intent_hash_sample_other() -> TransactionIntentHash {
    InternalTransactionIntentHash::sample_other().into()
}
