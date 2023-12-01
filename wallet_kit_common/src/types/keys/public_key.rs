use serde::{de, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

use crate::error::Error;

use super::{
    ed25519::public_key::Ed25519PublicKey, secp256k1::public_key::Secp256k1PublicKey,
    slip10_curve::SLIP10Curve,
};
use enum_as_inner::EnumAsInner;
#[derive(Clone, Copy, Debug, PartialEq, EnumAsInner, Eq, Hash, PartialOrd, Ord)]
/// A tagged union of supported public keys on different curves, supported
/// curves are `secp256k1` and `Curve25519`
pub enum PublicKey {
    /// An Ed25519 public key used to verify cryptographic signatures.
    Ed25519(Ed25519PublicKey),
    Secp256k1(Secp256k1PublicKey),
}

impl From<Ed25519PublicKey> for PublicKey {
    fn from(value: Ed25519PublicKey) -> Self {
        Self::Ed25519(value)
    }
}

impl From<Secp256k1PublicKey> for PublicKey {
    fn from(value: Secp256k1PublicKey) -> Self {
        Self::Secp256k1(value)
    }
}

impl PublicKey {
    pub fn secp256k1_from_bytes(slice: &[u8]) -> Result<Self, Error> {
        Secp256k1PublicKey::try_from(slice).map(Self::Secp256k1)
    }

    pub fn ed25519_from_bytes(slice: &[u8]) -> Result<Self, Error> {
        Ed25519PublicKey::try_from(slice).map(Self::Ed25519)
    }

    pub fn secp256k1_from_str(hex: &str) -> Result<Self, Error> {
        Secp256k1PublicKey::from_str(hex).map(Self::Secp256k1)
    }

    pub fn ed25519_from_str(hex: &str) -> Result<Self, Error> {
        Ed25519PublicKey::from_str(hex).map(Self::Ed25519)
    }
}

impl PublicKey {
    pub fn curve(&self) -> SLIP10Curve {
        match self {
            PublicKey::Ed25519(_) => SLIP10Curve::Curve25519,
            PublicKey::Secp256k1(_) => SLIP10Curve::Secp256k1,
        }
    }

    pub fn to_hex(&self) -> String {
        match self {
            PublicKey::Ed25519(key) => key.to_hex(),
            PublicKey::Secp256k1(key) => key.to_hex(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            PublicKey::Ed25519(key) => key.to_bytes(),
            PublicKey::Secp256k1(key) => key.to_bytes(),
        }
    }
}

impl PublicKey {
    pub fn placeholder_secp256k1() -> Self {
        Self::placeholder_secp256k1_alice()
    }

    pub fn placeholder_secp256k1_alice() -> Self {
        Self::Secp256k1(Secp256k1PublicKey::placeholder_alice())
    }

    pub fn placeholder_secp256k1_bob() -> Self {
        Self::Secp256k1(Secp256k1PublicKey::placeholder_bob())
    }

    pub fn placeholder_ed25519() -> Self {
        Self::placeholder_ed25519_alice()
    }

    pub fn placeholder_ed25519_alice() -> Self {
        Self::Ed25519(Ed25519PublicKey::placeholder_alice())
    }

    pub fn placeholder_ed25519_bob() -> Self {
        Self::Ed25519(Ed25519PublicKey::placeholder_bob())
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(rename = "compressedData")]
            hex: String,
            curve: SLIP10Curve,
        }
        let wrapper = Wrapper::deserialize(deserializer)?;
        match wrapper.curve {
            SLIP10Curve::Curve25519 => Ed25519PublicKey::from_str(&wrapper.hex)
                .map(|pk| PublicKey::Ed25519(pk))
                .map_err(de::Error::custom),
            SLIP10Curve::Secp256k1 => Secp256k1PublicKey::from_str(&wrapper.hex)
                .map(|pk| PublicKey::Secp256k1(pk))
                .map_err(de::Error::custom),
        }
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

#[cfg(test)]
mod tests {

    use std::collections::BTreeSet;

    use crate::json::assert_eq_after_json_roundtrip;

    use super::PublicKey;

    #[test]
    fn json_roundtrip_ed25519() {
        let model = PublicKey::placeholder_ed25519_alice();

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
    fn json_roundtrip_secp256k1() {
        let model = PublicKey::placeholder_secp256k1_alice();

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
    fn inequality_secp256k1() {
        assert_ne!(
            PublicKey::placeholder_secp256k1_alice(),
            PublicKey::placeholder_secp256k1_bob(),
        );
    }

    #[test]
    fn equality_secp256k1() {
        assert_eq!(
            PublicKey::placeholder_secp256k1(),
            PublicKey::placeholder_secp256k1_alice()
        );
    }

    #[test]
    fn hash_secp256k1() {
        assert_eq!(
            BTreeSet::from_iter([
                PublicKey::placeholder_secp256k1_alice(),
                PublicKey::placeholder_secp256k1_alice()
            ])
            .len(),
            1
        );
    }

    #[test]
    fn inequality_ed25519() {
        assert_ne!(
            PublicKey::placeholder_ed25519_alice(),
            PublicKey::placeholder_ed25519_bob(),
        );
    }

    #[test]
    fn equality_ed25519() {
        assert_eq!(
            PublicKey::placeholder_ed25519(),
            PublicKey::placeholder_ed25519_alice()
        );
    }

    #[test]
    fn hash_ed25519() {
        assert_eq!(
            BTreeSet::from_iter([
                PublicKey::placeholder_ed25519_alice(),
                PublicKey::placeholder_ed25519_alice()
            ])
            .len(),
            1
        );
    }

    #[test]
    fn inequality_different_curves() {
        assert_ne!(
            PublicKey::placeholder_ed25519_alice(),
            PublicKey::placeholder_secp256k1_alice(),
        );
    }

    #[test]
    fn secp256k1_bytes_roundtrip() {
        let bytes: &[u8] = &[
            0x02, 0x51, 0x7b, 0x88, 0x91, 0x6e, 0x7f, 0x31, 0x5b, 0xb6, 0x82, 0xf9, 0x92, 0x6b,
            0x14, 0xbc, 0x67, 0xa0, 0xe4, 0x24, 0x6f, 0x8a, 0x41, 0x9b, 0x98, 0x62, 0x69, 0xe1,
            0xa7, 0xe6, 0x1f, 0xff, 0xa7,
        ];
        let key = PublicKey::secp256k1_from_bytes(bytes).unwrap();
        assert_eq!(
            key.to_hex(),
            "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
        );
        assert_eq!(key.to_bytes(), bytes);
    }

    #[test]
    fn secp256k1_hex_roundtrip() {
        let hex = "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7";
        let key = PublicKey::secp256k1_from_str(hex).unwrap();
        assert_eq!(key.to_hex(), hex);
    }

    #[test]
    fn ed25519_bytes_roundtrip() {
        let bytes: &[u8] = &[
            0xec, 0x17, 0x2b, 0x93, 0xad, 0x5e, 0x56, 0x3b, 0xf4, 0x93, 0x2c, 0x70, 0xe1, 0x24,
            0x50, 0x34, 0xc3, 0x54, 0x67, 0xef, 0x2e, 0xfd, 0x4d, 0x64, 0xeb, 0xf8, 0x19, 0x68,
            0x34, 0x67, 0xe2, 0xbf,
        ];
        let key = PublicKey::ed25519_from_bytes(bytes).unwrap();
        assert_eq!(
            key.to_hex(),
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
        );
        assert_eq!(key.to_bytes(), bytes);
    }

    #[test]
    fn ed25519_hex_roundtrip() {
        let hex = "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf";
        let key = PublicKey::ed25519_from_str(hex).unwrap();
        assert_eq!(key.to_hex(), hex);
    }
}
