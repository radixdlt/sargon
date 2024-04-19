use crate::prelude::*;

/// A BIP39 Mnemonic and BIP39 passphrase - aka "25th word" tuple,
/// from which we can derive a HD Root used for derivation.
#[derive(
    Zeroize,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Debug,
    derive_more::Display,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[display("<OBFUSCATED>")]
#[debug("{:?}", self.partially_obfuscated_string())]
pub struct MnemonicWithPassphrase {
    pub mnemonic: Mnemonic,
    pub passphrase: BIP39Passphrase,
}

impl MnemonicWithPassphrase {
    #[cfg(not(tarpaulin_include))] // false negative
    pub fn partially_obfuscated_string(&self) -> String {
        format!(
            "{} + {}",
            self.mnemonic.partially_obfuscated_string(),
            self.passphrase.partially_obfuscated_string()
        )
    }
}
impl SafeToLog for MnemonicWithPassphrase {
    /// Logs the word count and FactorSourceID o
    fn non_sensitive(&self) -> impl std::fmt::Debug {
        self.partially_obfuscated_string()
    }
}

impl MnemonicWithPassphrase {
    pub fn with_passphrase(
        mnemonic: Mnemonic,
        passphrase: BIP39Passphrase,
    ) -> Self {
        Self {
            mnemonic,
            passphrase,
        }
    }

    /// Instantiates a new `MnemonicWithPassphrase` with empty passphrase (no passphrase).
    pub fn new(mnemonic: Mnemonic) -> Self {
        Self {
            mnemonic,
            passphrase: BIP39Passphrase::default(),
        }
    }

    pub fn generate_new() -> Self {
        Self::new(Mnemonic::generate_new())
    }

    /// Instantiates a new `MnemonicWithPassphrase` with empty passphrase (no passphrase),
    /// from the specified BIP39 mnemonic phrase.
    pub fn from_phrase(phrase: &str) -> Result<Self> {
        Mnemonic::from_phrase(phrase).map(Self::new)
    }

    pub fn to_seed(&self) -> BIP39Seed {
        self.mnemonic.to_seed(&self.passphrase.0)
    }
}

impl HasSampleValues for MnemonicWithPassphrase {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::with_passphrase(Mnemonic::sample(), BIP39Passphrase::sample())
    }

    fn sample_other() -> Self {
        Self::new(Mnemonic::sample_other())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = MnemonicWithPassphrase;

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
        assert_eq!(format!("{}", SUT::sample()), "<OBFUSCATED>");
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", SUT::sample()),
            format!("{:?}", "24 words (bright...mandate) + <NOT EMPTY>")
        );
        assert_eq!(
            format!("{:?}", SUT::sample_other()),
            format!("{:?}", "12 words (zoo...wrong) + <EMPTY>")
        );
    }

    #[test]
    fn non_sensitive() {
        assert_eq!(
            format!("{:?}", SUT::sample().non_sensitive()),
            format!("{:?}", "24 words (bright...mandate) + <NOT EMPTY>")
        );
        assert_eq!(
            format!("{:?}", SUT::sample_other().non_sensitive()),
            format!("{:?}", "12 words (zoo...wrong) + <EMPTY>")
        );
    }

    #[test]
    fn with_passphrase() {
        let phrase = "equip will roof matter pink blind book anxiety banner elbow sun young";
        let passphrase = "25th";
        let mwp = SUT::with_passphrase(
            Mnemonic::from_phrase(phrase).unwrap(),
            BIP39Passphrase::new(passphrase),
        );
        assert_eq!(mwp.mnemonic.phrase(), phrase);
        assert_eq!(mwp.passphrase.0, passphrase);
    }

    #[test]
    fn new_eq_from_phrase() {
        let phrase = "equip will roof matter pink blind book anxiety banner elbow sun young";
        assert_eq!(
            SUT::new(Mnemonic::from_phrase(phrase).unwrap()),
            SUT::from_phrase(phrase).unwrap()
        );
    }

    /// Test vector: https://github.com/radixdlt/babylon-wallet-ios/blob/99161cbbb11a78f36db6991e5d5c5f092678d5fa/RadixWalletTests/CryptographyTests/SLIP10Tests/TestVectors/cap26_curve25519.json#L8
    #[test]
    fn derive_a_curve25519_key_with_cap26() {
        let mwp = SUT::with_passphrase(
            Mnemonic::from_phrase(
                "equip will roof matter pink blind book anxiety banner elbow sun young",
            )
            .unwrap(),
            BIP39Passphrase::default(),
        );
        let seed = mwp.to_seed();
        let private_key = seed.derive_private_key(
            &AccountPath::from_str("m/44H/1022H/12H/525H/1460H/0H").unwrap(),
        );

        assert_eq!(
            "13e971fb16cb2c816d6b9f12176e9b8ab9af1831d006114d344d119ab2715506",
            private_key.to_hex()
        );
        assert_eq!(
            "451152a1cef7be603205086d4ebac0a0b78fda2ff4684b9dea5ca9ef003d4e7d",
            private_key.public_key().to_hex()
        );
    }

    /// Test vector: https://github.com/radixdlt/babylon-wallet-ios/blob/99161cbbb11a78f36db6991e5d5c5f092678d5fa/RadixWalletTests/CryptographyTests/SLIP10Tests/TestVectors/bip44_secp256k1.json#L288
    #[test]
    fn derive_a_secp256k1_key_with_bip44_olympia() {
        let mwp = SUT::with_passphrase(
            Mnemonic::from_phrase(
     "habit special recipe upon giraffe manual evil badge dwarf welcome inspire shrug post arrive van",
            )
            .unwrap(),
            BIP39Passphrase::default(),
        );
        let seed = mwp.to_seed();
        let private_key = seed.derive_private_key(
            &BIP44LikePath::from_str("m/44H/1022H/0H/0/5H").unwrap(),
        );

        assert_eq!(
            "111323d507d9d690836798e3ef2e5292cfd31092b75b9b59fa584ff593a3d7e4",
            private_key.to_hex()
        );

        assert_eq!(
            "03e78cdb2e0b7ea6e55e121a58560ccf841a913d3a4a9b8349e0ef00c2102f48d8",
            private_key.public_key().to_hex()
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = SUT::sample();

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "mnemonic": "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate",
                "passphrase": "radix"
            }
            "#,
        );
    }

    #[test]
    fn keys_for_sample() {
        let mwp = SUT::sample();
        let path = AccountPath::new(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            0,
        );
        let seed = mwp.to_seed();
        let private_key = seed.derive_private_key(&path);

        assert_eq!(path.to_string(), "m/44H/1022H/1H/525H/1460H/0H");

        assert_eq!(
            "cf52dbc7bb2663223e99fb31799281b813b939440a372d0aa92eb5f5b8516003",
            private_key.to_hex()
        );
        assert_eq!(
            "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b",
            private_key.public_key().to_hex()
        );
    }

    #[test]
    fn hash() {
        let n = 100;
        let set = (0..n).map(|_| SUT::generate_new()).collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn zeroize() {
        let mut sut = SUT::sample();
        sut.zeroize();
        assert_ne!(sut, SUT::sample());
    }
}
