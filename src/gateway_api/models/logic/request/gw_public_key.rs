use crate::prelude::*;

impl From<PublicKey> for GWPublicKey {
    fn from(value: PublicKey) -> Self {
        match value {
            PublicKey::Ed25519(key) => Self::Ed25519(key),
            PublicKey::Secp256k1(key) => Self::Secp256k1(key),
        }
    }
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
            Err(CommonError::InvalidEd25519PublicKeyFromString { bad_value: wrapper.key_hex }).map_err(de::Error::custom)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = GWPublicKey;

    #[test]
    fn json_roundtrip_ed25519() {
        let json = r#"
        {
			"key_hex": "d1b96e7cce2a5267a4a638c4b77fa7c33d220fa27a42c6f38136067d667353d0",
			"key_type": "EddsaEd25519"
		}
        "#;
        let sut = SUT::Ed25519(Ed25519PublicKey::from_str("d1b96e7cce2a5267a4a638c4b77fa7c33d220fa27a42c6f38136067d667353d0").unwrap());
        assert_eq_after_json_roundtrip(&sut, json);
        assert_json_roundtrip(&sut);
    }

    #[test]
    fn json_roundtrip_secp256k1() {
        let json = r#"
        {
			"key_hex": "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7",
			"key_type": "EcdsaSecp256k1"
		}
        "#;
        let sut = SUT::Secp256k1(Secp256k1PublicKey::from_str("02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7").unwrap());
        assert_eq_after_json_roundtrip(&sut, json);
        assert_json_roundtrip(&sut);
    }

    #[test]
    fn invalid_json_secp256k1() {
        assert_json_fails::<SUT>(
            r#"
        {
			"key_hex": "not even hex string, super fail!",
			"key_type": "EcdsaSecp256k1"
		}
        "#,
        )
    }

    #[test]
    fn invalid_json_ed25519() {
        assert_json_fails::<SUT>(
            r#"
        {
			"key_hex": "not even hex string, super fail!",
			"key_type": "EddsaEd25519"
		}
        "#,
        )
    }

    #[test]
    fn from_secp256k1() {
        let secp256k1_key = Secp256k1PublicKey::sample_alice();
        let public_key = PublicKey::from(secp256k1_key);
        let sut = SUT::from(public_key);
        assert_eq!(sut.as_secp256k1().unwrap(), &secp256k1_key);
    }

    #[test]
    fn from_ed25519() {
        let ed25519_key = Ed25519PublicKey::sample_alice();
        let public_key = PublicKey::from(ed25519_key);
        let sut = SUT::from(public_key);
        assert_eq!(sut.as_ed25519().unwrap(), &ed25519_key);
    }
}
