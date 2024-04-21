use crate::prelude::*;

/// An ephemeral (never persisted) HD PrivateKey which contains
/// the derivation path used to derive it.
#[derive(Debug, PartialEq, Eq)]
pub struct HierarchicalDeterministicPrivateKey {
    /// The PrivateKey derived from some HD FactorSource using `derivation_path`.
    pub private_key: PrivateKey,

    /// Derivation path used to derive the `PrivateKey` from some HD FactorSource.
    pub derivation_path: DerivationPath,
}

impl HierarchicalDeterministicPrivateKey {
    /// Instantiates a new `HierarchicalDeterministicPrivateKey` from a PrivateKey and
    /// the derivation path used to derive it.
    pub fn new(
        private_key: PrivateKey,
        derivation_path: DerivationPath,
    ) -> Self {
        Self {
            private_key,
            derivation_path,
        }
    }
}

impl HierarchicalDeterministicPrivateKey {
    pub fn sign(&self, hash_to_sign: &Hash) -> SignatureWithPublicKey {
        let signature = self.private_key.sign(hash_to_sign);
        match signature {
            Signature::Ed25519 { value } => SignatureWithPublicKey::Ed25519 {
                public_key: *self
                    .private_key
                    .public_key()
                    .as_ed25519()
                    .unwrap(),
                signature: value,
            },
            Signature::Secp256k1 { value } => {
                SignatureWithPublicKey::Secp256k1 {
                    public_key: *self
                        .private_key
                        .public_key()
                        .as_secp256k1()
                        .unwrap(),
                    signature: value,
                }
            }
        }
    }

    /// Returns the public key of the private key with the derivation path intact.
    pub fn public_key(&self) -> HierarchicalDeterministicPublicKey {
        HierarchicalDeterministicPublicKey::new(
            self.private_key.public_key(),
            self.derivation_path.clone(),
        )
    }

    /// The PrivateKey as hex string.
    pub fn to_hex(&self) -> String {
        self.private_key.to_hex()
    }
}

impl HasSampleValues for HierarchicalDeterministicPrivateKey {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::new(
            Ed25519PrivateKey::from_str(
                "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003",
            )
            .unwrap()
            .into(),
            AccountPath::from_str("m/44H/1022H/1H/525H/1460H/0H")
                .unwrap()
                .into(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            Secp256k1PrivateKey::from_str(
                "111323d507d9d690836798e3ef2e5292cfd31092b75b9b59fa584ff593a3d7e4",
            )
            .unwrap()
            .into(),
            BIP44LikePath::from_str("m/44H/1022H/0H/0/5H")
                .unwrap()
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HierarchicalDeterministicPrivateKey;

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
    fn public_key_of_sample() {
        let sut = SUT::sample();
        assert_eq!(
            sut.public_key().to_hex(),
            "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
        );
    }

    #[test]
    fn test_sign_secp256k1() {
        let sut = SUT::sample_other();
        let hash = Hash::sample();
        let signature = sut.sign(&hash);
        assert!(signature.is_valid(&hash));
    }
}
