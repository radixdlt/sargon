use crate::prelude::*;

use transaction::model::NotarySignatureV1 as ScryptoNotarySignature;

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
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
        let roundtrip =
            |s: SUT| Into::<SUT>::into(Into::<NotarySignature>::into(s));
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
            "2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b".parse::<SUT>().unwrap(), SUT::sample());
    }

    #[test]
    fn parse_secp256k1() {
        assert_eq!(
            "018ad795353658a0cd1b513c4414cbafd0f990d329522977f8885a27876976a7d41ed8a81c1ac34551819627689cf940c4e27cacab217f00a0a899123c021ff6ef".parse::<SUT>().unwrap(), SUT::sample_other());
    }

    #[test]
    fn from_scrypto_notary() {
        let sig: radix_engine_common::crypto::Ed25519Signature = "2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b".parse().unwrap();
        let scrypto_notary = ScryptoNotarySignature(
            transaction::model::SignatureV1::Ed25519(sig),
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
