use crate::prelude::*;

/// The **source** of a virtual hierarchical deterministic badge, contains a
/// derivation path and public key, from which a private key is derived which
/// produces virtual badges (signatures).
///
/// The `.device` `FactorSource` produces `FactorInstance`s with this kind if badge source.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct HierarchicalDeterministicPublicKey {
    /// The expected public key of the private key derived at `derivationPath`
    pub public_key: PublicKey,

    /// The HD derivation path for the key pair which produces virtual badges (signatures).
    pub derivation_path: DerivationPath,
}

impl HierarchicalDeterministicPublicKey {
    pub fn new(public_key: PublicKey, derivation_path: DerivationPath) -> Self {
        Self {
            public_key,
            derivation_path,
        }
    }
}

impl HierarchicalDeterministicPublicKey {
    pub fn to_hex(&self) -> String {
        self.public_key.to_hex()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.public_key.to_bytes()
    }

    pub fn is_valid_signature_for_hash(
        &self,
        signature: impl Into<Signature>,
        hash: &impl ScryptoIsHash,
    ) -> bool {
        self.public_key.is_valid_signature_for_hash(signature, hash)
    }
}

impl HasSampleValues for HierarchicalDeterministicPublicKey {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        let mwp = MnemonicWithPassphrase::sample();
        let path = AccountPath::new(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            UnsecurifiedHardened::from_global_key_space(
                GLOBAL_OFFSET_HARDENED,
            )
            .unwrap(),
        );
        let seed = mwp.to_seed();
        let private_key = seed.derive_private_key(&path);

        assert_eq!(path.to_string(), "m/44H/1022H/1H/525H/1460H/0H");

        assert_eq!(
            "88ec4649da764965f862510dbe53d551a3fc2da49e1ef1f383d9d17006773bee",
            private_key.to_hex()
        );
        let public_key = private_key.public_key();
        assert_eq!(
            "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36",
            public_key.to_hex()
        );

        public_key
    }

    fn sample_other() -> Self {
        let mwp = MnemonicWithPassphrase::sample_other();
        let seed = mwp.to_seed();
        let private_key = seed.derive_private_key(
            &BIP44LikePath::from_str("m/44H/1022H/0H/0/5H").unwrap(),
        );

        assert_eq!(
            "09c5ec59b0cc08d07e5ed4aaee8c583264ffa060563d4b531e15db13d35b2a87",
            private_key.to_hex()
        );
        let public_key = private_key.public_key();
        assert_eq!(
            "038c9ae8b50356cfd87b6e8c069c14cbda692578e87cd41291701947a2d1b794c4",
            public_key.to_hex()
        );

        public_key
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HierarchicalDeterministicPublicKey;

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
    fn to_hex() {
        assert_eq!(
            SUT::sample().to_hex(),
            "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
        );
    }

    #[test]
    fn json() {
        let model = SUT::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
				"publicKey": {
					"curve": "curve25519",
					"compressedData": "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
				},
				"derivationPath": {
					"scheme": "cap26",
					"path": "m/44H/1022H/1H/525H/1460H/0H"
				}
			}
       "#,
        );
    }
}
