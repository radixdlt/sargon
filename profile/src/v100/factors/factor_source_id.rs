use super::{
    factor_source_id_from_address::FactorSourceIDFromAddress,
    factor_source_id_from_hash::FactorSourceIDFromHash,
};
use enum_as_inner::EnumAsInner;
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

/// A unique and stable identifier of a FactorSource, e.g. a
/// DeviceFactorSource being a mnemonic securely stored in a
/// device (phone), where the ID of it is the hash of a special
/// key derived near the root of it.
#[derive(Serialize, Deserialize, EnumAsInner, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(remote = "Self")]
pub enum FactorSourceID {
    #[serde(rename = "fromHash")]
    Hash(FactorSourceIDFromHash),
    #[serde(rename = "fromAddress")]
    Address(FactorSourceIDFromAddress),
}

impl<'de> Deserialize<'de> for FactorSourceID {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // https://github.com/serde-rs/serde/issues/1343#issuecomment-409698470
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(rename = "discriminator")]
            _ignore: String,
            #[serde(flatten, with = "FactorSourceID")]
            inner: FactorSourceID,
        }
        Wrapper::deserialize(deserializer).map(|w| w.inner)
    }
}

impl Serialize for FactorSourceID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FactorSourceID", 2)?;
        match self {
            FactorSourceID::Hash(from_hash) => {
                let discriminant = "fromHash";
                state.serialize_field("discriminator", discriminant)?;
                state.serialize_field(discriminant, from_hash)?;
            }
            FactorSourceID::Address(from_address) => {
                let discriminant = "fromAddress";
                state.serialize_field("discriminator", discriminant)?;
                state.serialize_field(discriminant, from_address)?;
            }
        }
        state.end()
    }
}

impl FactorSourceID {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        FactorSourceID::Hash(FactorSourceIDFromHash::placeholder())
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use crate::v100::factors::factor_source_id_from_address::FactorSourceIDFromAddress;

    use super::FactorSourceID;

    #[test]
    fn json_roundtrip_from_hash() {
        let model = FactorSourceID::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "fromHash": {
                    "kind": "device",
                    "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
                },
                "discriminator" : "fromHash"
            }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_from_address() {
        let model = FactorSourceID::Address(FactorSourceIDFromAddress::placeholder());
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "fromAddress": {
                    "kind": "trustedContact",
                    "body": "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                },
                "discriminator" : "fromAddress"
            }
            "#,
        )
    }
}
