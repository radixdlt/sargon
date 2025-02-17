use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
