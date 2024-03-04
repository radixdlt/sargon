use crate::prelude::*;

use transaction::model::{
    HashHasHrp as ScryptoHashHasHrp, IntentHash as ScryptoIntentHash,
    TransactionHashBech32Decoder as ScryptoTransactionHashBech32Decoder,
};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_network() {
        // valid bech32 encoded string, unknown network (id: 0xfa, hrp: "fake")
        let s = "txid_fake_1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sfken4s";
        assert_eq!(
            validate_and_decode_hash::<transaction::model::IntentHash>(s),
            Err(CommonError::FailedToBech32DecodeTransactionHashAfterHavingTestedAllNetworkID { bad_value: s.to_owned() })
        );
    }

    #[test]
    fn decode_sim_success() {
        let s = "txid_sim1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2svr7ejr";
        assert_eq!(
            validate_and_decode_hash::<transaction::model::IntentHash>(s)
                .unwrap()
                .1,
            NetworkID::Simulator.into()
        );
    }
}
