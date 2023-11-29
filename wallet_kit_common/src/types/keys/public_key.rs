use serde::{de, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

use super::{
    ed25519::public_key::Ed25519PublicKey, secp256k1::public_key::Secp256k1PublicKey,
    slip10_curve::SLIP10Curve,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PublicKey {
    Ed25519(Ed25519PublicKey),
    Secp256k1(Secp256k1PublicKey),
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
            PublicKey::placeholder_secp256k1_alice(),
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
            PublicKey::placeholder_ed25519_alice(),
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
}
