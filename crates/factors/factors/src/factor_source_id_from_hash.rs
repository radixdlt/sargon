use core_utils::prelude::MapToFailedToDeserializeJSONToValue;
use hash::hash_of;

use crate::prelude::*;

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
)]
#[display("{}", self.to_canonical_string())]
#[debug("{}", self.to_canonical_string())]
pub struct FactorSourceIDFromHash {
    /// The kind of the FactorSource this ID refers to, typically `device` or `ledger`.
    pub kind: FactorSourceKind,

    /// The blake2b hash of the special HD public key derived at `CAP26::GetID`.
    pub body: Exactly32Bytes,
}

impl TryFrom<FactorSourceID> for FactorSourceIDFromHash {
    type Error = CommonError;

    fn try_from(value: FactorSourceID) -> Result<Self> {
        value
            .into_hash()
            .map_err(|_| CommonError::FactorSourceIDNotFromHash)
    }
}

impl FromStr for FactorSourceIDFromHash {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split(Self::SEPARATOR).collect_vec();
        if parts.len() != Self::STR_COMPONENTS_COUNT {
            return Err(CommonError::InvalidFactorSourceIDFromHashStringWrongComponentCount { expected: Self::STR_COMPONENTS_COUNT as u64, found: parts.len() as u64 });
        }
        let kind = FactorSourceKind::from_str(parts[0])?;
        let body = Exactly32Bytes::from_str(parts[1])?;
        Ok(Self::new(kind, body))
    }
}

impl FactorSourceIDFromHash {
    pub const STR_COMPONENTS_COUNT: usize = 2;

    /// Instantiates a new `FactorSourceIDFromHash` from the `kind` and `body`.
    pub fn new(kind: FactorSourceKind, body: Exactly32Bytes) -> Self {
        Self { kind, body }
    }

    pub const SEPARATOR: &str = ":";

    pub fn from_mnemonic_with_passphrase(
        factor_source_kind: FactorSourceKind,
        mnemonic_with_passphrase: &MnemonicWithPassphrase,
    ) -> Self {
        let seed = mnemonic_with_passphrase.to_seed();
        let private_key =
            seed.derive_ed25519_private_key(GetIDPath.to_hd_path());
        let public_key_bytes = private_key.public_key().to_bytes();
        // TODO: Impl Zeroize for `PrivateKey`!
        Self::from_public_key_bytes(factor_source_kind, public_key_bytes)
        // `BIP39Seed` implements `ZeroizeOnDrop` so `seed` should now be zeroized
    }

    pub fn from_public_key_bytes(
        factor_source_kind: FactorSourceKind,
        public_key_bytes: impl Into<Vec<u8>>,
    ) -> Self {
        let hash = hash_of(public_key_bytes.into());
        let body = Exactly32Bytes::from(hash);
        Self::new(factor_source_kind, body)
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

    pub fn new_for_arculus(public_key_bytes: impl Into<Vec<u8>>) -> Self {
        Self::from_public_key_bytes(
            FactorSourceKind::ArculusCard,
            public_key_bytes,
        )
    }

    pub fn new_for_arculus_with_mwp(
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

    pub fn new_for_password(
        mnemonic_with_passphrase: &MnemonicWithPassphrase,
    ) -> Self {
        Self::from_mnemonic_with_passphrase(
            FactorSourceKind::Password,
            mnemonic_with_passphrase,
        )
    }
}

impl FactorSourceIDFromHash {
    pub fn to_canonical_string(&self) -> String {
        [self.kind.discriminant(), self.body.to_string()]
            .into_iter()
            .join(Self::SEPARATOR)
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

    pub fn sample_device_12_words() -> Self {
        Self::new_for_device(&MnemonicWithPassphrase::sample_device_12_words())
    }

    pub fn sample_device_12_words_other() -> Self {
        Self::new_for_device(
            &MnemonicWithPassphrase::sample_device_12_words_other(),
        )
    }

    pub fn sample_ledger() -> Self {
        Self::new_for_ledger(&MnemonicWithPassphrase::sample_ledger())
    }

    pub fn sample_ledger_other() -> Self {
        Self::new_for_ledger(&MnemonicWithPassphrase::sample_ledger_other())
    }

    pub fn sample_arculus() -> Self {
        Self::from_mnemonic_with_passphrase(
            FactorSourceKind::ArculusCard,
            &MnemonicWithPassphrase::sample_arculus(),
        )
    }

    pub fn sample_arculus_other() -> Self {
        Self::from_mnemonic_with_passphrase(
            FactorSourceKind::ArculusCard,
            &MnemonicWithPassphrase::sample_arculus_other(),
        )
    }

    pub fn sample_off_device() -> Self {
        Self::new_for_off_device(&MnemonicWithPassphrase::sample_off_device())
    }

    pub fn sample_off_device_other() -> Self {
        Self::new_for_off_device(
            &MnemonicWithPassphrase::sample_off_device_other(),
        )
    }

    pub fn sample_security_questions() -> Self {
        Self::new_for_security_questions(
            &MnemonicWithPassphrase::sample_security_questions(),
        )
    }

    pub fn sample_security_questions_other() -> Self {
        Self::new_for_security_questions(
            &MnemonicWithPassphrase::sample_security_questions_other(),
        )
    }

    pub fn sample_password() -> Self {
        Self::new_for_password(&MnemonicWithPassphrase::sample_password())
    }

    pub fn sample_password_other() -> Self {
        Self::new_for_password(&MnemonicWithPassphrase::sample_password_other())
    }
}

/// Exposed function to deserialize `BagOfBytes` into a `Vec<FactorSourceIDFromHash>` for
/// uniffi crate
pub fn new_vec_of_factor_source_id_from_hash_from_json(
    json_bytes: BagOfBytes,
) -> Result<Vec<FactorSourceIDFromHash>> {
    serde_json::from_slice(json_bytes.as_slice())
        .map_failed_to_deserialize_bytes::<Vec<FactorSourceIDFromHash>>(
            json_bytes.as_slice(),
        )
}

/// Exposed function to serialize `Vec<FactorSourceIDFromHash>` into `BagOfBytes` uniffi crate
pub fn vec_of_factor_source_id_from_hash_to_json(
    ids: Vec<FactorSourceIDFromHash>,
) -> Result<BagOfBytes> {
    serde_json::to_vec(&ids)
        .map_err(|_| CommonError::FailedToSerializeToJSON)
        .map(BagOfBytes::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceIDFromHash;

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
    fn display() {
        assert_eq!(
            format!("{}", SUT::sample()),
            "device:f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
        );
    }

    #[test]
    fn from_str() {
        let s = "device:f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a";
        assert_eq!(SUT::from_str(s).unwrap(), SUT::sample());
    }

    #[test]
    fn from_str_err() {
        let s = "device";
        assert!(
            matches!(
                SUT::from_str(s),
                Err(CommonError::InvalidFactorSourceIDFromHashStringWrongComponentCount { expected: 2, found: 1 })
            )
        );
    }

    #[test]
    fn str_roundtrip() {
        let test = |sut: SUT| {
            let s = sut.to_string();
            let back_again = SUT::from_str(&s).unwrap();
            assert_eq!(sut, back_again);
        };
        test(SUT::sample());
        test(SUT::sample_other());
        test(SUT::from_mnemonic_with_passphrase(
            FactorSourceKind::ArculusCard,
            &MnemonicWithPassphrase::sample_arculus(),
        ));
        test(SUT::new_for_ledger(&MnemonicWithPassphrase::sample_ledger()));
        test(SUT::new_for_security_questions(
            &MnemonicWithPassphrase::sample_security_questions(),
        ));
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

    #[test]
    fn test_vec_serde_roundtrip() {
        let id1 = FactorSourceIDFromHash::sample_device();
        let id2 = FactorSourceIDFromHash::sample_device_other();

        let vec = vec![id1, id2];

        let encoded =
            vec_of_factor_source_id_from_hash_to_json(vec.clone()).unwrap();
        let decoded =
            new_vec_of_factor_source_id_from_hash_from_json(encoded).unwrap();

        assert_eq!(vec, decoded);
    }
}
