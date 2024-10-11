use crate::prelude::*;
use radix_common::crypto::{blake2b_256_hash, Hash};

/// FactorSourceID from the blake2b hash of the special HD public key derived at `CAP26::GetID`,
/// for a certain `FactorSourceKind`
#[derive(
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[display("{}", self.to_canonical_string())]
#[debug("{}", self.to_canonical_string())]
pub struct FactorSourceIDFromHash {
    /// The kind of the FactorSource this ID refers to, typically `device` or `ledger`.
    pub kind: FactorSourceKind,

    /// The blake2b hash of the special HD public key derived at `CAP26::GetID`.
    pub body: Exactly32Bytes,
}

impl FactorSourceIDFromHash {
    /// Instantiates a new `FactorSourceIDFromHash` from the `kind` and `body`.
    pub fn new(kind: FactorSourceKind, body: Exactly32Bytes) -> Self {
        Self { kind, body }
    }

    pub fn from_mnemonic_with_passphrase(
        factor_source_kind: FactorSourceKind,
        mnemonic_with_passphrase: &MnemonicWithPassphrase,
    ) -> Self {
        let seed = mnemonic_with_passphrase.to_seed();
        let private_key = seed.derive_private_key(&GetIDPath::default());
        let public_key_bytes = private_key.public_key().to_bytes();
        // TODO: Impl Zeroize for `PrivateKey`!
        let hash = hash_of(public_key_bytes);
        let body = Exactly32Bytes::from(hash);
        Self::new(factor_source_kind, body)
        // `BIP39Seed` implements `ZeroizeOnDrop` so `seed` should now be zeroized
    }

    pub fn new_for_device(
        mnemonic_with_passphrase: &MnemonicWithPassphrase,
    ) -> Self {
        Self::from_mnemonic_with_passphrase(
            FactorSourceKind::Device,
            mnemonic_with_passphrase,
        )
    }

    pub fn new_for_ledger(
        mnemonic_with_passphrase: &MnemonicWithPassphrase,
    ) -> Self {
        Self::from_mnemonic_with_passphrase(
            FactorSourceKind::LedgerHQHardwareWallet,
            mnemonic_with_passphrase,
        )
    }

    pub fn new_for_security_questions(
        mnemonic_with_passphrase: &MnemonicWithPassphrase,
    ) -> Self {
        Self::from_mnemonic_with_passphrase(
            FactorSourceKind::SecurityQuestions,
            mnemonic_with_passphrase,
        )
    }

    pub fn new_for_arculus(
        mnemonic_with_passphrase: &MnemonicWithPassphrase,
    ) -> Self {
        Self::from_mnemonic_with_passphrase(
            FactorSourceKind::ArculusCard,
            mnemonic_with_passphrase,
        )
    }

    pub fn new_for_off_device(
        mnemonic_with_passphrase: &MnemonicWithPassphrase,
    ) -> Self {
        Self::from_mnemonic_with_passphrase(
            FactorSourceKind::OffDeviceMnemonic,
            mnemonic_with_passphrase,
        )
    }

    pub fn new_for_passphrase(
        mnemonic_with_passphrase: &MnemonicWithPassphrase,
    ) -> Self {
        Self::from_mnemonic_with_passphrase(
            FactorSourceKind::Passphrase,
            mnemonic_with_passphrase
        )
    }
}

impl FactorSourceIDFromHash {
    pub fn to_canonical_string(&self) -> String {
        format!("{}:{}", self.kind.discriminant(), self.body)
    }
}

impl HasSampleValues for FactorSourceIDFromHash {
    /// A sample used to facilitate unit tests, just an alias
    /// for `sample_device`
    fn sample() -> Self {
        Self::sample_device()
    }

    fn sample_other() -> Self {
        Self::sample_ledger()
    }
}

impl FactorSourceIDFromHash {
    pub fn sample_device() -> Self {
        Self::new_for_device(&MnemonicWithPassphrase::sample_device())
    }

    pub fn sample_device_other() -> Self {
        Self::new_for_device(&MnemonicWithPassphrase::sample_device_other())
    }

    pub fn sample_ledger() -> Self {
        Self::new_for_ledger(&MnemonicWithPassphrase::sample_ledger())
    }

    pub fn sample_ledger_other() -> Self {
        Self::new_for_ledger(&MnemonicWithPassphrase::sample_ledger_other())
    }

    pub fn sample_arculus() -> Self {
        Self::new_for_arculus(&MnemonicWithPassphrase::sample_arculus())
    }

    pub fn sample_arculus_other() -> Self {
        Self::new_for_arculus(&MnemonicWithPassphrase::sample_arculus_other())
    }

    pub fn sample_off_device() -> Self {
        Self::new_for_arculus(&MnemonicWithPassphrase::sample_off_device())
    }

    pub fn sample_off_device_other() -> Self {
        Self::new_for_arculus(&MnemonicWithPassphrase::sample_off_device_other())
    }

    pub fn sample_security_questions() -> Self {
        Self::new_for_security_questions(&MnemonicWithPassphrase::sample_security_questions())
    }

    pub fn sample_security_questions_other() -> Self {
        Self::new_for_security_questions(&MnemonicWithPassphrase::sample_security_questions_other())
    }

    pub fn sample_passphrase() -> Self {
        Self::new_for_passphrase(&MnemonicWithPassphrase::sample_passphrase())
    }

    pub fn sample_passphrase_other() -> Self {
        Self::new_for_passphrase(&MnemonicWithPassphrase::sample_passphrase_other())
    }

    pub(crate) fn sample_at(index: usize) -> FactorSourceIDFromHash {
        ALL_FACTOR_SOURCE_IDS_SAMPLES[index].clone()
    }
}

/// FactorSourceIDFromHash samples used in various tests, specifically in signature collector tests.
pub(crate) static ALL_FACTOR_SOURCE_IDS_SAMPLES: Lazy<[FactorSourceIDFromHash; 11]> = Lazy::new(|| {
    [
        FactorSourceIDFromHash::sample_device(),
        FactorSourceIDFromHash::sample_ledger(),
        FactorSourceIDFromHash::sample_ledger_other(),
        FactorSourceIDFromHash::sample_arculus(),
        FactorSourceIDFromHash::sample_arculus_other(),
        FactorSourceIDFromHash::sample_passphrase(),
        FactorSourceIDFromHash::sample_passphrase_other(),
        FactorSourceIDFromHash::sample_off_device(),
        FactorSourceIDFromHash::sample_off_device_other(),
        FactorSourceIDFromHash::sample_security_questions(),
        FactorSourceIDFromHash::sample_device_other()
    ]
});

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceIDFromHash;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());

        let s = SUT::sample_all()[0];
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", SUT::sample()),
            "device:f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::sample()),
            "device:f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
        );
    }

    #[test]
    fn json_roundtrip_sample() {
        let model = SUT::sample();

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "kind": "device",
                "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
            }
            "#,
        );
    }

    #[test]
    fn json_from_sample_mnemonic() {
        let mwp = MnemonicWithPassphrase::sample();
        let model = SUT::new_for_device(&mwp);
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "kind": "device",
                "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
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
            BIP39Passphrase::new(vector.pass),
        );
        let id = SUT::new_for_device(&mwp);
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
        vectors.into_iter().for_each(test_vector);
    }
}
