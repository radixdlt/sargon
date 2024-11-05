use crate::prelude::*;
use sargon::NonFungibleLocalId as InternalNonFungibleLocalId;

#[derive(Clone, Hash, PartialEq, Eq, uniffi::Record)]
pub struct NonFungibleLocalId {
    pub kind: NonFungibleLocalIdKind,
    pub as_string: String,
    pub user_facing_string: String,
    pub formatted: FormattedAddress,
}

#[derive(Clone, Hash, PartialEq, Eq, uniffi::Enum)]
pub enum NonFungibleLocalIdKind {
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

impl NonFungibleLocalId {
    pub fn into_internal(&self) -> InternalNonFungibleLocalId {
        self.clone().into()
    }
}

impl From<InternalNonFungibleLocalId> for NonFungibleLocalId {
    fn from(val: InternalNonFungibleLocalId) -> Self {
        Self {
            kind: val.clone().into(),
            as_string: val.to_string(),
            user_facing_string: val.to_user_facing_string(),
            formatted: val.into(),
        }
    }
}

impl From<NonFungibleLocalId> for InternalNonFungibleLocalId {
    fn from(val: NonFungibleLocalId) -> Self {
        val.kind.into()
    }
}

impl From<InternalNonFungibleLocalId> for NonFungibleLocalIdKind {
    fn from(val: InternalNonFungibleLocalId) -> Self {
        match val {
            InternalNonFungibleLocalId::Integer { value } => {
                NonFungibleLocalIdKind::Integer { value: value }
            }
            InternalNonFungibleLocalId::Bytes { value } => {
                NonFungibleLocalIdKind::Bytes {
                    value: value.into(),
                }
            }
            InternalNonFungibleLocalId::Str { value } => {
                NonFungibleLocalIdKind::Str {
                    value: value.into(),
                }
            }
            InternalNonFungibleLocalId::Ruid { value } => {
                NonFungibleLocalIdKind::Ruid {
                    value: value.into(),
                }
            }
        }
    }
}

impl From<NonFungibleLocalIdKind> for InternalNonFungibleLocalId {
    fn from(val: NonFungibleLocalIdKind) -> Self {
        match val {
            NonFungibleLocalIdKind::Integer { value } => {
                InternalNonFungibleLocalId::Integer { value: value }
            }
            NonFungibleLocalIdKind::Bytes { value } => {
                InternalNonFungibleLocalId::Bytes {
                    value: value.into(),
                }
            }
            NonFungibleLocalIdKind::Str { value } => {
                InternalNonFungibleLocalId::Str {
                    value: value.into(),
                }
            }
            NonFungibleLocalIdKind::Ruid { value } => {
                InternalNonFungibleLocalId::Ruid {
                    value: value.into(),
                }
            }
        }
    }
}

impl From<InternalNonFungibleLocalId> for FormattedAddress {
    fn from(val: InternalNonFungibleLocalId) -> Self {
        Self {
            full: val.formatted(sargon::AddressFormat::Full),
            raw: val.formatted(sargon::AddressFormat::Raw),
            default: val.formatted(sargon::AddressFormat::Default),
        }
    }
}

decl_conversion_tests_for!(NonFungibleLocalId);

#[uniffi::export]
pub fn new_non_fungible_local_id_from_string(
    local_id: String,
) -> Result<NonFungibleLocalId> {
    InternalNonFungibleLocalId::from_str(&local_id).into_result()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_int(value: u64) -> NonFungibleLocalId {
    InternalNonFungibleLocalId::integer(value).into()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_string(
    string: String,
) -> Result<NonFungibleLocalId> {
    InternalNonFungibleLocalId::string(string).into_result()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_bytes(
    bytes: BagOfBytes,
) -> Result<NonFungibleLocalId> {
    InternalNonFungibleLocalId::bytes(bytes.into_internal()).into_result()
}

#[uniffi::export]
pub fn new_non_fungible_local_id_ruid(
    bytes: BagOfBytes,
) -> Result<NonFungibleLocalId> {
    InternalNonFungibleLocalId::ruid(bytes.into_internal()).into_result()
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
