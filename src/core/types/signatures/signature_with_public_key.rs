use crate::prelude::*;

/// Represents any natively supported signature, including public key.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumAsInner,
    uniffi::Enum,
)]
pub enum SignatureWithPublicKey {
    // N.B. `radix_transactions::model::SignatureWithPublicKeyV1::Secp256k1` does
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

impl From<SignatureWithPublicKey> for Signature {
    fn from(value: SignatureWithPublicKey) -> Self {
        value.signature()
    }
}

impl SignatureWithPublicKey {
    pub fn is_valid_for_hash(&self, hash: &impl ScryptoIsHash) -> bool {
        self.public_key()
            .is_valid_signature_for_hash(self.signature(), hash)
    }
}

impl TryFrom<(ScryptoSignatureWithPublicKey, Hash)> for SignatureWithPublicKey {
    type Error = crate::CommonError;

    fn try_from(
        value: (ScryptoSignatureWithPublicKey, Hash),
    ) -> Result<Self, Self::Error> {
        match value.0 {
            ScryptoSignatureWithPublicKey::Secp256k1 { signature } => {
                let hash: radix_common::crypto::Hash = value.1.into();
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
            Self::Secp256k1 { signature, .. } => (*signature).into(),
            Self::Ed25519 { signature, .. } => (*signature).into(),
        }
    }

    pub fn public_key(&self) -> PublicKey {
        match &self {
            Self::Secp256k1 { public_key, .. } => (*public_key).into(),
            Self::Ed25519 { public_key, .. } => (*public_key).into(),
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
            "88ec4649da764965f862510dbe53d551a3fc2da49e1ef1f383d9d17006773bee"
                .parse()
                .unwrap();
        let public_key = private_key.public_key();
        assert_eq!(
            &public_key.to_hex(),
            "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
        );

        let signature: Ed25519Signature = "fc6a4a15516b886b10f26777094cb1abdccb213c9ebdea7a4bceb83b6fcba50fea181b0136ee5659c3dfae5f771e5b6e6f9abbaa3f0435df0be1f732be965103".parse().unwrap();
        public_key
            .is_valid_signature_for_hash(&signature, &Self::sample_hash());

        (public_key, signature).into()
    }

    fn sample_other() -> Self {
        let private_key: Secp256k1PrivateKey =
            "09c5ec59b0cc08d07e5ed4aaee8c583264ffa060563d4b531e15db13d35b2a87"
                .parse()
                .unwrap();
        let public_key = private_key.public_key();
        assert_eq!(&public_key.to_hex(), "038c9ae8b50356cfd87b6e8c069c14cbda692578e87cd41291701947a2d1b794c4");

        let signature: Secp256k1Signature = "0001598e989470d125dafac276b95bb1ba21e2ee8e0beb0547599335f83b48a0a830cd6a956a54421039cef5fb7e492ebaa315f751a2dd5b74bd9cebbda997ec12".parse().unwrap();
        public_key
            .is_valid_signature_for_hash(&signature, &Self::sample_hash());

        (public_key, signature).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
                .parse()
                .unwrap();
        assert!(pubkey.is_valid_signature_for_hash(
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
                    signature: ScryptoSecp256k1Signature::from_str("0001598e989470d125dafac276b95bb1ba21e2ee8e0beb0547599335f83b48a0a830cd6a956a54421039cef5fb7e492ebaa315f751a2dd5b74bd9cebbda997ec12").unwrap()
                },
                Hash::sample()
            )).unwrap().into_secp256k1().unwrap().0,
            Secp256k1PublicKey::from_str("0395679954e3c312cab9905070effb4935e4b1e5b82f987396cabdb8faf9e554d0").unwrap()
        );
    }
}
