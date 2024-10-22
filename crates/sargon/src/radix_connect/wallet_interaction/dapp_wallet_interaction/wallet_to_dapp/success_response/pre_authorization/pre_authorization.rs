use crate::prelude::*;
use radix_transactions::model::TransactionPayload;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionPreAuthorizationResponseItems {
    /// A hex encoded signed partial transaction.
    #[serde(rename = "signedPartialTransaction")]
    pub encoded_signed_partial_transaction: String,
}

impl WalletToDappInteractionPreAuthorizationResponseItems {
    pub fn new(
        signed_partial_transaction: ScryptoSignedPartialTransaction,
    ) -> Result<Self> {
        let bytes = signed_partial_transaction
            .to_raw()
            .map_err(|e| match e {
                sbor::EncodeError::MaxDepthExceeded(max) => {
                    CommonError::InvalidTransactionMaxSBORDepthExceeded {
                        max: max as u16,
                    }
                }
                _ => {
                    CommonError::InvalidSignedPartialTransactionFailedToEncode {
                        underlying: format!("{:?}", e),
                    }
                }
            })?
            .to_vec();
        let encoded_signed_partial_transaction = hex_encode(&bytes);
        Ok(Self {
            encoded_signed_partial_transaction,
        })
    }
}

impl HasSampleValues for WalletToDappInteractionPreAuthorizationResponseItems {
    fn sample() -> Self {
        Self {
            encoded_signed_partial_transaction:
                "replace_actual_encoded_string_here".to_owned(),
        }
    }

    fn sample_other() -> Self {
        Self {
            encoded_signed_partial_transaction:
                "replace_other_encoded_string_here".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionPreAuthorizationResponseItems;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
