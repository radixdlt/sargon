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
#[derive(
    Serialize, Deserialize, EnumAsInner, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(remote = "Self")]
pub enum FactorSourceID {
    /// FactorSourceID from the blake2b hash of the special HD public key derived at `CAP26::GetID`,
    /// for a certain `FactorSourceKind`
    #[serde(rename = "fromHash")]
    Hash(FactorSourceIDFromHash),

    /// FactorSourceID from an AccountAddress, typically used by `trustedContact` FactorSource.
    #[serde(rename = "fromAddress")]
    Address(FactorSourceIDFromAddress),
}

impl From<FactorSourceIDFromHash> for FactorSourceID {
    fn from(value: FactorSourceIDFromHash) -> Self {
        Self::Hash(value)
    }
}

impl From<FactorSourceIDFromAddress> for FactorSourceID {
    fn from(value: FactorSourceIDFromAddress) -> Self {
        Self::Address(value)
    }
}

impl<'de> Deserialize<'de> for FactorSourceID {
    #[cfg(not(tarpaulin_include))] // false negative
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
    #[cfg(not(tarpaulin_include))] // false negative
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

#[cfg(any(test, feature = "placeholder"))]
impl FactorSourceID {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        FactorSourceID::Hash(FactorSourceIDFromHash::placeholder())
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::assert_eq_after_json_roundtrip;

    use crate::v100::factors::{
        factor_source_id_from_address::FactorSourceIDFromAddress,
        factor_source_id_from_hash::FactorSourceIDFromHash,
    };

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

    #[test]
    fn hash_into_as_roundtrip() {
        let from_hash = FactorSourceIDFromHash::placeholder();
        let id: FactorSourceID = from_hash.clone().into(); // test `into()`
        assert_eq!(id.as_hash().unwrap(), &from_hash);
    }

    #[test]
    fn hash_into_as_wrong_fails() {
        let from_hash = FactorSourceIDFromHash::placeholder();
        let id: FactorSourceID = from_hash.into(); // test `into()`
        assert!(id.as_address().is_none());
    }

    #[test]
    fn address_into_as_roundtrip() {
        let from_address = FactorSourceIDFromAddress::placeholder();
        let id: FactorSourceID = from_address.clone().into(); // test `into()`
        assert_eq!(id.as_address().unwrap(), &from_address);
    }

    #[test]
    fn address_into_as_wrong_fails() {
        let from_address = FactorSourceIDFromAddress::placeholder();
        let id: FactorSourceID = from_address.into(); // test `into()`
        assert!(id.as_hash().is_none());
    }
}
