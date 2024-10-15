use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct TransactionStatusResponsePayloadItem {
    pub payload_status: Option<TransactionStatusResponsePayloadStatus>,
}
