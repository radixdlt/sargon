use derive_getters::Getters;
use hd::{GetIDPath, MnemonicWithPassphrase};
use radix_engine_common::crypto::{blake2b_256_hash, Hash};
use serde::{Deserialize, Serialize};
use wallet_kit_common::Hex32Bytes;

use super::factor_source_kind::FactorSourceKind;

/// FactorSourceID from the blake2b hash of the special HD public key derived at `CAP26::GetID`,
/// for a certain `FactorSourceKind`
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Getters)]
pub struct FactorSourceIDFromHash {
    /// The kind of the FactorSource this ID refers to, typically `device` or `ledger`.
    kind: FactorSourceKind,

    /// The blake2b hash of the special HD public key derived at `CAP26::GetID`.
    body: Hex32Bytes,
}

impl ToString for FactorSourceIDFromHash {
    fn to_string(&self) -> String {
        format!("{}:{}", self.kind.discriminant(), self.body.to_string())
    }
}

impl FactorSourceIDFromHash {
    /// Instantiates a new `FactorSourceIDFromHash` from the `kind` and `body`.
    pub fn new(kind: FactorSourceKind, body: Hex32Bytes) -> Self {
        Self { kind, body }
    }

    pub fn from_mnemonic_with_passphrase(
        factor_source_kind: FactorSourceKind,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
    ) -> Self {
        let private_key = mnemonic_with_passphrase.derive_private_key(GetIDPath::default());
        let public_key_bytes = private_key.public_key().to_bytes();
        let hash: Hash = blake2b_256_hash(public_key_bytes);
        let body = Hex32Bytes::from(hash);
        Self::new(factor_source_kind, body)
    }

    pub fn new_for_device(mnemonic_with_passphrase: MnemonicWithPassphrase) -> Self {
        Self::from_mnemonic_with_passphrase(FactorSourceKind::Device, mnemonic_with_passphrase)
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl FactorSourceIDFromHash {
    /// A placeholder used to facilitate unit tests, just an alias
    /// for `placeholder_device`
    pub fn placeholder() -> Self {
        Self::placeholder_device()
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_device() -> Self {
        Self::new_for_device(MnemonicWithPassphrase::placeholder())
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_ledger() -> Self {
        Self::from_mnemonic_with_passphrase(
            FactorSourceKind::LedgerHQHardwareWallet,
            MnemonicWithPassphrase::placeholder(),
        )
    }
}

#[cfg(test)]
mod tests {
    use hd::{Mnemonic, MnemonicWithPassphrase};
    use wallet_kit_common::assert_eq_after_json_roundtrip;

    use super::FactorSourceIDFromHash;

    #[test]
    fn json_roundtrip_placeholder() {
        let model = FactorSourceIDFromHash::placeholder();

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "kind": "device",
                "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
            }
            "#,
        );
    }

    #[test]
    fn json_from_placeholder_mnemonic() {
        let mwp = MnemonicWithPassphrase::placeholder();
        let model = FactorSourceIDFromHash::new_for_device(mwp);
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "kind": "device",
                "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
            }
            "#,
        );
    }

    struct Vector {
        /// Given input, bip49 mnemonic phrase
        phrase: String,
        /// Given input, bip39 passphrase
        pass: String,
        /// Expected output
        expected_id: String,
    }
    impl Vector {
        fn new(phrase: &str, pass: &str, id: &str) -> Self {
            Self {
                phrase: phrase.to_string(),
                pass: pass.to_string(),
                expected_id: id.to_string(),
            }
        }
        fn no_pass(phrase: &str, id: &str) -> Self {
            Self::new(phrase, "", id)
        }
    }

    fn test_vector(vector: Vector) {
        let mwp = MnemonicWithPassphrase::with_passphrase(
            Mnemonic::from_phrase(&vector.phrase).unwrap(),
            vector.pass,
        );
        let id = FactorSourceIDFromHash::new_for_device(mwp);
        assert_eq!(id.to_string(), vector.expected_id);
    }

    #[test]
    fn factor_source_id_from_mnemonic() {
        let vectors: Vec<Vector> = vec![
            Vector::no_pass(
                // source: https://github.com/radixdlt/babylon-wallet-ios/blob/main/RadixWalletTests/ProfileTests/TestCases/FactorSourceTests/FactorSourceTests.swift#L89
                "surprise jaguar gloom bring cage obey rotate fiber agree castle rich tomorrow",
                "device:56ee829c02d24487cbe98993f668ff646146e7c9bd02d1815118908c5355d750",
            ),
            Vector::no_pass(
                // source: https://github.com/radixdlt/babylon-wallet-ios/blob/main/RadixWalletTests/ProfileTests/TestCases/FactorSourceTests/FactorSourceTests.swift#L65
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong", 
                "device:09a501e4fafc7389202a82a3237a405ed191cdb8a4010124ff8e2c9259af1327"
            ),
            Vector::new(
                // source: https://github.com/radixdlt/babylon-wallet-ios/blob/main/RadixWalletTests/ProfileTests/TestCases/FactorSourceTests/FactorSourceTests.swift#L66C1-L66C1
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong",
                "foo", 
                "device:537b56b9881258f08994392e9858962825d92361b6b4775a3bdfeb4eecc0d069"
            ),
            Vector::no_pass(
                // source: https://github.com/radixdlt/babylon-wallet-ios/blob/a8d1543ed8242fcbe6cb0222fea246fe5b623c7b/RadixWalletTests/ProfileTests/TestCases/FactorSourceTests/FactorSourceTests.swift#L71
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote",
                "device:3bf4636876a9c795486194d2eaff32790961ed9005e18a7ebe677f0947b54087"
            ),
            Vector::new(
                // source: https://github.com/radixdlt/babylon-wallet-ios/blob/a8d1543ed8242fcbe6cb0222fea246fe5b623c7b/RadixWalletTests/ProfileTests/TestCases/FactorSourceTests/FactorSourceTests.swift#L72
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote",
                "foo",
                "device:883882e1d9d47b98090163bb4b369ae00349507693d856b1854de103dfe52793"
            ),
        ];
        vectors.into_iter().for_each(|v| test_vector(v));
    }
}
