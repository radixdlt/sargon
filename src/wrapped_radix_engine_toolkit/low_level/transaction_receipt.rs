use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct TransactionReceipt {
    pub(crate) decoded: ScryptoTransactionReceipt,
}

impl TryFrom<BagOfBytes> for TransactionReceipt {
    type Error = crate::CommonError;

    fn try_from(encoded: BagOfBytes) -> Result<Self, Self::Error> {
        Scrypto_scrypto_decode::<ScryptoVersionedTransactionReceipt>(&encoded)
            .map(|r| r.into_latest())
            .map_err(|e| {
                error!("Failed to decode encoded receipt, {:?}", e);
                CommonError::FailedToDecodeEncodedReceipt
            })
            .map(|decoded| Self { decoded })
    }
}
