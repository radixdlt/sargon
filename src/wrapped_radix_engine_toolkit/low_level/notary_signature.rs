use crate::prelude::*;

#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::FromStr,
    uniffi::Record,
)]
pub struct NotarySignature {
    pub(crate) secret_magic: Signature,
}

impl From<ScryptoNotarySignature> for NotarySignature {
    fn from(value: ScryptoNotarySignature) -> Self {
        Self {
            secret_magic: value.0.into(),
        }
    }
}

impl From<NotarySignature> for ScryptoNotarySignature {
    fn from(value: NotarySignature) -> Self {
        ScryptoNotarySignature(value.secret_magic.into())
    }
}

impl From<Signature> for NotarySignature {
    fn from(value: Signature) -> Self {
        Self {
            secret_magic: value,
        }
    }
}

impl From<Secp256k1Signature> for NotarySignature {
    fn from(value: Secp256k1Signature) -> Self {
        Signature::from(value).into()
    }
}

impl From<Ed25519Signature> for NotarySignature {
    fn from(value: Ed25519Signature) -> Self {
        Signature::from(value).into()
    }
}

impl HasSampleValues for NotarySignature {
    fn sample() -> Self {
        Signature::sample().into()
    }

    fn sample_other() -> Self {
        Signature::sample_other().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NotarySignature;

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
        let roundtrip = |s: SUT| SUT::from(s);
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
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
            "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103".parse::<SUT>().unwrap(), SUT::sample());
    }

    #[test]
    fn parse_secp256k1() {
        assert_eq!(
            "0001598e989470d125dafac276b95bb1ba21e2ee8e0beb0547599335f83b48a0a830cd6a956a54421039cef5fb7e492ebaa315f751a2dd5b74bd9cebbda997ec12".parse::<SUT>().unwrap(), SUT::sample_other());
    }

    #[test]
    fn from_scrypto_notary() {
        let sig: radix_common::crypto::Ed25519Signature = "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103".parse().unwrap();
        let scrypto_notary = ScryptoNotarySignature(
            radix_transactions::model::SignatureV1::Ed25519(sig),
        );
        assert_eq!(SUT::from(scrypto_notary), SUT::sample());
    }

    #[test]
    fn from_ed25519() {
        assert_eq!(SUT::from(Ed25519Signature::sample()), SUT::sample());
    }

    #[test]
    fn from_secp256k1() {
        assert_eq!(
            SUT::from(Secp256k1Signature::sample()),
            SUT::sample_other()
        );
    }
}
