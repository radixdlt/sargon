use crate::prelude::*;
use sargon::NonFungibleLocalIdString as InternalNonFungibleLocalIdString;

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
     uniffi::Record,
)]
pub struct NonFungibleLocalIdString {
    value: String,
}

impl From<InternalNonFungibleLocalIdString> for NonFungibleLocalIdString {
    fn from(value: InternalNonFungibleLocalIdString) -> Self {
        Self { value: value.to_string() }
    }
}

impl TryInto<InternalNonFungibleLocalIdString> for NonFungibleLocalIdString {
    type Error = CommonError;

    fn try_into(self) -> Result<InternalNonFungibleLocalIdString> {
        self.value.parse::<InternalNonFungibleLocalIdString>()
    }
}

#[uniffi::export]
pub fn new_non_fungible_local_id_string_from_str(
    string: String,
) -> Result<NonFungibleLocalIdString> {
    NonFungibleLocalIdString { value: string }.try_into().map_result()
}