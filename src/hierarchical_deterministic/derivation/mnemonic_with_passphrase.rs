use crate::prelude::*;

/// A BIP39 Mnemonic and BIP39 passphrase - aka "25th word" tuple,
/// from which we can derive a HD Root used for derivation.
#[derive(
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

pub type PrivateKeyBytes = [u8; 32];

use crypto::{
    keys::slip10::{self as IotaSlip10, Hardened as IotaSlip10PathComponent},
    signatures::ed25519 as IotaSlip10Ed25519,
    signatures::secp256k1_ecdsa as IotaSlip10Secp256k1,
};

impl HDPath {
    fn hardened_chain(&self) -> Vec<IotaSlip10PathComponent> {
        self.components
            .iter()
            .map(|c| c.value)
            .map(|v| IotaSlip10PathComponent::try_from(v).expect("Should work"))
            .collect_vec()
    }
}

impl MnemonicWithPassphrase {
    pub fn to_seed(&self) -> Seed {
        self.mnemonic.to_seed(&self.passphrase.0)
    }

    fn derive_slip10_private_key<K, I>(
        seed: &Seed,
        chain: I,
    ) -> IotaSlip10::Slip10<K>
    where
        K: IotaSlip10::IsSecretKey
            + IotaSlip10::WithSegment<<I as Iterator>::Item>,
        I: Iterator,
        <I as Iterator>::Item: IotaSlip10::Segment,
    {
        let seed = IotaSlip10::Seed::from_bytes(seed);
        seed.derive(chain)
    }

    pub fn derive_ed25519_private_key(
        seed: &Seed,
        path: &HDPath,
    ) -> Ed25519PrivateKey {
        let ck = Self::derive_slip10_private_key::<
            IotaSlip10Ed25519::SecretKey,
            _,
        >(seed, path.hardened_chain().into_iter());
        Ed25519PrivateKey::from_bytes(ck.secret_key().as_slice())
            .expect("Valid Ed25519PrivateKey bytes")
    }

    pub fn derive_secp256k1_private_key(
        seed: &Seed,
        path: &HDPath,
    ) -> Secp256k1PrivateKey {
        let ck = Self::derive_slip10_private_key::<
            IotaSlip10Secp256k1::SecretKey,
            _,
        >(
            seed, path.components.iter().cloned().map(|c| c.value)
        );
        Secp256k1PrivateKey::from_bytes(&*ck.secret_key().to_bytes())
            .expect("Valid Secp256k1PrivateKey bytes")
    }

    #[cfg(not(tarpaulin_include))] // false negative
    pub fn derive_private_key<D>(
        &self,
        derivation: D,
    ) -> HierarchicalDeterministicPrivateKey
    where
        D: Derivation,
    {
        let seed = self.to_seed();
        let path = derivation.derivation_path();
        match derivation.scheme() {
            DerivationPathScheme::Cap26 => {
                assert_eq!(
                    derivation.scheme().curve(),
                    SLIP10Curve::Curve25519
                );
                let key =
                    Self::derive_ed25519_private_key(&seed, path.hd_path());
                HierarchicalDeterministicPrivateKey::new(key.into(), path)
            }
            DerivationPathScheme::Bip44Olympia => {
                assert_eq!(derivation.scheme().curve(), SLIP10Curve::Secp256k1);
                let key =
                    Self::derive_secp256k1_private_key(&seed, path.hd_path());
                HierarchicalDeterministicPrivateKey::new(key.into(), path)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            MnemonicWithPassphrase::sample(),
            MnemonicWithPassphrase::sample()
        );
        assert_eq!(
            MnemonicWithPassphrase::sample_other(),
            MnemonicWithPassphrase::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            MnemonicWithPassphrase::sample(),
            MnemonicWithPassphrase::sample_other()
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", MnemonicWithPassphrase::sample()),
            "<OBFUSCATED>"
        );
    }

    #[test]
    fn debug() {
        assert_eq!(
            format!("{:?}", MnemonicWithPassphrase::sample()),
            format!("{:?}", "24 words (bright...mandate) + <NOT EMPTY>")
        );
        assert_eq!(
            format!("{:?}", MnemonicWithPassphrase::sample_other()),
            format!("{:?}", "12 words (zoo...wrong) + <EMPTY>")
        );
    }

    #[test]
    fn non_sensitive() {
        assert_eq!(
            format!("{:?}", MnemonicWithPassphrase::sample().non_sensitive()),
            format!("{:?}", "24 words (bright...mandate) + <NOT EMPTY>")
        );
        assert_eq!(
            format!(
                "{:?}",
                MnemonicWithPassphrase::sample_other().non_sensitive()
            ),
            format!("{:?}", "12 words (zoo...wrong) + <EMPTY>")
        );
    }

    #[test]
    fn with_passphrase() {
        let phrase = "equip will roof matter pink blind book anxiety banner elbow sun young";
        let passphrase = "25th";
        let mwp = MnemonicWithPassphrase::with_passphrase(
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
            MnemonicWithPassphrase::new(Mnemonic::from_phrase(phrase).unwrap()),
            MnemonicWithPassphrase::from_phrase(phrase).unwrap()
        );
    }

    /// Test vector: https://github.com/radixdlt/babylon-wallet-ios/blob/99161cbbb11a78f36db6991e5d5c5f092678d5fa/RadixWalletTests/CryptographyTests/SLIP10Tests/TestVectors/cap26_curve25519.json#L8
    #[test]
    fn derive_a_curve25519_key_with_cap26() {
        let mwp = MnemonicWithPassphrase::with_passphrase(
            Mnemonic::from_phrase(
                "equip will roof matter pink blind book anxiety banner elbow sun young",
            )
            .unwrap(),
            BIP39Passphrase::default(),
        );

        let private_key = mwp.derive_private_key(
            AccountPath::from_str("m/44H/1022H/12H/525H/1460H/0H").unwrap(),
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
        let mwp = MnemonicWithPassphrase::with_passphrase(
            Mnemonic::from_phrase(
     "habit special recipe upon giraffe manual evil badge dwarf welcome inspire shrug post arrive van",
            )
            .unwrap(),
            BIP39Passphrase::default(),
        );

        let private_key = mwp.derive_private_key(
            BIP44LikePath::from_str("m/44H/1022H/0H/0/5H").unwrap(),
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
        let model = MnemonicWithPassphrase::with_passphrase(
            Mnemonic::from_phrase(
     "habit special recipe upon giraffe manual evil badge dwarf welcome inspire shrug post arrive van",
            )
            .unwrap(),
            "25th".into(),
        );

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "mnemonic": "habit special recipe upon giraffe manual evil badge dwarf welcome inspire shrug post arrive van",
                "passphrase": "25th"
            }
            "#,
        );
    }

    #[test]
    fn keys_for_sample() {
        let mwp = MnemonicWithPassphrase::sample();
        let path = AccountPath::new(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            0,
        );
        let private_key = mwp.derive_private_key(path.clone());

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
        let set = (0..n)
            .map(|_| MnemonicWithPassphrase::generate_new())
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }
}
