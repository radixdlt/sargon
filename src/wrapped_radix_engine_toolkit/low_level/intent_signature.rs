use crate::prelude::*;

use transaction::model::IntentSignatureV1 as ScryptoIntentSignature;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct IntentSignature {
    pub(crate) secret_magic: SignatureWithPublicKey,
}

impl IntentSignature {
    pub fn signature(&self) -> Signature {
        self.secret_magic.clone().signature()
    }

    pub fn public_key(&self) -> PublicKey {
        self.secret_magic.clone().public_key()
    }

    pub fn validate(&self, hash: impl Into<Hash>) -> bool {
        let hash = hash.into();
        self.public_key().is_valid(self.signature(), &hash)
    }
}

impl From<IntentSignature> for Signature {
    fn from(value: IntentSignature) -> Self {
        value.signature()
    }
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
        assert_eq!(Into::<Signature>::into(SUT::sample()), Signature::sample());
    }

    #[test]
    fn into_signature_for_secp256k1() {
        assert_eq!(
            Into::<Signature>::into(SUT::sample_other()),
            Signature::sample_other()
        );
    }
}
