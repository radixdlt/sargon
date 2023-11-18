use radix_engine_common::types::ResourceAddress as EngineResourceAddress;
use serde::{Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceAddress(EngineResourceAddress);

impl Serialize for ResourceAddress {
    /// Serializes this `AccountAddress` into its bech32 address string as JSON.
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        // serializer.serialize_str(&self.address)
        todo!();
    }
}

impl<'de> serde::Deserialize<'de> for ResourceAddress {
    /// Tries to deserializes a JSON string as a bech32 address into an `AccountAddress`.
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        // Self::try_from_bech32(&s).map_err(de::Error::custom)
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_test_utils::json::{
        assert_json_roundtrip, assert_json_value_eq_after_roundtrip,
        assert_json_value_ne_after_roundtrip,
    };

    use super::ResourceAddress;

    // #[test]
    // fn json_roundtrip() {
    //     let a: ResourceAddress =
    //         "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
    //             .try_into()
    //             .unwrap();

    //     assert_json_value_eq_after_roundtrip(
    //         &a,
    //         json!("resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"),
    //     );
    //     assert_json_roundtrip(&a);
    //     assert_json_value_ne_after_roundtrip(
    //         &a,
    //         json!("resource_rdx1tkk83magp3gjyxrpskfsqwkg4g949rmcjee4tu2xmw93ltw2cz94sq"),
    //     );
    // }
}
