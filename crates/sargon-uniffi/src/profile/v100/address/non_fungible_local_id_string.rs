use crate::prelude::*;

/// A string matching `[_0-9a-zA-Z]{1,64}`.
///
/// This is an internal wrapping of Scrypto's `StringNonFungibleLocalId`
/// with a UniFFI custom converter using `String` as `Builtin`.
///
/// Using this type instead of `String` directly in `NonFungibleLocalId::Str`,
/// allows us to do impl `From<NonFungibleLocalId> for NonFungibleLocalId` instead
/// of `TryFrom<NonFungibleLocalId>`.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[debug("{}", self.to_string())]
#[display("{}", self.secret_magic.value().to_owned())]
pub struct NonFungibleLocalIdString {
    secret_magic: ScryptoStringNonFungibleLocalId,
}

#[uniffi::export]
pub fn new_non_fungible_local_id_string_from_str(
    string: String,
) -> Result<NonFungibleLocalIdString> {
    string.parse()
}


uniffi::custom_type!(ScryptoStringNonFungibleLocalId, String);

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