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

impl SubintentStatusResponse {
    pub fn new(
        ledger_state: LedgerState,
        subintent_status: SubintentStatus,
        finalized_at_transaction_intent_hash: Option<String>,
    ) -> Self {
        Self {
            ledger_state,
            subintent_status,
            finalized_at_transaction_intent_hash,
        }
    }
}

#[cfg(test)]
impl SubintentStatusResponse {
    // Helper functions to create sample responses

    pub fn sample_unknown() -> Self {
        Self::new(
            LedgerState::sample_stokenet(),
            SubintentStatus::Unknown,
            None,
        )
    }

    pub fn sample_committed_success() -> Self {
        Self::committed_success("txid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6szm3ltd".to_owned())
    }

    pub fn committed_success(
        finalized_at_transaction_intent_hash: impl Into<Option<String>>,
    ) -> Self {
        Self::new(
            LedgerState::sample_stokenet(),
            SubintentStatus::CommittedSuccess,
            finalized_at_transaction_intent_hash.into(),
        )
    }
}
