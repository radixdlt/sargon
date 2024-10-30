use crate::prelude::*;
use radix_transactions::model::TransactionPayload;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

    pub fn new_with_subintent_and_signatures(
        subintent: Subintent,
        signatures: Vec<IntentSignature>,
    ) -> Result<Self> {
        Self::new(build_signed_partial_transaction(subintent, signatures))
    }
}

impl HasSampleValues for WalletToDappInteractionPreAuthorizationResponseItems {
    fn sample() -> Self {
        Self {
            encoded_signed_partial_transaction:
                "4d220e03210221012105210607010a872c0100000000000a912c01000000000022010105008306670000000022010105e8860667000000000a15cd5b070000000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f205261646978212020002022054103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c086c6f636b5f6665652101850000fda0c4277708000000000000000000000000000000004103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a480000000000000000000000000000004103800051ac224ee242c339b5ea5f1ae567f0520a6ffa24b52a10b8e6cd96a8347f0c147472795f6465706f7369745f6f725f61626f727421028100000000220000600121002021002022010102200720c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b362101200740fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103202000".to_owned(),
        }
    }

    fn sample_other() -> Self {
        Self {
            encoded_signed_partial_transaction:
                "4d220e03210221012105210607f20a00000000000000000a0a000000000000002200002200000ab168de3a00000000202000220000202000202200202100202201000121012007410001598e989470d125dafac276b95bb1ba21e2ee8e0beb0547599335f83b48a0a830cd6a956a54421039cef5fb7e492ebaa315f751a2dd5b74bd9cebbda997ec12202000".to_owned(),
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

    #[test]
    fn new() {
        let subintent = Subintent::sample();
        let signatures = vec![IntentSignature::sample()];
        let sut = SUT::new_with_subintent_and_signatures(subintent, signatures)
            .unwrap();
        assert_eq!(sut, SUT::sample());
    }
}
