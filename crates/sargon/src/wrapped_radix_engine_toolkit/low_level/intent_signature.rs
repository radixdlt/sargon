use crate::prelude::*;

#[derive(
    Clone, Copy, PartialOrd, Ord, Debug, PartialEq, Eq, Hash
)]
pub struct IntentSignature(pub SignatureWithPublicKey)

impl IntentSignature {
    pub fn signature(&self) -> Signature {
        self.0.clone().signature()
    }

    pub fn public_key(&self) -> PublicKey {
        self.0.clone().public_key()
    }

    pub fn validate(&self, hash: impl Into<Hash>) -> bool {
        let hash = hash.into();
        self.0.is_valid_for_hash(&hash)
    }
}

impl From<IntentSignature> for Signature {
    fn from(value: IntentSignature) -> Self {
        value.signature()
    }
}

impl TryFrom<(ScryptoIntentSignature, Hash)> for IntentSignature {
    type Error = crate::CommonError;

    fn try_from(
        value: (ScryptoIntentSignature, Hash),
    ) -> Result<Self, Self::Error> {
        TryInto::<SignatureWithPublicKey>::try_into((value.0 .0, value.1))
            .map(Self::from)
    }
}

impl From<SignatureWithPublicKey> for IntentSignature {
    fn from(value: SignatureWithPublicKey) -> Self {
        Self(value)
    }
}

impl From<IntentSignature> for ScryptoIntentSignature {
    fn from(value: IntentSignature) -> Self {
        ScryptoIntentSignature(value.secret_magic.into())
    }
}

impl HasSampleValues for IntentSignature {
    fn sample() -> Self {
        SignatureWithPublicKey::sample().into()
    }

    fn sample_other() -> Self {
        SignatureWithPublicKey::sample_other().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IntentSignature;

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
    fn into_signature_for_ed25519() {
        assert_eq!(Signature::from(SUT::sample()), Signature::sample());
    }

    #[test]
    fn into_signature_for_secp256k1() {
        assert_eq!(
            Signature::from(SUT::sample_other()),
            Signature::sample_other()
        );
    }

    #[test]
    fn into_scrypto() {
        let scrypto: ScryptoIntentSignature = SUT::sample_other().into();
        assert_eq!(
            scrypto.0.signature(),
            SUT::sample_other().signature().into()
        )
    }

    #[test]
    fn try_from_scrypto_valid() {
        let scrypto =
            ScryptoIntentSignature(SignatureWithPublicKey::sample().into());
        assert!(SUT::try_from((scrypto, Hash::sample())).is_ok());
    }
}
