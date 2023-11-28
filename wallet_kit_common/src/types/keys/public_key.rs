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

    use crate::json::assert_eq_after_json_roundtrip;

    use super::PublicKey;

    #[test]
    fn json_roundtrip_ed25519() {
        let model = PublicKey::Ed25519(
            "3feb8194ead2e526fbcc4c1673a7a8b29d8cee0b32bb9393692f739821dd256b"
                .try_into()
                .unwrap(),
        );

        assert_eq_after_json_roundtrip(
            &model,
            r#"
			{
				"curve": "curve25519",
				"compressedData": "3feb8194ead2e526fbcc4c1673a7a8b29d8cee0b32bb9393692f739821dd256b"
			}
            "#,
        );
    }

    #[test]
    fn json_roundtrip_secp256k1() {
        let model = PublicKey::Secp256k1(
            "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
                .try_into()
                .unwrap(),
        );

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
}
