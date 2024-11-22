use crate::prelude::*;

impl TransactionStatusResponse {
    pub fn new(
        known_payloads: impl IntoIterator<
            Item = TransactionStatusResponsePayloadItem,
        >,
        ledger_state: LedgerState,
        error_message: Option<String>,
    ) -> Self {
        Self {
            known_payloads: Vec::from_iter(known_payloads),
            ledger_state,
            error_message,
        }
    }
}

#[cfg(test)]
impl TransactionStatusResponse {
    // Helper functions to create sample responses

    pub fn sample_empty() -> Self {
        Self::success([])
    }

    pub fn sample_unknown() -> Self {
        Self::success([TransactionStatusResponsePayloadItem::sample_unknown()])
    }

    pub fn sample_pending() -> Self {
        Self::success([TransactionStatusResponsePayloadItem::sample_pending()])
    }

    pub fn sample_commit_pending_outcome_unknown() -> Self {
        Self::success([TransactionStatusResponsePayloadItem::sample_commit_pending_outcome_unknown()])
    }

    pub fn sample_committed_success() -> Self {
        Self::success([
            TransactionStatusResponsePayloadItem::sample_committed_success(),
        ])
    }

    pub fn sample_committed_failure(error_message: Option<String>) -> Self {
        Self::failure(
            [
                TransactionStatusResponsePayloadItem::sample_committed_failure(
                ),
            ],
            error_message,
        )
    }

    pub fn sample_permanently_rejected(error_message: Option<String>) -> Self {
        Self::failure([TransactionStatusResponsePayloadItem::sample_committed_permanently_rejected()], error_message)
    }

    pub fn sample_temporarily_rejected() -> Self {
        Self::failure([TransactionStatusResponsePayloadItem::sample_temporarily_rejected()], None)
    }

    fn success(
        known_payloads: impl IntoIterator<
            Item = TransactionStatusResponsePayloadItem,
        >,
    ) -> Self {
        Self::new(known_payloads, LedgerState::sample_stokenet(), None)
    }

    fn failure(
        known_payloads: impl IntoIterator<
            Item = TransactionStatusResponsePayloadItem,
        >,
        error_message: Option<String>,
    ) -> Self {
        Self::new(
            known_payloads,
            LedgerState::sample_stokenet(),
            error_message,
        )
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
