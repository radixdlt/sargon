use crate::prelude::*;

#[cfg(test)]
impl SubintentStatusResponse {
    // Helper functions to create sample responses

    pub fn sample_unknown() -> Self {
        Self {
            ledger_state: LedgerState::sample_stokenet(),
            subintent_status: SubintentStatus::Unknown,
            finalized_at_transaction_intent_hash: None,
        }
    }

    pub fn sample_committed_success() -> Self {
        Self::committed_success(Some(
            TransactionIntentHash::sample().to_string(),
        ))
    }

    pub fn sample_committed_success_corrupt() -> Self {
        Self::committed_success(None)
    }

    fn committed_success(
        finalized_at_transaction_intent_hash: Option<String>,
    ) -> Self {
        Self {
            ledger_state: LedgerState::sample_stokenet(),
            subintent_status: SubintentStatus::CommittedSuccess,
            finalized_at_transaction_intent_hash,
        }
    }
}
