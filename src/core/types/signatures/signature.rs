use transaction::model::SignatureV1 as ScryptoSignature;

use crate::prelude::*;

/// Either a Signature on `Curve25519` or `Secp256k1`
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    EnumAsInner,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Enum,
)]
pub enum Signature {
    Secp256k1 { value: Secp256k1Signature },
    Ed25519 { value: Ed25519Signature },
}

impl From<ScryptoSignature> for Signature {
    fn from(value: ScryptoSignature) -> Self {
        match value {
            ScryptoSignature::Secp256k1(s) => {
                Self::Secp256k1 { value: s.into() }
            }
            ScryptoSignature::Ed25519(s) => Self::Ed25519 { value: s.into() },
        }
    }
}
impl From<Signature> for ScryptoSignature {
    fn from(value: Signature) -> Self {
        match value {
            Signature::Secp256k1 { value } => Self::Secp256k1(value.into()),
            Signature::Ed25519 { value } => Self::Ed25519(value.into()),
        }
    }
}

impl From<Secp256k1Signature> for Signature {
    fn from(signature: Secp256k1Signature) -> Self {
        Self::Secp256k1 { value: signature }
    }
}

impl From<Ed25519Signature> for Signature {
    fn from(signature: Ed25519Signature) -> Self {
        Self::Ed25519 { value: signature }
    }
}

impl HasSampleValues for Signature {
    fn sample() -> Self {
        Ed25519Signature::sample().into()
    }

    fn sample_other() -> Self {
        Secp256k1Signature::sample().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Signature;

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
    fn enum_as_inner() {
        assert_eq!(
            SUT::sample().as_ed25519().unwrap(),
            &Ed25519Signature::sample()
        );
        assert_eq!(
            SUT::sample_other().as_secp256k1().unwrap(),
            &Secp256k1Signature::sample()
        );
    }
}
