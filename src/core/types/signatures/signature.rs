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
    fn parse_ed25519() {
        assert_eq!(
            "2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b".parse::<SUT>().unwrap(), SUT::sample());
    }

    #[test]
    fn parse_secp256k1() {
        assert_eq!(
            "018ad795353658a0cd1b513c4414cbafd0f990d329522977f8885a27876976a7d41ed8a81c1ac34551819627689cf940c4e27cacab217f00a0a899123c021ff6ef".parse::<SUT>().unwrap(), SUT::sample_other());
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip = |s: SUT| SUT::from(ScryptoSignature::from(s));
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }
}
