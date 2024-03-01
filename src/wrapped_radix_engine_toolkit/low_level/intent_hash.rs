use crate::prelude::*;
use transaction::model::{
    HashHasHrp as ScryptoHashHasHrp, IntentHash as ScryptoIntentHash,
    NotarizedTransactionV1 as ScryptoNotarizedTransaction,
    TransactionHashBech32Decoder as ScryptoTransactionHashBech32Decoder,
    TransactionHashBech32Encoder as ScryptoTransactionHashBech32Encoder,
};

use radix_engine_common::crypto::{
    Hash as ScryptoHash, IsHash as ScryptoIsHash,
};

/// A Transaction Intent Hash used to identify transactions always
/// is a bech32 encoded string starting with `txid_` e.g.:
/// `"txid_rdx19rpveua6xuhvz0axu0mwpqk8fywr83atv8mkrugchvw6uuslgppqh9cnj4"`
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, derive_more::Display, uniffi::Record,
)]
#[display("{}", self.bech32_encoded_tx_id)]
pub struct IntentHash {
    /// Which network this intent hash is used on
    pub network_id: NetworkID,
    /// the hash of the intent
    pub hash: Hash,
    /// Bech32 encoded TX id, starting with `txid_` e.g.:
    /// `"txid_rdx19rpveua6xuhvz0axu0mwpqk8fywr83atv8mkrugchvw6uuslgppqh9cnj4"`
    pub bech32_encoded_tx_id: String,
}

impl IntentHash {
    pub(crate) fn from_scrypto(
        intent_hash: ScryptoIntentHash,
        network_id: NetworkID,
    ) -> Self {
        let bech32_encoder = ScryptoTransactionHashBech32Encoder::new(
            &network_id.network_definition(),
        );
        let bech32_encoded_tx_id = bech32_encoder
            .encode(&intent_hash)
            .expect("should never fail");
        let scrypto_hash: ScryptoHash = *intent_hash.as_hash();

        Self {
            network_id,
            hash: scrypto_hash.into(),
            bech32_encoded_tx_id,
        }
    }

    pub fn new(hash: Hash, network_id: NetworkID) -> Self {
        let scrypto_hash: ScryptoHash = hash.clone().into_hash();
        Self::from_scrypto(
            ScryptoIntentHash::from_hash(scrypto_hash),
            network_id,
        )
    }

    pub fn from_bech32(s: &str) -> Result<Self> {
        validate_and_decode_hash::<ScryptoIntentHash>(s)
            .map(|t| Self::from_scrypto(t.0, t.1))
    }
}

impl FromStr for IntentHash {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bech32(s)
    }
}

fn validate_and_decode_hash_try_network<T: ScryptoHashHasHrp>(
    bech32_encoded_hash: &str,
    network_id: NetworkID,
) -> Result<T, ()> {
    ScryptoTransactionHashBech32Decoder::new(&network_id.network_definition())
        .validate_and_decode::<T>(bech32_encoded_hash)
        .map_err(|_| ())
}

pub(crate) fn validate_and_decode_hash<T: ScryptoHashHasHrp>(
    bech32_encoded_hash: &str,
) -> Result<(T, NetworkID)> {
    if let Some(t) = enum_iterator::all::<NetworkID>()
        .map(|n| {
            validate_and_decode_hash_try_network(bech32_encoded_hash, n)
                .map(|v| (v, n))
        })
        .find_map(Result::ok)
    {
        Ok(t)
    } else {
        Err(CommonError::FailedToBech32DecodeTransactionHashAfterHavingTestedAllNetworkID { bad_value: bech32_encoded_hash.to_owned() })
    }
}

impl HasSampleValues for IntentHash {
    fn sample() -> Self {
        todo!()
    }

    fn sample_other() -> Self {
        let intent = TransactionIntent::sample_other();
        intent.intent_hash().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IntentHash;

    #[test]
    fn to_string() {
        assert_eq!(SUT::sample_other().to_string(), "txid_sim1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2svr7ejr");
    }

    #[test]
    fn parse() {
        assert_eq!("txid_sim1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2svr7ejr".parse::<SUT>().unwrap(), SUT::sample_other());
    }

    #[test]
    fn from_hash() {
        let hash: Hash =
            "60e5617d670e6c8a42ba5f3749f4ff1079f66221f282554ecdda9ad385ecb195"
                .parse()
                .unwrap();
        assert_eq!(SUT::new(hash, NetworkID::Simulator), SUT::sample_other())
    }
}
