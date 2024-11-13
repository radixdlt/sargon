use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct GatewayStatusResponse {
    pub ledger_state: LedgerState,
}
