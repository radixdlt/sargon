use serde::{de, ser::SerializeStruct, Deserializer, Serializer};

use crate::prelude::*;

/// A tagged union of supported public keys on different curves, supported
/// curves are `secp256k1` and `Curve25519`
#[derive(
    Clone,
    Debug,
    Copy,
    PartialEq,
    EnumAsInner,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
)]
pub enum PublicKey {
    /// An Ed25519 public key used to verify cryptographic signatures.
    Ed25519(Ed25519PublicKey),

    /// A secp256k1 public key used to verify cryptographic signatures (ECDSA signatures).
    Secp256k1(Secp256k1PublicKey),
}

impl TryFrom<Vec<u8>> for PublicKey {
    type Error = crate::CommonError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ed25519PublicKey::try_from(value.clone())
            .map(Self::from)
            .or(Secp256k1PublicKey::try_from(value).map(Self::from))
    }
}

impl TryFrom<BagOfBytes> for PublicKey {
    type Error = crate::CommonError;

    fn try_from(value: BagOfBytes) -> Result<Self, Self::Error> {
        Self::try_from(value.to_vec())
    }
}

impl FromStr for PublicKey {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ed25519PublicKey::from_str(s)
            .map(Self::from)
            .or(Secp256k1PublicKey::from_str(s).map(Self::from))
    }
}

impl PublicKey {
    /// Verifies an Elliptic Curve signature over either Curve25519 or Secp256k1
    pub fn is_valid_signature_for_hash(
        &self,
        signature: impl Into<Signature>,
        hash: &impl ScryptoIsHash,
    ) -> bool {
        let signature = signature.into();
        match self {
            PublicKey::Ed25519(key) => match &signature {
                Signature::Secp256k1 { value: _ } => {
                    error!("Trying to validate Secp256k1 signature with Ed25519 PublicKey, that does not work.");
                    false
                }
                Signature::Ed25519 { value: signature } => {
                    key.is_valid_signature_for_hash(signature, hash)
                }
            },
            PublicKey::Secp256k1(key) => match &signature {
                Signature::Secp256k1 { value: signature } => {
                    key.is_valid_signature_for_hash(signature, hash)
                }
                Signature::Ed25519 { value: _ } => {
                    error!("Trying to validate Ed25519 signature with Secp256k1 PublicKey, that does not work.");
                    false
                }
            },
        }
    }
}

impl From<Ed25519PublicKey> for PublicKey {
    /// Enables:
    ///
    /// ```
    /// use ecc::prelude::*;
    ///
    /// let key: PublicKey = Ed25519PrivateKey::generate().public_key().into();
    /// ```
    fn from(value: Ed25519PublicKey) -> Self {
        Self::Ed25519(value)
    }
}

impl From<Secp256k1PublicKey> for PublicKey {
    /// Enables:
    ///
    /// ```
    /// use ecc::prelude::*;
    ///
    /// let key: PublicKey = Secp256k1PrivateKey::generate().public_key().into();
    /// ```
    fn from(value: Secp256k1PublicKey) -> Self {
        Self::Secp256k1(value)
    }
}

impl PublicKey {
    /// Try to instantiate a `PublicKey` from bytes as a `Secp256k1PublicKey`.
    pub fn secp256k1_from_bytes(slice: &[u8]) -> Result<Self> {
        Secp256k1PublicKey::try_from(slice).map(|k| k.into())
    }

    /// Try to instantiate a `PublicKey` from bytes as a `Ed25519PublicKey`.
    pub fn ed25519_from_bytes(slice: &[u8]) -> Result<Self> {
        Ed25519PublicKey::try_from(slice).map(|k| k.into())
    }

    /// Try to instantiate a `PublicKey` from hex string as a `Secp256k1PublicKey`.
    pub fn secp256k1_from_str(hex: &str) -> Result<Self> {
        Secp256k1PublicKey::from_str(hex).map(|k| k.into())
    }

    /// Try to instantiate a `PublicKey` from hex string as a `Ed25519PublicKey`.
    pub fn ed25519_from_str(hex: &str) -> Result<Self> {
        Ed25519PublicKey::from_str(hex).map(|k| k.into())
    }
}

impl PublicKey {
    /// Returns a `SLIP10Curve`, being the curve of the `PublicKey`.
    pub fn curve(&self) -> SLIP10Curve {
        match self {
            PublicKey::Ed25519(_) => SLIP10Curve::Curve25519,
            PublicKey::Secp256k1(_) => SLIP10Curve::Secp256k1,
        }
    }

    /// Returns a hex encoding of the inner public key.
    pub fn to_hex(&self) -> String {
        match self {
            PublicKey::Ed25519(key) => key.to_hex(),
            PublicKey::Secp256k1(key) => key.to_hex(),
        }
    }

    /// Returns a clone of the bytes of the inner public key as a `Vec`.
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            PublicKey::Ed25519(key) => key.to_bytes(),
            PublicKey::Secp256k1(key) => key.to_bytes(),
        }
    }
}

impl HasSampleValues for PublicKey {
    fn sample() -> Self {
        Self::sample_ed25519_alice()
    }

    fn sample_other() -> Self {
        Self::sample_secp256k1_bob()
    }
}

impl PublicKey {
    /// A sample used to facilitate unit tests.
    pub fn sample_secp256k1() -> Self {
        Self::sample_secp256k1_alice()
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_secp256k1_alice() -> Self {
        Self::Secp256k1(Secp256k1PublicKey::sample_alice())
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_secp256k1_bob() -> Self {
        Self::Secp256k1(Secp256k1PublicKey::sample_bob())
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_ed25519() -> Self {
        Self::sample_ed25519_alice()
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_ed25519_alice() -> Self {
        Self::Ed25519(Ed25519PublicKey::sample_alice())
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_ed25519_bob() -> Self {
        Self::Ed25519(Ed25519PublicKey::sample_bob())
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(rename = "compressedData")]
            hex: String,
            curve: SLIP10Curve,
        }
        let wrapper = Wrapper::deserialize(deserializer)?;
        let public_key = match wrapper.curve {
            SLIP10Curve::Curve25519 => {
                match Ed25519PublicKey::from_str(&wrapper.hex) {
                    Ok(value) => PublicKey::Ed25519(value),
                    Err(_) => {
                        match Secp256k1PublicKey::from_str(&wrapper.hex) {
                            Ok(value) => PublicKey::Secp256k1(value),
                            Err(e) => Err(serde::de::Error::custom(e))?,
                        }
                    }
                }
            }
            SLIP10Curve::Secp256k1 => PublicKey::Secp256k1(
                Secp256k1PublicKey::from_str(&wrapper.hex)
                    .map_err(de::Error::custom)?,
            ),
        };
        Ok(public_key)
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PublicKey", 2)?;
        state.serialize_field("curve", &self.curve())?;
        state.serialize_field("compressedData", &self.to_hex())?;
        state.end()
    }
}

impl From<PublicKey> for ScryptoPublicKey {
    fn from(value: PublicKey) -> Self {
        match value {
            PublicKey::Ed25519(key) => Self::Ed25519(key.into()),
            PublicKey::Secp256k1(key) => Self::Secp256k1(key.into()),
        }
    }
}

impl TryFrom<ScryptoPublicKey> for PublicKey {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoPublicKey) -> Result<Self, Self::Error> {
        match value {
            ScryptoPublicKey::Secp256k1(key) => {
                Secp256k1PublicKey::try_from(key).map(|k| k.into())
            }
            ScryptoPublicKey::Ed25519(key) => {
                Ed25519PublicKey::try_from(key).map(|k| k.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PublicKey;

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
    fn engine_roundtrip_secp256k1() {
        let public_key_secp256k1: PublicKey =
            Secp256k1PublicKey::sample().into();
        let engine_key_secp256k1: ScryptoPublicKey =
            public_key_secp256k1.into();
        match engine_key_secp256k1 {
            ScryptoPublicKey::Secp256k1(k) => {
                assert_eq!(k.to_vec(), public_key_secp256k1.to_bytes())
            }
            ScryptoPublicKey::Ed25519(_) => panic!("wrong kind"),
        }
    }

    #[test]
    fn engine_roundtrip_ed25519() {
        let public_key_ed25519: PublicKey = Ed25519PublicKey::sample().into();
        let engine_key_ed25519: ScryptoPublicKey = public_key_ed25519.into();
        match engine_key_ed25519 {
            ScryptoPublicKey::Ed25519(k) => {
                assert_eq!(k.to_vec(), public_key_ed25519.to_bytes())
            }
            ScryptoPublicKey::Secp256k1(_) => panic!("wrong kind"),
        }
    }

    #[test]
    fn display_secp256k1() {
        assert_eq!(format!("{}", SUT::sample_secp256k1()), "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7");
    }

    #[test]
    fn display_ed25519() {
        assert_eq!(
            format!("{}", SUT::sample_ed25519()),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        );
    }

    #[test]
    fn debug_secp256k1() {
        assert_eq!(format!("{:?}", SUT::sample_secp256k1()), "Secp256k1(02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7)");
    }

    #[test]
    fn debug_ed25519() {
        assert_eq!(
            format!("{:?}", SUT::sample_ed25519()),
            "Ed25519(ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf)"
        );
    }

    #[test]
    fn json_roundtrip_ed25519() {
        let model = SUT::sample_ed25519_alice();

        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
				"curve": "curve25519",
				"compressedData": "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
			}
            "#,
        );
    }

    #[test]
    fn json_invalid_curve() {
        assert_json_fails::<PublicKey>(
            r#"
			{
				"curve": "invalid curve",
				"compressedData": "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
			}
            "#,
        );
    }

    #[test]
    fn json_invalid_public_key_not_on_curve() {
        assert_json_fails::<PublicKey>(
            r#"
			{
				"curve": "curve25519",
				"compressedData": "abbaabbaabbaabbaabbaabbaabbaabbaabbaabbaabbaabbaabbaabbaabbaabba"
			}
            "#,
        );
    }

    #[test]
    fn json_roundtrip_secp256k1() {
        let model = SUT::sample_secp256k1_alice();

        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
				"curve": "secp256k1",
				"compressedData": "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
			}
            "#,
        );
    }

    #[test]
    fn json_android_bug_secp256k1_incorrectly_marked_as_25519_is_indeed_deserialized_as_secp256k1(
    ) {
        let expected_secp256k1 = SUT::sample_secp256k1_alice();
        let json_with_wrong_curve = r#"
        {
            "curve": "curve25519",
            "compressedData": "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
        }
        "#;

        let result =
            serde_json::from_str::<PublicKey>(json_with_wrong_curve).unwrap();
        assert_eq!(expected_secp256k1, result);
    }

    #[test]
    fn inequality_secp256k1() {
        assert_ne!(SUT::sample_secp256k1_alice(), SUT::sample_secp256k1_bob(),);
    }

    #[test]
    fn equality_secp256k1() {
        assert_eq!(SUT::sample_secp256k1(), SUT::sample_secp256k1_alice());
    }

    #[test]
    fn hash_secp256k1() {
        assert_eq!(
            BTreeSet::from_iter([
                SUT::sample_secp256k1_alice(),
                SUT::sample_secp256k1_alice()
            ])
            .len(),
            1
        );
    }

    #[test]
    fn inequality_ed25519() {
        assert_ne!(SUT::sample_ed25519_alice(), SUT::sample_ed25519_bob(),);
    }

    #[test]
    fn equality_ed25519() {
        assert_eq!(SUT::sample_ed25519(), SUT::sample_ed25519_alice());
    }

    #[test]
    fn hash_ed25519() {
        assert_eq!(
            BTreeSet::from_iter([
                SUT::sample_ed25519_alice(),
                SUT::sample_ed25519_alice()
            ])
            .len(),
            1
        );
    }

    #[test]
    fn inequality_different_curves() {
        assert_ne!(SUT::sample_ed25519_alice(), SUT::sample_secp256k1_alice(),);
    }

    #[test]
    fn secp256k1_bytes_roundtrip() {
        let bytes: &[u8] = &[
            0x02, 0x51, 0x7b, 0x88, 0x91, 0x6e, 0x7f, 0x31, 0x5b, 0xb6, 0x82,
            0xf9, 0x92, 0x6b, 0x14, 0xbc, 0x67, 0xa0, 0xe4, 0x24, 0x6f, 0x8a,
            0x41, 0x9b, 0x98, 0x62, 0x69, 0xe1, 0xa7, 0xe6, 0x1f, 0xff, 0xa7,
        ];
        let key = SUT::secp256k1_from_bytes(bytes).unwrap();
        assert_eq!(
            key.to_hex(),
            "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
        );
        assert_eq!(key.to_bytes(), bytes);
    }

    #[test]
    fn secp256k1_hex_roundtrip() {
        let hex = "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7";
        let key = SUT::secp256k1_from_str(hex).unwrap();
        assert_eq!(key.to_hex(), hex);
    }

    #[test]
    fn from_str_ed25519() {
        assert_eq!(SUT::from_str("ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf").unwrap(), SUT::Ed25519(Ed25519PublicKey::sample()))
    }

    #[test]
    fn from_str_secp256k1() {
        assert_eq!(SUT::from_str("02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7").unwrap(), SUT::Secp256k1(Secp256k1PublicKey::sample()))
    }

    #[test]
    fn try_from_bag_of_bytes_ed25519() {
        let bag_of_bytes: BagOfBytes =
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
                .parse()
                .unwrap();
        assert_eq!(
            SUT::try_from(bag_of_bytes).unwrap(),
            SUT::Ed25519(Ed25519PublicKey::sample())
        )
    }

    #[test]
    fn try_from_bag_of_bytes_secp256k1() {
        let bag_of_bytes: BagOfBytes = "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7".parse().unwrap();
        assert_eq!(
            SUT::try_from(bag_of_bytes).unwrap(),
            SUT::Secp256k1(Secp256k1PublicKey::sample())
        )
    }

    #[test]
    fn ed25519_bytes_roundtrip() {
        let bytes: &[u8] = &[
            0xec, 0x17, 0x2b, 0x93, 0xad, 0x5e, 0x56, 0x3b, 0xf4, 0x93, 0x2c,
            0x70, 0xe1, 0x24, 0x50, 0x34, 0xc3, 0x54, 0x67, 0xef, 0x2e, 0xfd,
            0x4d, 0x64, 0xeb, 0xf8, 0x19, 0x68, 0x34, 0x67, 0xe2, 0xbf,
        ];
        let key = SUT::ed25519_from_bytes(bytes).unwrap();
        assert_eq!(
            key.to_hex(),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        );
        assert_eq!(key.to_bytes(), bytes);
    }

    #[test]
    fn ed25519_hex_roundtrip() {
        let hex =
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf";
        let key = SUT::ed25519_from_str(hex).unwrap();
        assert_eq!(key.to_hex(), hex);
    }

    #[test]
    fn ed25519_into_as_roundtrip() {
        let ed25519 = Ed25519PublicKey::sample();
        let key: PublicKey = ed25519.into();
        assert_eq!(key.as_ed25519().unwrap(), &ed25519);
    }

    #[test]
    fn secp256k1_into_as_roundtrip() {
        let secp256k1 = Secp256k1PublicKey::sample();
        let key: PublicKey = secp256k1.into();
        assert_eq!(key.as_secp256k1().unwrap(), &secp256k1);
    }

    #[test]
    fn is_valid_with_secp256k1() {
        let secp256k1_public_key: Secp256k1PublicKey =
        "02f0d85a3b9082683f689e6115f37e1e24b7448fff14b14877e3a4e750e86fba8b"
            .parse()
            .unwrap();

        let message = "All those moments will be lost in time, like tears in rain. Time to die...";

        let hash = hash_of(message.as_bytes());
        let secp256k1_signature: Secp256k1Signature = "01aa1c4f46f8437b7f8ec9008ae10e6f33bb8be3e81e35c63f3498070dfbd6a20b2daee6073ead3c9e72d8909bc32a02e46cede3885cf8568d4c380ac97aa7fbcd".parse().unwrap();

        assert!(SUT::Secp256k1(secp256k1_public_key)
            .is_valid_signature_for_hash(
                Signature::Secp256k1 {
                    value: secp256k1_signature
                },
                &hash
            ));
    }

    #[test]
    fn is_valid_with_ed25519() {
        let message = "All those moments will be lost in time, like tears in rain. Time to die...";
        let hash = hash_of(message.as_bytes());

        let ed25519_public_key: Ed25519PublicKey =
            "08740a2fd178c40ce71966a6537f780978f7f00548cfb59196344b5d7d67e9cf"
                .parse()
                .unwrap();

        let ed25519_signature: Ed25519Signature = "06cd3772c5c70d44819db80192a5b2521525e2529f770bff970ec4edc7c1bd76e41fcfa8e59ff93b1675c48f4af3b1697765286d999ee8b5bb8257691e3b7b09".parse().unwrap();

        assert!(
            SUT::Ed25519(ed25519_public_key).is_valid_signature_for_hash(
                Signature::Ed25519 {
                    value: ed25519_signature
                },
                &hash
            )
        );
    }

    #[test]
    fn is_valid_with_ed25519_public_key_for_secp256k1_signature_fails() {
        let message = "All those moments will be lost in time, like tears in rain. Time to die...";

        let hash = hash_of(message.as_bytes());
        let secp256k1_signature: Secp256k1Signature = "01aa1c4f46f8437b7f8ec9008ae10e6f33bb8be3e81e35c63f3498070dfbd6a20b2daee6073ead3c9e72d8909bc32a02e46cede3885cf8568d4c380ac97aa7fbcd".parse().unwrap();

        let ed25519_public_key: Ed25519PublicKey =
            "08740a2fd178c40ce71966a6537f780978f7f00548cfb59196344b5d7d67e9cf"
                .parse()
                .unwrap();

        assert!(
            !SUT::Ed25519(ed25519_public_key).is_valid_signature_for_hash(
                Signature::Secp256k1 {
                    value: secp256k1_signature
                },
                &hash
            )
        );
    }

    #[test]
    fn is_valid_with_secp256k1_public_key_for_ed25519_signature_fails() {
        let secp256k1_public_key: Secp256k1PublicKey =
        "02f0d85a3b9082683f689e6115f37e1e24b7448fff14b14877e3a4e750e86fba8b"
            .parse()
            .unwrap();

        let message = "All those moments will be lost in time, like tears in rain. Time to die...";

        let hash = hash_of(message.as_bytes());

        let ed25519_signature: Ed25519Signature = "06cd3772c5c70d44819db80192a5b2521525e2529f770bff970ec4edc7c1bd76e41fcfa8e59ff93b1675c48f4af3b1697765286d999ee8b5bb8257691e3b7b09".parse().unwrap();

        assert!(!SUT::Secp256k1(secp256k1_public_key)
            .is_valid_signature_for_hash(
                Signature::Ed25519 {
                    value: ed25519_signature
                },
                &hash
            ));
    }
}
