use crate::prelude::*;

use transaction::model::IntentSignatureV1 as ScryptoIntentSignature;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct IntentSignature {
    pub(crate) secret_magic: SignatureWithPublicKey,
}

impl From<SignatureWithPublicKey> for IntentSignature {
    fn from(value: SignatureWithPublicKey) -> Self {
        Self {
            secret_magic: value,
        }
    }
}

impl From<IntentSignature> for ScryptoIntentSignature {
    fn from(value: IntentSignature) -> Self {
        ScryptoIntentSignature(value.secret_magic.into())
    }
}

impl From<ScryptoIntentSignature> for IntentSignature {
    fn from(value: ScryptoIntentSignature) -> Self {
        Self {
            secret_magic: value.0.into(),
        }
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
    fn to_from_scrypto() {
        let roundtrip =
            |s: SUT| Into::<SUT>::into(Into::<ScryptoIntentSignature>::into(s));
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }
}
