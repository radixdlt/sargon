use crate::prelude::*;

use radix_engine::transaction::TransactionReceiptV1 as ScryptoTransactionReceipt;
use radix_engine::transaction::VersionedTransactionReceipt as ScryptoVersionedTransactionReceipt;
use radix_engine_common::data::scrypto::{scrypto_decode, scrypto_encode};
use sbor::HasLatestVersion;

#[derive(Clone, Debug)]
pub struct TransactionReceipt {
    pub(crate) decoded: ScryptoTransactionReceipt,
    pub(crate) encoded: BagOfBytes,
}
impl PartialEq for TransactionReceipt {
    fn eq(&self, other: &Self) -> bool {
        self.encoded == other.encoded
    }
}
impl Eq for TransactionReceipt {}

impl TryFrom<BagOfBytes> for TransactionReceipt {
    type Error = crate::CommonError;

    fn try_from(encoded: BagOfBytes) -> Result<Self, Self::Error> {
        scrypto_decode::<ScryptoVersionedTransactionReceipt>(&encoded)
            .map(|r| r.into_latest())
            .map_err(|e| {
                error!("Failed to decode encoded receipt, {:?}", e);
                CommonError::FailedToDecodeEncodedReceipt
            })
            .map(|decoded| Self { decoded, encoded })
    }
}
