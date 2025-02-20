use core_misc::decl_bool_type;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[display("{} intent_discriminator: {}", network_id, intent_discriminator)]
pub struct TransactionHeader {
    pub network_id: NetworkID,
    pub start_epoch_inclusive: Epoch,
    pub end_epoch_exclusive: Epoch,
    pub intent_discriminator: IntentDiscriminator32,
    pub notary_public_key: PublicKey,
    pub notary_is_signatory: NotaryIsSignatory,
    pub tip_percentage: u16,
}

decl_bool_type!(NotaryIsSignatory, true);

impl TransactionHeader {
    /// Creates a new `TransactionHeader`
    ///
    /// # Panics
    /// Panics if `end_epoch_exclusive < start_epoch_inclusive`
    pub fn new(
        network_id: NetworkID,
        start_epoch_inclusive: impl Into<Epoch>,
        end_epoch_exclusive: impl Into<Epoch>,
        intent_discriminator: impl Into<IntentDiscriminator32>,
        notary_public_key: impl Into<PublicKey>,
        notary_is_signatory: impl Into<NotaryIsSignatory>,
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
            intent_discriminator: intent_discriminator.into(),
            notary_public_key: notary_public_key.into(),
            notary_is_signatory: notary_is_signatory.into(),
            tip_percentage,
        }
    }
}

impl From<TransactionHeader> for ScryptoTransactionHeader {
    fn from(value: TransactionHeader) -> Self {
        Self {
            network_id: value.network_id.into(),
            start_epoch_inclusive: value.start_epoch_inclusive.into(),
            end_epoch_exclusive: value.end_epoch_exclusive.into(),
            nonce: value.intent_discriminator.into(),
            notary_public_key: value.notary_public_key.into(),
            notary_is_signatory: value.notary_is_signatory.0,
            tip_percentage: value.tip_percentage,
        }
    }
}

impl TryFrom<ScryptoTransactionHeader> for TransactionHeader {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoTransactionHeader) -> Result<Self, Self::Error> {
        let network_id: NetworkID = value.network_id.try_into()?;
        let notary_public_key: PublicKey =
            value.notary_public_key.try_into()?;
        Ok(Self {
            network_id,
            start_epoch_inclusive: value.start_epoch_inclusive.into(),
            end_epoch_exclusive: value.end_epoch_exclusive.into(),
            intent_discriminator: value.nonce.into(),
            notary_public_key,
            notary_is_signatory: NotaryIsSignatory(value.notary_is_signatory),
            tip_percentage: value.tip_percentage,
        })
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

    // The Header of:
    // https://github.com/radixdlt/radixdlt-scrypto/blob/ff21f24952318387803ae720105eec079afe33f3/transaction/src/model/hash/encoder.rs#L115
    fn sample_other() -> Self {
        let private_key: Secp256k1PrivateKey =
            ScryptoSecp256k1PrivateKey::from_u64(1).unwrap().into();
        let public_key: Secp256k1PublicKey = private_key.public_key();
        let network_id = NetworkID::Simulator;
        Self::new(network_id, 0, 10, 10, public_key, true, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn to_from_scrypto() {
        let roundtrip =
            |s: SUT| SUT::try_from(ScryptoTransactionHeader::from(s)).unwrap();
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
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
