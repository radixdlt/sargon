use crate::prelude::*;

/// Represents any natively supported signature, including public key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum, EnumAsInner)]
pub enum SignatureWithPublicKey {
    // N.B. `transaction::model::SignatureWithPublicKeyV1::Secp256k1` does
    // NOT include the public key, it relies on ECDSA Signature supporting
    // recovery, but it is not reliable since passing the wrong hash to
    // a signature will return the WRONG public key. In other words one might
    // naively believe that recovery should fail for the wrong hash passed in,
    // but instead the wrong public key is returned. In the context of Scrypto
    // or Node, they might have a mechanism by which they can validate the
    // public key against some address or sub-state, but we play it safe, the
    // cost of having the public key around in the ephemeral operations working
    // with `SignatureWithPublicKey` is near-zero, so we have it explicit in state.
    Secp256k1 {
        public_key: Secp256k1PublicKey,
        signature: Secp256k1Signature,
    },
    Ed25519 {
        public_key: Ed25519PublicKey,
        signature: Ed25519Signature,
    },
}

impl From<SignatureWithPublicKey> for ScryptoSignatureWithPublicKey {
    fn from(value: SignatureWithPublicKey) -> Self {
        match value {
            SignatureWithPublicKey::Secp256k1 {
                public_key: _,
                signature,
            } => Self::Secp256k1 {
                signature: signature.into(),
            },
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

impl TryFrom<(ScryptoSignatureWithPublicKey, Hash)> for SignatureWithPublicKey {
    type Error = crate::CommonError;

    fn try_from(
        value: (ScryptoSignatureWithPublicKey, Hash),
    ) -> Result<Self, Self::Error> {
        match value.0 {
            ScryptoSignatureWithPublicKey::Secp256k1 { signature } => {
                let hash: radix_engine_common::crypto::Hash = value.1.into();
                let scrypto_public_key = Scrypto_recover_secp256k1(
                    &hash, &signature,
                )
                .ok_or(
                    CommonError::FailedToRecoverSecp256k1PublicKeyFromSignature,
                )?;
                TryInto::<Secp256k1PublicKey>::try_into(scrypto_public_key).map(
                    |public_key| Self::Secp256k1 {
                        public_key,
                        signature: signature.into(),
                    },
                )
            }
            ScryptoSignatureWithPublicKey::Ed25519 {
                public_key: scrypto_public_key,
                signature,
            } => TryInto::<Ed25519PublicKey>::try_into(scrypto_public_key).map(
                |public_key| Self::Ed25519 {
                    public_key,
                    signature: signature.into(),
                },
            ),
        }
    }
}

impl SignatureWithPublicKey {
    pub fn signature(&self) -> Signature {
        match &self {
            Self::Secp256k1 { signature, .. } => signature.clone().into(),
            Self::Ed25519 { signature, .. } => signature.clone().into(),
        }
    }

    pub fn public_key(&self) -> PublicKey {
        match &self {
            Self::Secp256k1 { public_key, .. } => public_key.clone().into(),
            Self::Ed25519 { public_key, .. } => public_key.clone().into(),
        }
    }
}

impl From<(Secp256k1PublicKey, Secp256k1Signature)> for SignatureWithPublicKey {
    fn from(
        (public_key, signature): (Secp256k1PublicKey, Secp256k1Signature),
    ) -> Self {
        Self::Secp256k1 {
            public_key,
            signature,
        }
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

        (public_key, signature).into()
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
    fn to_scrypto() {
        match ScryptoSignatureWithPublicKey::from(SUT::sample()) {
            ScryptoSignatureWithPublicKey::Ed25519 {
                signature,
                public_key: _,
            } => {
                assert_eq!(
                    Ed25519Signature::from(signature),
                    Ed25519Signature::sample()
                );
            }
            ScryptoSignatureWithPublicKey::Secp256k1 { signature: _ } => {
                panic!("wrong curve")
            }
        }
    }

    #[test]
    fn to_scrypto_other() {
        match ScryptoSignatureWithPublicKey::from(SUT::sample_other()) {
            ScryptoSignatureWithPublicKey::Secp256k1 { signature } => {
                assert_eq!(
                    Secp256k1Signature::from(signature),
                    Secp256k1Signature::sample()
                )
            }
            ScryptoSignatureWithPublicKey::Ed25519 {
                public_key: _,
                signature: _,
            } => panic!("wrong curve"),
        }
    }

    #[test]
    fn try_from_scrypto_invalid_ed25519() {
        assert_eq!(
            TryInto::<SUT>::try_into((
                ScryptoSignatureWithPublicKey::Ed25519 {
                    public_key: ScryptoEd25519PublicKey([4u8; 32]),
                    signature: ScryptoEd25519Signature([2u8; 64])
                },
                Hash::sample()
            )),
            Err(CommonError::InvalidEd25519PublicKeyPointNotOnCurve)
        );
    }

    #[test]
    fn try_from_scrypto_invalid_secp256k1_public_key() {
        assert_eq!(
            TryInto::<SUT>::try_into((
                ScryptoSignatureWithPublicKey::Secp256k1 {
                    signature: ScryptoSecp256k1Signature([2u8; 65])
                },
                Hash::sample()
            )),
            Err(CommonError::FailedToRecoverSecp256k1PublicKeyFromSignature)
        );
    }

    // This is unfortunate, but it is how ECDSA recoverable signatures work, a
    // WRONG hash will produce the WRONG publickey instead of error, given a
    // secp256k1 signature
    #[test]
    fn try_from_scrypto_invalid_secp256k1_ok_even_for_wrong_hash() {
        assert_eq!(
            TryInto::<SUT>::try_into((
                ScryptoSignatureWithPublicKey::Secp256k1 {
                    signature: ScryptoSecp256k1Signature::from_str("018ad795353658a0cd1b513c4414cbafd0f990d329522977f8885a27876976a7d41ed8a81c1ac34551819627689cf940c4e27cacab217f00a0a899123c021ff6ef").unwrap()
                },
                Hash::sample()
            )).unwrap().into_secp256k1().unwrap().0,
            Secp256k1PublicKey::from_str("02634e157ed84916e1a79c8c0e802772d2b095ea4e5636243a0cade9896dd4b500").unwrap()
        );
    }
}
