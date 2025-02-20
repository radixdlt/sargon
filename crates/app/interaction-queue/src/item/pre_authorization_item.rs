use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
/// A Pre-Authorization item that is part of the interaction queue.
pub struct PreAuthorizationQueueItem {
    /// The identifier of the subintent.
    pub subintent_id: SubintentHash,

    /// The identifier of the Transaction in which this PreAuthorization was committed.
    /// Only available when status is success.
    pub transaction_id: Option<TransactionIntentHash>,

    /// The name of the DApp that requested this PreAuthorization.
    pub dapp_name: String,

    /// The timestamp at which the subintent expires
    pub expiration_timestamp: Instant,
    // TODO: Define if we need the interaction_id here
    // It is currently used when dismissing the `PollPreAuthorizationStatus` on iOS, to identify the interaction that must be dismissed.
    //pub interaction_id: WalletInteractionId,
}

impl PreAuthorizationQueueItem {
    pub fn new(
        subintent_id: SubintentHash,
        transaction_id: Option<TransactionIntentHash>,
        dapp_name: String,
        expiration_timestamp: Instant,
    ) -> Self {
        Self {
            subintent_id,
            transaction_id,
            dapp_name,
            expiration_timestamp,
        }
    }
}

impl HasSampleValues for PreAuthorizationQueueItem {
    fn sample() -> Self {
        Self::new(
            SubintentHash::sample(),
            None,
            "DApp Name".to_string(),
            Instant::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            SubintentHash::sample_other(),
            Some(TransactionIntentHash::sample_other()),
            "Other DApp Name".to_string(),
            Instant::sample_other(),
        )
    }
}
