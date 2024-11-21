use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct TransactionStatusResponse {
    pub ledger_state: LedgerState,
    pub known_payloads: Vec<TransactionStatusResponsePayloadItem>,
    pub error_message: Option<String>,
}
