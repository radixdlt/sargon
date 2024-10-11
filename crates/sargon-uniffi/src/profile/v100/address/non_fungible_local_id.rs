use crate::prelude::*;
use sargon::NonFungibleLocalId as InternalNonFungibleLocalId;

#[derive(Clone, Hash, PartialEq, Eq, InternalConversionV2, uniffi::Enum)]
pub enum NonFungibleLocalId {
    /// Unsigned integers, up to u64.
    ///
    /// Create using `NonFungibleLocalId::integer(...)`.
    Integer { value: u64 },

    /// String matching `[_0-9a-zA-Z]{1,64}`.
    ///
    /// Create using `NonFungibleLocalId::string(...).unwrap()`.
    Str { value: NonFungibleLocalIdString },

    /// Bytes, of length between 1 and 64.
    ///
    /// Create using `NonFungibleLocalId::bytes(...).unwrap()`.
    Bytes { value: NonEmptyMax64Bytes },

    /// RUID, v4, variant 1, big endian. See https://www.rfc-editor.org/rfc/rfc4122
    ///
    /// Create using `NonFungibleLocalId::ruid(...).unwrap()`.
    Ruid { value: Exactly32Bytes },
}

#[uniffi::export]
pub fn new_non_fungible_local_id_from_string(
    local_id: String,
) -> Result<NonFungibleLocalId> {
    InternalNonFungibleLocalId::from_str(&local_id).map_result()
}

#[uniffi::export]
pub fn non_fungible_local_id_as_str(id: NonFungibleLocalId) -> String {
    id.into_internal().to_string()
}

#[uniffi::export]
pub fn non_fungible_local_id_to_user_facing_string(
    id: &NonFungibleLocalId,
) -> String {
    id.into_internal().to_user_facing_string()
}

#[uniffi::export]
pub fn non_fungible_local_id_formatted(
    id: &NonFungibleLocalId,
    format: AddressFormat,
) -> String {
    id.into_internal().formatted(format.into_internal())
}

#[uniffi::export]
pub fn new_non_fungible_local_id_int(value: u64) -> NonFungibleLocalId {
    InternalNonFungibleLocalId::integer(value).into()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_string(
    string: String,
) -> Result<NonFungibleLocalId> {
    InternalNonFungibleLocalId::string(string).map_result()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_bytes(
    bytes: BagOfBytes,
) -> Result<NonFungibleLocalId> {
    InternalNonFungibleLocalId::bytes(bytes.into_internal()).map_result()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_ruid(
    bytes: BagOfBytes,
) -> Result<NonFungibleLocalId> {
    InternalNonFungibleLocalId::ruid(bytes.into_internal()).map_result()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_sample() -> NonFungibleLocalId {
    InternalNonFungibleLocalId::sample().into()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_sample_other() -> NonFungibleLocalId {
    InternalNonFungibleLocalId::sample_other().into()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_random() -> NonFungibleLocalId {
    InternalNonFungibleLocalId::random().into()
}
