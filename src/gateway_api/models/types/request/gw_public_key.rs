use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub(crate) enum GWPublicKey {
    Secp256k1(Secp256k1PublicKey),
    Ed25519(Ed25519PublicKey),
}

impl<'de> Deserialize<'de> for GWPublicKey {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            key_type: String,
            key_hex: String,
        }
        let wrapper = Wrapper::deserialize(deserializer)?;
        if wrapper.key_type == GW_PUBLIC_KEY_JSON_KEY_TYPE_SECP256K1 {
            Secp256k1PublicKey::from_str(&wrapper.key_hex)
                .map(Self::Secp256k1)
                .map_err(de::Error::custom)
        } else if wrapper.key_type == GW_PUBLIC_KEY_JSON_KEY_TYPE_ED25519 {
            Ed25519PublicKey::from_str(&wrapper.key_hex)
                .map(Self::Ed25519)
                .map_err(de::Error::custom)
        } else {
            Err(CommonError::Unknown).map_err(de::Error::custom)
        }
    }
}
const GW_PUBLIC_KEY_JSON_KEY_TYPE_SECP256K1: &str = "EcdsaSecp256k1";
const GW_PUBLIC_KEY_JSON_KEY_TYPE_ED25519: &str = "EddsaEd25519";
impl Serialize for GWPublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GWPublicKey", 2)?;
        match self {
            Self::Secp256k1(key) => {
                state.serialize_field(
                    "key_type",
                    GW_PUBLIC_KEY_JSON_KEY_TYPE_SECP256K1,
                )?;
                state.serialize_field("key_hex", &key.to_hex())?;
            }
            Self::Ed25519(key) => {
                state.serialize_field(
                    "key_type",
                    GW_PUBLIC_KEY_JSON_KEY_TYPE_ED25519,
                )?;
                state.serialize_field("key_hex", &key.to_hex())?;
            }
        }
        state.end()
    }
}
