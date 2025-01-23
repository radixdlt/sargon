use crate::prelude::*;
use sargon::DappToWalletInteractionBatchOfTransactions as InternalDappToWalletInteractionBatchOfTransactions;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionBatchOfTransactions {
    pub transactions: Vec<UnvalidatedTransactionManifest>,
}
