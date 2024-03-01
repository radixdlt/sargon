use crate::prelude::*;
use transaction::model::{
    HashHasHrp as ScryptoHashHasHrp,
    NotarizedTransactionV1 as ScryptoNotarizedTransaction,
    SignedIntentHash as ScryptoSignedIntentHash,
    TransactionHashBech32Decoder as ScryptoTransactionHashBech32Decoder,
    TransactionHashBech32Encoder as ScryptoTransactionHashBech32Encoder,
};

use radix_engine_common::crypto::{
    Hash as ScryptoHash, IsHash as ScryptoIsHash,
};

/// A Signed Intent Hash is a bech32 encoded string starting with `"signedintent_"
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, derive_more::Display, uniffi::Record,
)]
#[display("{}", self.bech32_encoded_tx_id)]
pub struct SignedIntentHash {
    /// Which network this intent hash is used on
    pub network_id: NetworkID,
    /// the hash of the intent
    pub hash: Hash,
    /// Bech32 encoded TX id, starting with `signedintent_` e.g.:
    pub bech32_encoded_tx_id: String,
}

impl SignedIntentHash {
    pub fn from_scrypto(
        signed_intent_hash: ScryptoSignedIntentHash,
        network_id: NetworkID,
    ) -> Self {
        let bech32_encoder = ScryptoTransactionHashBech32Encoder::new(
            &network_id.network_definition(),
        );
        let bech32_encoded_tx_id = bech32_encoder
            .encode(&signed_intent_hash)
            .expect("should never fail");
        let scrypto_hash: ScryptoHash = *signed_intent_hash.as_hash();

        Self {
            network_id,
            hash: scrypto_hash.into(),
            bech32_encoded_tx_id,
        }
    }
}

impl FromStr for SignedIntentHash {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate_and_decode_hash::<ScryptoSignedIntentHash>(s)
            .map(|t| Self::from_scrypto(t.0, t.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignedIntentHash;

    // see https://github.com/radixdlt/radixdlt-scrypto/blob/ff21f24952318387803ae720105eec079afe33f3/transaction/src/model/hash/encoder.rs#L65-L79
    #[test]
    fn new_from_network_id_and_hash() {
        /*
            // Arrange
        let encoder = TransactionHashBech32Encoder::for_simulator();
        let transaction = transaction();
        let hash = transaction.prepare().unwrap().intent_hash();

        // Act
        let encoded = encoder.encode(&hash).unwrap();

        // Assert
        assert_eq!(
            encoded,
            "txid_sim1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2svr7ejr"
        )
        */
    }

    #[test]
    fn from_bech32() {
        // let decoder = ScryptoTransactionHashBech32Decoder::new(network)
        // let encoded_hash = "txid_sim1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2svr7ejr";
        // let expected_hash = Hash::from_str(
        //     "60e5617d670e6c8a42ba5f3749f4ff1079f66221f282554ecdda9ad385ecb195",
        // )
        // .unwrap();

        // // Act
        // let decoded = decoder
        //     .validate_and_decode::<IntentHash>(encoded_hash)
        //     .unwrap();
    }
}
