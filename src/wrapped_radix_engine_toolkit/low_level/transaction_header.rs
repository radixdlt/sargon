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
        start_epoch_inclusive: impl Into<Epoch>,
        end_epoch_exclusive: impl Into<Epoch>,
        nonce: impl Into<Nonce>,
        notary_public_key: impl Into<PublicKey>,
        notary_is_signatory: bool,
        tip_percentage: u16,
    ) -> Self {
        let start_epoch_inclusive = start_epoch_inclusive.into();
        let end_epoch_exclusive = end_epoch_exclusive.into();
        assert!(
            end_epoch_exclusive >= start_epoch_inclusive,
            "End epoch MUST be greater than or equal start epoch."
        );
        Self {
            network_id,
            start_epoch_inclusive,
            end_epoch_exclusive,
            nonce: nonce.into(),
            notary_public_key: notary_public_key.into(),
            notary_is_signatory,
            tip_percentage,
        }
    }
}

impl HasSampleValues for TransactionHeader {
    fn sample() -> Self {
        Self::new(
            NetworkID::Mainnet,
            76935,
            76945,
            2371337,
            Ed25519PublicKey::sample(),
            true,
            0,
        )
    }

    fn sample_other() -> Self {
        Self::new(
            NetworkID::Mainnet,
            237,
            237,
            421337237,
            Ed25519PublicKey::sample_other(),
            false,
            10,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionHeader;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    #[should_panic(
        expected = "End epoch MUST be greater than or equal start epoch."
    )]
    fn panics_if_end_epoch_is_smaller_than_start() {
        _ = SUT::new(
            NetworkID::Mainnet,
            237,
            236,
            421337237,
            Ed25519PublicKey::sample_other(),
            false,
            10,
        )
    }
}
