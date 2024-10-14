use crate::prelude::*;

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
