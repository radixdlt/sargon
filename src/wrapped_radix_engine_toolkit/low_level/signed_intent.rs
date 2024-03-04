use crate::prelude::*;

use radix_engine_toolkit::functions::signed_intent::hash as RET_signed_intent_hash;
use transaction::model::{
    IntentSignatureV1 as ScryptoIntentSignature,
    IntentSignaturesV1 as ScryptoIntentSignatures,
    SignatureWithPublicKeyV1 as ScryptoSignatureWithPublicKey,
    SignedIntentHash as ScryptoSignedIntentHash,
    SignedIntentV1 as ScryptoSignedIntent,
};

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct SignedIntent {
    pub intent: TransactionIntent,
    pub intent_signatures: IntentSignatures,
}

impl SignedIntent {
    pub fn new_validating_signatures(
        intent: TransactionIntent,
        intent_signatures: IntentSignatures,
    ) -> Result<Self> {
        if !intent_signatures.validate(intent.intent_hash()) {
            return Err(CommonError::InvalidSignaturesForIntentSomeDidNotValidateIntentHash);
        }

        Ok(Self {
            intent,
            intent_signatures,
        })
    }
}

impl SignedIntent {
    pub fn network_id(&self) -> NetworkID {
        self.intent.network_id()
    }

    pub fn hash(&self) -> SignedIntentHash {
        let scrypto_signed_intent: ScryptoSignedIntent = self.clone().into();
        let hash = RET_signed_intent_hash(&scrypto_signed_intent).expect("Sargon should only produce valid SignedIntent, should never fail to produce signed intent hash using RET.");
        let scrypto_signed_intent_hash = ScryptoSignedIntentHash(hash.hash);
        SignedIntentHash::from_scrypto(
            scrypto_signed_intent_hash,
            self.network_id(),
        )
    }
}

impl From<SignedIntent> for ScryptoSignedIntent {
    fn from(value: SignedIntent) -> Self {
        Self {
            intent: value.intent.into(),
            intent_signatures: value.intent_signatures.into(),
        }
    }
}

impl HasSampleValues for SignedIntent {
    fn sample() -> Self {
        Self::new_validating_signatures(
            TransactionIntent::sample(),
            IntentSignatures::default(),
        )
        .unwrap()
    }

    fn sample_other() -> Self {
        let intent = TransactionIntent::sample_other();
        let private_key: Secp256k1PrivateKey =
            radix_engine::types::Secp256k1PrivateKey::from_u64(1)
                .unwrap()
                .into();
        let intent_signature =
            private_key.sign_intent_hash(&intent.intent_hash());

        Self::new_validating_signatures(
            intent,
            IntentSignatures::new([intent_signature]),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::prelude::*;

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
    fn many_intent_signatures_all_valid() {
        let intent = TransactionIntent::sample_other();

        let mut signatures = Vec::<IntentSignature>::new();
        for n in 1..4 {
            let private_key: Secp256k1PrivateKey =
                radix_engine::types::Secp256k1PrivateKey::from_u64(n)
                    .unwrap()
                    .into();

            let intent_signature =
                private_key.sign_intent_hash(&intent.intent_hash());
            signatures.push(intent_signature)
        }

        let intent_signatures = IntentSignatures::new(signatures);
        assert_eq!(intent_signatures.clone().signatures.into_iter().map(|s| s.signature().to_string()).collect_vec(), ["01da59c65684d07f1997bf9615c1e9330a54d8f3b13d8caaef1a8b32f64259be05544dc9290b64294a174c2857dd1043b3a5c0ca50bfc4ff35a95dd4338edee80b", "01427f6b48420da77ecb31c62b693d1970fb6cd3bcf68ea4ae21ae6c4e4521eff80100fed2410cba034a46cc5c546c9470cce1b44ff1c2d2e31c7ded420aa84024", "00570e538b1f84b323ea21b87930debed81d46a1a1abec5007c72106c4985ab515501af9a4ebbbfb75570416e0cc52dc93e064478c254fafb5065159e40b606612"]);

        assert_eq!(
            SUT::new_validating_signatures(intent, intent_signatures.clone())
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
                radix_engine::types::Secp256k1PrivateKey::from_u64(n)
                    .unwrap()
                    .into();

            let intent_signature =
                private_key.sign_intent_hash(&intent.intent_hash());
            signatures.push(intent_signature)
        }

        signatures.push(IntentSignature::sample());

        let intent_signatures = IntentSignatures::new(signatures);

        assert_eq!(
            SUT::new_validating_signatures(intent, intent_signatures.clone()),
            Err(CommonError::InvalidSignaturesForIntentSomeDidNotValidateIntentHash)
        );
    }

    #[test]
    fn many_intent_signatures_invalid_because_mismatching_intent() {
        let intent = TransactionIntent::sample_other();

        let mut signatures = Vec::<IntentSignature>::new();
        for n in 1..4 {
            let private_key: Secp256k1PrivateKey =
                radix_engine::types::Secp256k1PrivateKey::from_u64(n)
                    .unwrap()
                    .into();
            let hash = intent.intent_hash();
            let intent_signature = private_key.sign_intent_hash(&hash);
            signatures.push(intent_signature)
        }

        let intent_signatures = IntentSignatures::new(signatures);

        assert_eq!(
            SUT::new_validating_signatures(
                TransactionIntent::sample(), // <-- WRONG Intent, not was signed.
                intent_signatures.clone()
            ),
            Err(CommonError::InvalidSignaturesForIntentSomeDidNotValidateIntentHash)
        );
    }
}
