use crate::prelude::*;

/// A unique and stable identifier of a FactorSource, e.g. a
/// DeviceFactorSource being a mnemonic securely stored in a
/// device (phone), where the ID of it is the hash of a special
/// key derived near the root of it.
#[derive(
    Serialize,
    Deserialize,
    EnumAsInner,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Enum,
)]
#[serde(untagged, remote = "Self")]
pub enum FactorSourceID {
    /// FactorSourceID from the blake2b hash of the special HD public key derived at `CAP26::GetID`,
    /// for a certain `FactorSourceKind`
    Hash {
        #[serde(rename = "fromHash")]
        #[display("{}", value)]
        value: FactorSourceIDFromHash,
    },

    /// FactorSourceID from an AccountAddress, typically used by `trustedContact` FactorSource.
    Address {
        #[serde(rename = "fromAddress")]
        #[display("{}", value)]
        value: FactorSourceIDFromAddress,
    },
}

impl From<FactorSourceIDFromHash> for FactorSourceID {
    fn from(val: FactorSourceIDFromHash) -> Self {
        FactorSourceID::Hash { value: val }
    }
}

impl From<FactorSourceIDFromAddress> for FactorSourceID {
    fn from(val: FactorSourceIDFromAddress) -> Self {
        FactorSourceID::Address { value: val }
    }
}

impl<'de> Deserialize<'de> for FactorSourceID {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        // https://github.com/serde-rs/serde/issues/1343#issuecomment-409698470
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(rename = "discriminator")]
            _ignore: String,
            #[serde(flatten, with = "FactorSourceID")]
            value: FactorSourceID,
        }
        Wrapper::deserialize(deserializer).map(|w| w.value)
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
            FactorSourceID::Hash { value } => {
                let discriminant = "fromHash";
                state.serialize_field("discriminator", discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
            FactorSourceID::Address { value } => {
                let discriminant = "fromAddress";
                state.serialize_field("discriminator", discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
        }
        state.end()
    }
}

impl HasPlaceholder for FactorSourceID {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        FactorSourceID::Hash {
            value: FactorSourceIDFromHash::placeholder(),
        }
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        FactorSourceID::Hash {
            value: FactorSourceIDFromHash::placeholder_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            FactorSourceID::placeholder(),
            FactorSourceID::placeholder()
        );
        assert_eq!(
            FactorSourceID::placeholder_other(),
            FactorSourceID::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            FactorSourceID::placeholder(),
            FactorSourceID::placeholder_other()
        );
    }

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
        let model = FactorSourceID::Address {
            value: FactorSourceIDFromAddress::placeholder(),
        };
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
