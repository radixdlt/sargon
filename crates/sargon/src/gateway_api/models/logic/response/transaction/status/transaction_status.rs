use crate::prelude::*;

#[cfg(test)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionStatusResponse;

    #[test]
    fn json_test() {
        let pending = fixture::<SUT>(include_str!(concat!(
            env!("FIXTURES_MODELS_GW"),
            "transaction/response_status__pending.json"
        )))
        .unwrap();
        assert_eq!(
            pending.known_payloads.first().unwrap().payload_status,
            Some(TransactionStatusResponsePayloadStatus::Pending)
        );

        let committed_success = fixture::<SUT>(include_str!(concat!(
            env!("FIXTURES_MODELS_GW"),
            "transaction/response_status__committed_success.json"
        )))
        .unwrap();
        assert_eq!(
            committed_success
                .known_payloads
                .first()
                .unwrap()
                .payload_status,
            Some(TransactionStatusResponsePayloadStatus::CommittedSuccess)
        );
    }
}
