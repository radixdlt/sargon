use crate::prelude::*;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, derive_more::Display, uniffi::Record,
)]
#[display("{} nonce: {}", network_id, nonce)]
pub struct TransactionHeader {
    pub network_id: NetworkID,
    pub start_epoch_inclusive: Epoch,
    pub end_epoch_exclusive: Epoch,
    pub nonce: Nonce,
    pub notary_public_key: PublicKey,
    pub notary_is_signatory: bool,
    pub tip_percentage: u16,
}

impl TransactionHeader {
    /// Creates a new `TransactionHeader`
    ///
    /// # Panics
    /// Panics if `end_epoch_exclusive < start_epoch_inclusive`
    pub fn new(
        network_id: NetworkID,
        start_epoch_inclusive: Epoch,
        end_epoch_exclusive: Epoch,
        nonce: Nonce,
        notary_public_key: PublicKey,
        notary_is_signatory: bool,
        tip_percentage: u16,
    ) -> Self {
        assert!(
            end_epoch_exclusive >= start_epoch_inclusive,
            "End epoch MUST be greater than or equal start epoch."
        );
        Self {
            network_id,
            start_epoch_inclusive,
            end_epoch_exclusive,
            nonce,
            notary_public_key,
            notary_is_signatory,
            tip_percentage,
        }
    }
}
