use crate::prelude::*;

#[derive(Deserialize, Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct TransactionConstructionResponse {
    pub ledger_state: LedgerState,
}
