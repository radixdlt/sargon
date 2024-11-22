use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct SubintentStatusResponse {
    /// The ledger state against which the response was generated. Can be used to detect if the Network Gateway is returning up-to-date information.
    pub ledger_state: LedgerState,

    /// The finalization status of this subintent.
    /// Each subintent can only be successfully committed once, but unlike a transaction intent,
    /// could be committed as a failure zero or more times first.
    pub subintent_status: SubintentStatus,

    /// The Transaction ID in which the subintent was included.
    /// This field is only present if the status is `CommittedSuccess`
    pub finalized_at_transaction_intent_hash: Option<String>,
}
