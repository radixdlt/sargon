use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionStatusRequest {
    /// Bech32m-encoded hash.
    pub intent_hash: String,
}

impl TransactionStatusRequest {
    pub fn new(intent_hash: String) -> Self {
        Self { intent_hash }
    }
}
