use crate::prelude::*;

/// Either a Signature on `Curve25519` or `Secp256k1`
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumAsInner,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Debug,
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

impl Signature {
    /// Returns a `SLIP10Curve`, being the curve of the `Signature`.
    pub fn curve(&self) -> SLIP10Curve {
        match self {
            Self::Ed25519 { .. } => SLIP10Curve::Curve25519,
            Self::Secp256k1 { .. } => SLIP10Curve::Secp256k1,
        }
    }
}

impl Signature {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Signature::Secp256k1 { value } => value.to_bytes(),
            Signature::Ed25519 { value } => value.to_bytes(),
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

impl TryFrom<BagOfBytes> for Signature {
    type Error = crate::CommonError;

    fn try_from(value: BagOfBytes) -> Result<Self> {
        if let Ok(sig) = Ed25519Signature::try_from(value.clone()) {
            Ok(Self::Ed25519 { value: sig })
        } else if let Ok(sig) = Secp256k1Signature::try_from(value.clone()) {
            Ok(Self::Secp256k1 { value: sig })
        } else {
            Err(CommonError::FailedToParseSignatureFromBytes {
                bad_value: value.to_hex(),
            })
        }
    }
}

impl FromStr for Signature {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(sig) = Ed25519Signature::from_str(s) {
            Ok(Self::Ed25519 { value: sig })
        } else if let Ok(sig) = Secp256k1Signature::from_str(s) {
            Ok(Self::Secp256k1 { value: sig })
        } else {
            Err(CommonError::FailedToParseSignatureFromString {
                bad_value: s.to_owned(),
            })
        }
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
    fn curve() {
        assert_eq!(SUT::sample().curve(), SLIP10Curve::Curve25519);
        assert_eq!(SUT::sample_other().curve(), SLIP10Curve::Secp256k1);
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip = |s: SUT| SUT::from(ScryptoSignature::from(s));
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
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

    #[test]
    fn parse_bad_str() {
        assert_eq!(
            "foobar".parse::<SUT>(),
            Err(CommonError::FailedToParseSignatureFromString {
                bad_value: "foobar".to_owned()
            })
        );
    }

    #[test]
    fn parse_bad_bytes() {
        let bytes = BagOfBytes::from_hex("dead").unwrap();
        assert_eq!(
            SUT::try_from(bytes),
            Err(CommonError::FailedToParseSignatureFromBytes {
                bad_value: "dead".to_owned()
            })
        );
    }

    #[test]
    fn ed25519_from_str() {
        assert_eq!(
            "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103".parse::<SUT>().unwrap(), SUT::sample());
    }

    #[test]
    fn ed25519_from_bag_of_bytes() {
        let bytes: BagOfBytes = "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103".parse().unwrap();
        assert_eq!(SUT::try_from(bytes.clone()).unwrap(), SUT::sample());
        assert_eq!(SUT::sample().to_bytes(), bytes.to_vec());
    }

    #[test]
    fn secp256k1_from_str() {
        let s = "0001598e989470d125dafac276b95bb1ba21e2ee8e0beb0547599335f83b48a0a830cd6a956a54421039cef5fb7e492ebaa315f751a2dd5b74bd9cebbda997ec12";
        assert_eq!(s.parse::<SUT>().unwrap(), SUT::sample_other());
        assert_eq!(hex_encode(SUT::sample_other().to_bytes()), s);
    }

    #[test]
    fn secp256k1_from_bag_of_bytes() {
        let bytes: BagOfBytes = "0001598e989470d125dafac276b95bb1ba21e2ee8e0beb0547599335f83b48a0a830cd6a956a54421039cef5fb7e492ebaa315f751a2dd5b74bd9cebbda997ec12".parse().unwrap();
        assert_eq!(SUT::try_from(bytes).unwrap(), SUT::sample_other());
    }
}
