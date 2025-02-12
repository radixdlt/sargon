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
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct NonFungibleLocalIdString {
    secret_magic: String,
}

impl NonFungibleLocalIdString {
    pub fn into_internal(&self) -> InternalNonFungibleLocalIdString {
        self.clone().into()
    }
}

impl From<InternalNonFungibleLocalIdString> for NonFungibleLocalIdString {
    fn from(value: InternalNonFungibleLocalIdString) -> Self {
        Self {
            secret_magic: value.to_string(),
        }
    }
}

impl From<NonFungibleLocalIdString> for InternalNonFungibleLocalIdString {
    fn from(val: NonFungibleLocalIdString) -> Self {
        val.secret_magic
            .parse::<InternalNonFungibleLocalIdString>()
            .unwrap()
    }
}

#[uniffi::export]
pub fn new_non_fungible_local_id_string_from_str(
    string: String,
) -> Result<NonFungibleLocalIdString> {
    string
        .parse::<InternalNonFungibleLocalIdString>()
        .into_result()
}

decl_conversion_tests_for! {
    NonFungibleLocalIdString
}
