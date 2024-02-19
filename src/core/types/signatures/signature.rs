use crate::prelude::*;

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

impl HasPlaceholder for Signature {
    fn placeholder() -> Self {
        Ed25519Signature::placeholder().into()
    }

    fn placeholder_other() -> Self {
        Secp256k1Signature::placeholder().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Signature;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
    }

    #[test]
    fn enum_as_inner() {
        assert_eq!(
            SUT::placeholder().as_ed25519().unwrap(),
            &Ed25519Signature::placeholder()
        );
        assert_eq!(
            SUT::placeholder_other().as_secp256k1().unwrap(),
            &Secp256k1Signature::placeholder()
        );
    }
}
