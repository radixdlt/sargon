use crate::prelude::*;

decl_tx_hash!(
    /// `IntentHash` used to identify transactions.
    /// Representation is bech32 encoded string starting with `txid_` e.g.:
    /// `"txid_rdx19rpveua6xuhvz0axu0mwpqk8fywr83atv8mkrugchvw6uuslgppqh9cnj4"`
    Intent,
    "txid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6szm3ltd",
    "txid...zm3ltd",
);

#[uniffi::export]
pub fn new_intent_hash_sample() -> IntentHash {
    InternalTxHash::sample().into()
}

#[uniffi::export]
pub fn new_intent_hash_sample_other() -> IntentHash {
    InternalTxHash::sample_other().into()
}

