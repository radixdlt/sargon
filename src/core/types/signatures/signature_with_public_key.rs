use crate::prelude::*;

use radix_engine_common::crypto::Ed25519Signature as ScryptoEd25519Signature;
use radix_engine_common::crypto::Secp256k1Signature as ScryptoSecp256k1Signature;
use transaction::model::SignatureWithPublicKeyV1 as ScryptoSignatureWithPublicKey;

/// Represents any natively supported signature, including public key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum, EnumAsInner)]
pub enum SignatureWithPublicKey {
    Secp256k1 {
        signature: Secp256k1Signature,
    },
    Ed25519 {
        public_key: Ed25519PublicKey,
        signature: Ed25519Signature,
    },
}

impl From<ScryptoSignatureWithPublicKey> for SignatureWithPublicKey {
    fn from(value: ScryptoSignatureWithPublicKey) -> Self {
        match value {
            ScryptoSignatureWithPublicKey::Secp256k1 { signature } => {
                Self::Secp256k1 {
                    signature: signature.into(),
                }
            }
            ScryptoSignatureWithPublicKey::Ed25519 {
                public_key,
                signature,
            } => Self::Ed25519 {
                public_key: public_key
                    .try_into()
                    .expect("Invalid public key found."),
                signature: signature.into(),
            },
        }
    }
}
impl From<SignatureWithPublicKey> for ScryptoSignatureWithPublicKey {
    fn from(value: SignatureWithPublicKey) -> Self {
        match value {
            SignatureWithPublicKey::Secp256k1 { signature } => {
                Self::Secp256k1 {
                    signature: signature.into(),
                }
            }
            SignatureWithPublicKey::Ed25519 {
                public_key,
                signature,
            } => Self::Ed25519 {
                public_key: public_key.into(),
                signature: signature.into(),
            },
        }
    }
}

impl SignatureWithPublicKey {
    pub fn signature(&self) -> Signature {
        match &self {
            Self::Secp256k1 { signature } => signature.clone().into(),
            Self::Ed25519 { signature, .. } => signature.clone().into(),
        }
    }
}

impl From<Secp256k1Signature> for SignatureWithPublicKey {
    fn from(signature: Secp256k1Signature) -> Self {
        Self::Secp256k1 { signature }
    }
}

impl From<(Ed25519PublicKey, Ed25519Signature)> for SignatureWithPublicKey {
    fn from(
        (public_key, signature): (Ed25519PublicKey, Ed25519Signature),
    ) -> Self {
        Self::Ed25519 {
            public_key,
            signature,
        }
    }
}

impl SignatureWithPublicKey {
    fn sample_message() -> String {
        "There is a computer disease that anybody who works with computers knows about. It's a very serious disease and it interferes completely with the work. The trouble with computers is that you 'play' with them!".to_owned()
    }
    fn sample_hash() -> Hash {
        hash_of(Self::sample_message().as_bytes())
    }
}

impl HasSampleValues for SignatureWithPublicKey {
    fn sample() -> Self {
        let private_key: Ed25519PrivateKey =
            "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003"
                .parse()
                .unwrap();
        let public_key = private_key.public_key();
        assert_eq!(
            &public_key.to_hex(),
            "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
        );
        let signature: Ed25519Signature = "2150c2f6b6c496d197ae03afb23f6adf23b275c675394f23786250abd006d5a2c7543566403cb414f70d0e229b0a9b55b4c74f42fc38cdf1aba2307f97686f0b".parse().unwrap();
        public_key.is_valid(&signature, &Self::sample_hash());

        (public_key, signature).into()
    }

    fn sample_other() -> Self {
        let private_key: Secp256k1PrivateKey =
            "111323d507d9d690836798e3ef2e5292cfd31092b75b9b59fa584ff593a3d7e4"
                .parse()
                .unwrap();
        let public_key = private_key.public_key();
        assert_eq!(&public_key.to_hex(), "03e78cdb2e0b7ea6e55e121a58560ccf841a913d3a4a9b8349e0ef00c2102f48d8");

        let signature: Secp256k1Signature = "018ad795353658a0cd1b513c4414cbafd0f990d329522977f8885a27876976a7d41ed8a81c1ac34551819627689cf940c4e27cacab217f00a0a899123c021ff6ef".parse().unwrap();
        public_key.is_valid(&signature, &Self::sample_hash());

        signature.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignatureWithPublicKey;

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
    fn signature() {
        let pubkey: Ed25519PublicKey =
            "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
                .parse()
                .unwrap();
        assert!(pubkey.is_valid(
            &SUT::sample().signature().into_ed25519().unwrap(),
            &SUT::sample_hash(),
        ));
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip = |s: SUT| {
            assert_eq!(
                Into::<SUT>::into(Into::<ScryptoSignatureWithPublicKey>::into(
                    s.clone()
                )),
                s
            )
        };
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }
}
