use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Default, Hash, uniffi::Record)]
pub struct IntentSignatures {
    pub signatures: Vec<IntentSignature>,
}

impl IntentSignatures {
    pub fn new<I>(signatures: I) -> Self
    where
        I: IntoIterator<Item = IntentSignature>,
    {
        Self {
            signatures: signatures.into_iter().collect_vec(),
        }
    }

    pub fn validate(&self, hash: impl Into<Hash>) -> bool {
        let hash = hash.into();

        self.signatures.iter().all(|s| s.validate(hash))
    }
}

impl From<IntentSignatures> for ScryptoIntentSignatures {
    fn from(value: IntentSignatures) -> Self {
        Self {
            signatures: value
                .signatures
                .into_iter()
                .map(|s| s.into())
                .collect_vec(),
        }
    }
}

impl TryFrom<(ScryptoIntentSignatures, Hash)> for IntentSignatures {
    type Error = crate::CommonError;

    fn try_from(
        value: (ScryptoIntentSignatures, Hash),
    ) -> Result<Self, Self::Error> {
        value
            .clone()
            .0
            .signatures
            .into_iter()
            .map(|s| {
                TryInto::<IntentSignature>::try_into((s, value.1.to_owned()))
            })
            .collect::<Result<Vec<IntentSignature>>>()
            .map(|signatures| Self { signatures })
    }
}

impl HasSampleValues for IntentSignatures {
    fn sample() -> Self {
        let intent = TransactionIntent::sample();
        let mut signatures = Vec::<IntentSignature>::new();
        for n in 1..4 {
            let private_key: Secp256k1PrivateKey =
                radix_engine::types::Secp256k1PrivateKey::from_u64(n)
                    .unwrap()
                    .into();

            signatures.push(private_key.sign_intent_hash(&intent.intent_hash()))
        }

        IntentSignatures::new(signatures)
    }

    fn sample_other() -> Self {
        let intent = TransactionIntent::sample_other();
        let mut signatures = Vec::<IntentSignature>::new();
        for n in 1..4 {
            let private_key: Secp256k1PrivateKey =
                radix_engine::types::Secp256k1PrivateKey::from_u64(n)
                    .unwrap()
                    .into();

            signatures.push(private_key.sign_intent_hash(&intent.intent_hash()))
        }

        IntentSignatures::new(signatures)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IntentSignatures;

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
    fn to_from_scrypto() {
        // We use `SignedIntent` instead of `SUT`, since `SignedIntent` contains
        // both SUT and Hash, needed for `TryFrom`.
        let roundtrip = |si: SignedIntent| {
            let first =
                ScryptoIntentSignatures::from(si.clone().intent_signatures);
            let second = si.clone().intent().intent_hash().hash;
            assert_eq!(
                SUT::try_from((first, second)).unwrap(),
                si.intent_signatures
            );
        };
        roundtrip(SignedIntent::sample());
        roundtrip(SignedIntent::sample_other());
    }
}
