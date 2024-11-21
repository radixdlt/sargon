use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct TransactionStatusResponse {
    pub ledger_state: LedgerState,
    pub known_payloads: Vec<TransactionStatusResponsePayloadItem>,
    pub error_message: Option<String>,
}

// TODO: Uncomment the following line once we don't need to mock response of GatewayClient::get_pre_authorization_status()
//#[cfg(test)]
impl TransactionStatusResponse {
    // Helper functions to create sample responses

    pub fn sample_empty() -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![],
            ledger_state: LedgerState::sample_stokenet(),
            error_message: None,
        }
    }

    pub fn sample_unknown() -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![
                TransactionStatusResponsePayloadItem::sample_unknown(),
            ],
            ledger_state: LedgerState::sample_stokenet(),
            error_message: None,
        }
    }

    pub fn sample_pending() -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![
                TransactionStatusResponsePayloadItem::sample_pending(),
            ],
            ledger_state: LedgerState::sample_stokenet(),
            error_message: None,
        }
    }

    pub fn sample_commit_pending_outcome_unknown() -> TransactionStatusResponse
    {
        TransactionStatusResponse {
            known_payloads: vec![TransactionStatusResponsePayloadItem::sample_commit_pending_outcome_unknown()],
            ledger_state: LedgerState::sample_stokenet(),
            error_message: None,
        }
    }

    pub fn sample_committed_success() -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![
                TransactionStatusResponsePayloadItem::sample_committed_success(
                ),
            ],
            ledger_state: LedgerState::sample_stokenet(),
            error_message: None,
        }
    }

    pub fn sample_committed_failure(
        error_message: Option<String>,
    ) -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![
                TransactionStatusResponsePayloadItem::sample_committed_failure(
                ),
            ],
            ledger_state: LedgerState::sample_stokenet(),
            error_message,
        }
    }

    pub fn sample_permanently_rejected(
        error_message: Option<String>,
    ) -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![TransactionStatusResponsePayloadItem::sample_committed_permanently_rejected()],
            ledger_state: LedgerState::sample_stokenet(),
            error_message,
        }
    }

    pub fn sample_temporarily_rejected() -> TransactionStatusResponse {
        TransactionStatusResponse {
            known_payloads: vec![TransactionStatusResponsePayloadItem::sample_temporarily_rejected()],
            ledger_state: LedgerState::sample_stokenet(),
            error_message: None,
        }
    }
}
