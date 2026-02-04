use crate::prelude::*;
use sargon::DappToWalletInteractionSubintentHeader as InternalDappToWalletInteractionSubintentHeader;

#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct DappToWalletInteractionSubintentHeader {
    pub network_id: u8,
    pub start_epoch_inclusive: u64,
    pub end_epoch_exclusive: u64,
    pub min_proposer_timestamp_inclusive: Option<i64>,
    pub max_proposer_timestamp_exclusive: Option<i64>,
    pub intent_discriminator: u64,
}
