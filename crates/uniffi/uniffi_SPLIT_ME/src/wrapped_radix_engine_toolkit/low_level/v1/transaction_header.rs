use crate::prelude::*;
use sargon::TransactionHeader as InternalTransactionHeader;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct TransactionHeader {
    pub network_id: NetworkID,
    pub start_epoch_inclusive: Epoch,
    pub end_epoch_exclusive: Epoch,
    pub intent_discriminator: IntentDisciminator32,
    pub notary_public_key: PublicKey,
    pub notary_is_signatory: bool,
    pub tip_percentage: u16,
}

#[uniffi::export]
pub fn new_transaction_header_sample() -> TransactionHeader {
    InternalTransactionHeader::sample().into()
}

#[uniffi::export]
pub fn new_transaction_header_sample_other() -> TransactionHeader {
    InternalTransactionHeader::sample_other().into()
}
