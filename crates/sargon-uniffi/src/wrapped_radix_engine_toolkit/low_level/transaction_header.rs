use crate::prelude::*;
use sargon::TransactionHeader as InternalTransactionHeader;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct TransactionHeader {
    pub network_id: NetworkID,
    pub start_epoch_inclusive: Epoch,
    pub end_epoch_exclusive: Epoch,
    pub nonce: Nonce,
    pub notary_public_key: PublicKey,
    pub notary_is_signatory: bool,
    pub tip_percentage: u16,
}

impl From<InternalTransactionHeader> for TransactionHeader {
    fn from(value: InternalTransactionHeader) -> Self {
        Self {
            network_id: value.network_id.into(),
            start_epoch_inclusive: value.start_epoch_inclusive.into(),
            end_epoch_exclusive: value.end_epoch_exclusive.into(),
            nonce: value.nonce.into(),
            notary_public_key: value.notary_public_key.into(),
            notary_is_signatory: value.notary_is_signatory,
            tip_percentage: value.tip_percentage,
        }
    }
}

impl Into<InternalTransactionHeader> for TransactionHeader {
    fn into(self) -> InternalTransactionHeader {
        InternalTransactionHeader {
            network_id: self.network_id.into(),
            start_epoch_inclusive: self.start_epoch_inclusive.into(),
            end_epoch_exclusive: self.end_epoch_exclusive.into(),
            nonce: self.nonce.into(),
            notary_public_key: self.notary_public_key.into(),
            notary_is_signatory: self.notary_is_signatory,
            tip_percentage: self.tip_percentage,
        }
    }
}

#[uniffi::export]
pub fn new_transaction_header_sample() -> TransactionHeader {
    InternalTransactionHeader::sample().into()
}

#[uniffi::export]
pub fn new_transaction_header_sample_other() -> TransactionHeader {
    InternalTransactionHeader::sample_other().into()
}
