use crate::prelude::*;

use radix_engine::transaction::TransactionReceipt as ScryptoTransactionReceipt;
use radix_engine_common::data::scrypto::scrypto_decode;

#[derive(Clone, Debug)]
pub struct TransactionReceipt(pub(crate) ScryptoTransactionReceipt);
impl TryFrom<Vec<u8>> for TransactionReceipt {
    type Error = crate::CommonError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        scrypto_decode(&value).map_err(|e| {
            error!("Failed to decode encoded Transaction Receipt (bytes) into a (Scrypto)TransactionReceipt, error: {:?}", e);
            CommonError::FailedToDecodeEncodedReceipt
        }).map(Self)
    }
}
impl TryFrom<BagOfBytes> for TransactionReceipt {
    type Error = crate::CommonError;

    fn try_from(value: BagOfBytes) -> Result<Self, Self::Error> {
        Self::try_from(value.to_vec())
    }
}
