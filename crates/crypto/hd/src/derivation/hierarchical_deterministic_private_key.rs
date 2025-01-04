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
                "88ec4649da764965f862510dbe53d551a3fc2da49e1ef1f383d9d17006773bee",
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
                "09c5ec59b0cc08d07e5ed4aaee8c583264ffa060563d4b531e15db13d35b2a87",
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
            "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
        );
    }

    #[test]
    fn test_sign_secp256k1() {
        let sut = SUT::sample_other();
        let hash = Hash::sample();
        let signature = sut.sign(&hash);
        assert!(signature.is_valid_for_hash(&hash));
    }
}
