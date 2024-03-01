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
