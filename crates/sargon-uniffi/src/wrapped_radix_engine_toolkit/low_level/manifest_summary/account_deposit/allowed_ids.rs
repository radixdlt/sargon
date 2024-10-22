use crate::prelude::*;
use sargon::AllowedIds as InternalAllowedIds;

/// Represents which ids are possible in a non-fungible balance.
///
/// `Any` represents that any id is possible. `Allowlist` represents that
/// any ids in the balance have to be in the allowlist.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum AllowedIds {
    Allowlist { ids: Vec<NonFungibleLocalId> },
    Any,
}