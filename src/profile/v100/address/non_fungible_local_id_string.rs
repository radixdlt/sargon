use crate::prelude::*;

use radix_engine_common::prelude::StringNonFungibleLocalId as ScryptoStringNonFungibleLocalId;

#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[debug("{}", self.to_string())]
#[display("{}", self.secret_magic.value().to_owned())]
pub struct NonFungibleLocalIdString {
    secret_magic: ScryptoStringNonFungibleLocalId,
}

impl FromStr for NonFungibleLocalIdString {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        scrypto_string_non_fungible_local_id(s).map(|i| i.into())
    }
}

impl From<ScryptoStringNonFungibleLocalId> for NonFungibleLocalIdString {
    fn from(value: ScryptoStringNonFungibleLocalId) -> Self {
        Self {
            secret_magic: value,
        }
    }
}
impl From<NonFungibleLocalIdString> for ScryptoStringNonFungibleLocalId {
    fn from(value: NonFungibleLocalIdString) -> Self {
        value.secret_magic
    }
}

fn scrypto_string_non_fungible_local_id(
    string: impl AsRef<str>,
) -> Result<ScryptoStringNonFungibleLocalId> {
    ScryptoStringNonFungibleLocalId::new(string).map_err(|e| {
        error!("{:?}", e);
        CommonError::InvalidNonFungibleLocalIDString
    })
}

impl crate::UniffiCustomTypeConverter for ScryptoStringNonFungibleLocalId {
    type Builtin = String;

    #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        scrypto_string_non_fungible_local_id(val).map_err(|e| e.into())
    }

    #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.value().to_owned()
    }
}
