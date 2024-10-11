use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Default, Hash, uniffi::Record)]
pub struct IntentSignaturesV2 {
    pub signatures: Vec<IntentSignature>,
}

impl IntentSignaturesV2 {
    pub fn new<I>(signatures: I) -> Self
    where
        I: IntoIterator<Item = IntentSignature>,
    {
        Self {
            signatures: signatures.into_iter().collect(),
        }
    }

    pub fn validate(&self, hash: impl Into<Hash>) -> bool {
        let hash = hash.into();

        self.signatures.iter().all(|s| s.validate(hash))
    }
}

impl From<IntentSignaturesV2> for ScryptoIntentSignaturesV2 {
    fn from(value: IntentSignaturesV2) -> Self {
        Self {
            signatures: value
                .signatures
                .into_iter()
                .map(|s| s.into())
                .collect(),
        }
    }
}

impl TryFrom<(ScryptoIntentSignaturesV2, Hash)> for IntentSignaturesV2 {
    type Error = crate::CommonError;

    fn try_from(
        value: (ScryptoIntentSignaturesV2, Hash),
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

impl HasSampleValues for IntentSignaturesV2 {
    fn sample() -> Self {
        let intent = TransactionIntent::sample();
        let mut signatures = Vec::<IntentSignature>::new();
        for n in 1..4 {
            let private_key: Secp256k1PrivateKey =
                ScryptoSecp256k1PrivateKey::from_u64(n).unwrap().into();

            signatures.push(private_key.sign_transaction_intent_hash(
                &intent.transaction_intent_hash(),
            ))
        }

        IntentSignaturesV2::new(signatures)
    }

    fn sample_other() -> Self {
        let intent = TransactionIntent::sample_other();
        let mut signatures = Vec::<IntentSignature>::new();
        for n in 1..4 {
            let private_key: Secp256k1PrivateKey =
                ScryptoSecp256k1PrivateKey::from_u64(n).unwrap().into();

            signatures.push(private_key.sign_transaction_intent_hash(
                &intent.transaction_intent_hash(),
            ))
        }

        IntentSignaturesV2::new(signatures)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IntentSignaturesV2;

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
