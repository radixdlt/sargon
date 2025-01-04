use crate::prelude::*;

pub use radix_engine_toolkit::functions::transaction_v1::signed_intent::to_payload_bytes as RET_signed_intent_compile;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedIntent {
    pub intent: TransactionIntent,
    pub intent_signatures: IntentSignatures,
}

impl IntoIterator for SignedIntent {
    type Item = SignatureWithPublicKey;
    type IntoIter = <Vec<SignatureWithPublicKey> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.intent_signatures
            .signatures
            .into_iter()
            .map(|s| s.0)
            .collect_vec()
            .into_iter()
    }
}

impl SignedIntent {
    pub fn new(
        intent: TransactionIntent,
        intent_signatures: IntentSignatures,
    ) -> Result<Self> {
        if !intent_signatures.validate(intent.transaction_intent_hash()) {
            return Err(CommonError::InvalidSignaturesForIntentSomeDidNotValidateIntentHash);
        }

        // Verify that this SignedIntent has acceptable depth and is compatible
        _ = compile_signed_intent_with(&intent, &intent_signatures)?;

        Ok(Self {
            intent,
            intent_signatures,
        })
    }

    pub fn with_signatures(
        intent: TransactionIntent,
        signatures: impl IntoIterator<Item = IntentSignature>,
    ) -> Result<Self> {
        Self::new(intent, IntentSignatures::new(signatures))
    }

    pub fn intent(&self) -> &TransactionIntent {
        &self.intent
    }
}

impl From<SignedIntent> for ScryptoSignedIntent {
    fn from(value: SignedIntent) -> Self {
        into_scrypto(&value.intent, &value.intent_signatures)
    }
}

fn into_scrypto(
    intent: &TransactionIntent,
    intent_signatures: &IntentSignatures,
) -> ScryptoSignedIntent {
    ScryptoSignedIntent {
        intent: intent.clone().into(),
        intent_signatures: intent_signatures.clone().into(),
    }
}

fn compile_signed_intent_with(
    intent: &TransactionIntent,
    intent_signatures: &IntentSignatures,
) -> Result<BagOfBytes> {
    compile_signed_intent(into_scrypto(intent, intent_signatures))
}

fn compile_signed_intent(
    scrypto_signed_intent: ScryptoSignedIntent,
) -> Result<BagOfBytes> {
    RET_signed_intent_compile(&scrypto_signed_intent)
        .map_err(|e| match e {
            sbor::EncodeError::MaxDepthExceeded(max) => {
                CommonError::InvalidTransactionMaxSBORDepthExceeded {
                    max: max as u16,
                }
            }
            _ => CommonError::InvalidSignedIntentFailedToEncode {
                underlying: format!("{:?}", e),
            },
        })
        .map(BagOfBytes::from)
}

impl SignedIntent {
    pub fn network_id(&self) -> NetworkID {
        self.intent.network_id()
    }

    pub fn hash(&self) -> SignedTransactionIntentHash {
        let scrypto_signed_intent: ScryptoSignedIntent = self.clone().into();
        let hash = RET_signed_intent_hash(&scrypto_signed_intent).expect("Sargon should only produce valid SignedIntent, should never fail to produce signed intent hash using RET.");
        let scrypto_transaction_signed_intent_hash =
            ScryptoSignedTransactionIntentHash(hash.hash);
        SignedTransactionIntentHash::from_scrypto(
            scrypto_transaction_signed_intent_hash,
            self.network_id(),
        )
    }
}

impl TryFrom<ScryptoSignedIntent> for SignedIntent {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoSignedIntent) -> Result<Self, Self::Error> {
        let intent: TransactionIntent = value.intent.try_into()?;
        let intent_signatures: IntentSignatures = (
            value.intent_signatures,
            intent.transaction_intent_hash().hash,
        )
            .try_into()?;
        Ok(Self {
            intent,
            intent_signatures,
        })
    }
}

impl HasSampleValues for SignedIntent {
    fn sample() -> Self {
        let intent = TransactionIntent::sample_other();

        let mut signatures = Vec::<IntentSignature>::new();
        for n in 1..4 {
            let private_key: Secp256k1PrivateKey =
                ScryptoSecp256k1PrivateKey::from_u64(n).unwrap().into();

            let intent_signature = private_key.sign_transaction_intent_hash(
                &intent.transaction_intent_hash(),
            );
            signatures.push(intent_signature)
        }

        let intent_signatures = IntentSignatures::new(signatures);

        Self::new(intent, intent_signatures).unwrap()
    }

    fn sample_other() -> Self {
        Self::new(TransactionIntent::sample(), IntentSignatures::default())
            .unwrap()
    }
}

#[cfg(test)]
impl SignedIntent {
    /// Utility function which uses `SignedIntent::new(<TransactionIntent>, <IntentSignatures>)`
    /// and SHOULD return `Err` if `depth > SignedIntent::MAX_SBOR_DEPTH`, which
    /// we can assert in unit tests.
    pub(crate) fn test_with_sbor_depth(
        depth: usize,
        network_id: NetworkID,
    ) -> Result<Self> {
        TransactionIntent::test_with_sbor_depth(depth, network_id).and_then(
            |intent| {
                let mut signatures = Vec::<IntentSignature>::new();
                for n in 1..4 {
                    let private_key: Secp256k1PrivateKey =
                        ScryptoSecp256k1PrivateKey::from_u64(n).unwrap().into();

                    let intent_signature = private_key
                        .sign_transaction_intent_hash(
                            &intent.transaction_intent_hash(),
                        );
                    signatures.push(intent_signature)
                }

                let intent_signatures = IntentSignatures::new(signatures);

                Self::new(intent, intent_signatures)
            },
        )
    }

    pub(crate) const MAX_SBOR_DEPTH: usize =
        TransactionIntent::MAX_SBOR_DEPTH - 1;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignedIntent;

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
    fn hash() {
        assert_eq!(SUT::sample().hash().to_string(), "signedintent_sim1ul0kjuvd63sslhxy869zdk4k3vcdg9e9244xwpuck4dyndzx9wnqrhxy5d");
        assert_eq!(
            hex_encode(SUT::sample().hash().hash),
            "e7df69718dd4610fdcc43e8a26dab68b30d41725556a670798b55a49b4462ba6"
        );
    }

    #[test]
    fn many_intent_signatures_all_valid() {
        let intent = TransactionIntent::sample_other();

        let mut signatures = Vec::<IntentSignature>::new();
        for n in 1..4 {
            let private_key: Secp256k1PrivateKey =
                ScryptoSecp256k1PrivateKey::from_u64(n).unwrap().into();

            let intent_signature = private_key.sign_transaction_intent_hash(
                &intent.transaction_intent_hash(),
            );
            signatures.push(intent_signature)
        }

        let intent_signatures = IntentSignatures::new(signatures);
        assert_eq!(intent_signatures.clone().signatures.into_iter().map(|s| s.signature().to_string()).collect_vec(), ["01da59c65684d07f1997bf9615c1e9330a54d8f3b13d8caaef1a8b32f64259be05544dc9290b64294a174c2857dd1043b3a5c0ca50bfc4ff35a95dd4338edee80b", "01427f6b48420da77ecb31c62b693d1970fb6cd3bcf68ea4ae21ae6c4e4521eff80100fed2410cba034a46cc5c546c9470cce1b44ff1c2d2e31c7ded420aa84024", "00570e538b1f84b323ea21b87930debed81d46a1a1abec5007c72106c4985ab515501af9a4ebbbfb75570416e0cc52dc93e064478c254fafb5065159e40b606612"]);

        assert_eq!(
            SUT::new(intent, intent_signatures.clone())
                .unwrap()
                .intent_signatures,
            intent_signatures
        );
    }

    #[test]
    fn many_intent_signatures_one_invalid() {
        let intent = TransactionIntent::sample_other();

        let mut signatures = Vec::<IntentSignature>::new();
        for n in 1..4 {
            let private_key: Secp256k1PrivateKey =
                ScryptoSecp256k1PrivateKey::from_u64(n).unwrap().into();

            let intent_signature = private_key.sign_transaction_intent_hash(
                &intent.transaction_intent_hash(),
            );
            signatures.push(intent_signature)
        }

        signatures.push(IntentSignature::sample());

        assert_eq!(
            SUT::with_signatures(intent, signatures),
            Err(CommonError::InvalidSignaturesForIntentSomeDidNotValidateIntentHash)
        );
    }

    #[test]
    fn many_intent_signatures_invalid_because_mismatching_intent() {
        let intent = TransactionIntent::sample_other();

        let mut signatures = Vec::<IntentSignature>::new();
        for n in 1..4 {
            let private_key: Secp256k1PrivateKey =
                ScryptoSecp256k1PrivateKey::from_u64(n).unwrap().into();
            let hash = intent.transaction_intent_hash();
            let intent_signature =
                private_key.sign_transaction_intent_hash(&hash);
            signatures.push(intent_signature)
        }

        assert_eq!(
            SUT::with_signatures(
                TransactionIntent::sample(), // <-- WRONG Intent, not was signed.
                signatures
            ),
            Err(CommonError::InvalidSignaturesForIntentSomeDidNotValidateIntentHash)
        );
    }

    #[test]
    fn signed_intent_with_max_sbor_depth_is_ok() {
        assert!(SUT::test_with_sbor_depth(
            SUT::MAX_SBOR_DEPTH,
            NetworkID::Stokenet
        )
        .is_ok());
    }

    #[test]
    fn signed_intent_with_sbor_depth_greater_than_max_is_err() {
        assert_eq!(
            SUT::test_with_sbor_depth(
                SUT::MAX_SBOR_DEPTH + 1,
                NetworkID::Stokenet
            ),
            Err(CommonError::InvalidTransactionMaxSBORDepthExceeded {
                max: 24_u16
            })
        );
    }

    #[test]
    fn other_reasons_for_invalid() {
        let res = compile_signed_intent(invalid_signed_intent());
        assert_eq!(
            res,
            Err(CommonError::InvalidSignedIntentFailedToEncode { underlying: "MismatchingArrayElementValueKind { element_value_kind: 7, actual_value_kind: 8 }".to_owned() }) 
        );
    }
}
