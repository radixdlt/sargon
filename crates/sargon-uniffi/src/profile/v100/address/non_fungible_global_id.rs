use crate::prelude::*;
use sargon::NonFungibleGlobalId as InternalNonFungibleGlobalId;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct NonFungibleGlobalId {
    // N.B. we WANT This to be a `NonFungibleResourceAddress` type, alas, it
    // cannot, since that validation does not happen part of Engine, so it is
    // possible (maybe even likely) that some Non Fungible tokens have addresses
    // which are "fungible" (i.e. entity type `GlobalFungibleResourceManager`
    // instead of `GlobalNonFungibleResourceManager`).
    //
    // For more info see slack:
    // https://rdxworks.slack.com/archives/C01HK4QFXNY/p1709633826502809?thread_ts=1709633374.199459&channel=C01HK4QFXNY&message_ts=1709633826.502809
    pub resource_address: ResourceAddress,
    pub non_fungible_local_id: NonFungibleLocalId,
    pub as_string: String,
    pub formatted: FormattedAddress
}

impl From<InternalNonFungibleGlobalId> for FormattedAddress {
    fn from(val: InternalNonFungibleGlobalId) -> Self {
        Self {
            full: val.formatted(sargon::AddressFormat::Full),
            raw: val.formatted(sargon::AddressFormat::Raw),
            default: val.formatted(sargon::AddressFormat::Default)
        }
    }
}

impl From<InternalNonFungibleGlobalId> for NonFungibleGlobalId {
    fn from(val: InternalNonFungibleGlobalId) -> Self {
        Self {
            resource_address: val.resource_address.into(),
            non_fungible_local_id: val.non_fungible_local_id.clone().into(),
            as_string: val.to_string(),
            formatted: val.into(),
        }
    }
}

impl From<NonFungibleGlobalId> for InternalNonFungibleGlobalId {
    fn from(val: NonFungibleGlobalId) -> Self {
        Self {
            resource_address: val.resource_address.into_internal(),
            non_fungible_local_id: val.non_fungible_local_id.into_internal(),
        }
    }
}

impl NonFungibleGlobalId {
    pub fn into_internal(&self) -> InternalNonFungibleGlobalId {
        self.clone().into()
    }
}

#[uniffi::export]
pub fn new_non_fungible_global_id_from_string(
    string: String,
) -> Result<NonFungibleGlobalId> {
    InternalNonFungibleGlobalId::from_str(&string).into_result()
}

#[uniffi::export]
pub fn new_non_fungible_global_id(
    address: NonFungibleResourceAddress,
    local_id: NonFungibleLocalId,
) -> NonFungibleGlobalId {
    InternalNonFungibleGlobalId::new(address.into_internal(), local_id.into_internal()).into()
}

#[uniffi::export]
pub fn new_non_fungible_global_id_sample() -> NonFungibleGlobalId {
    InternalNonFungibleGlobalId::sample().into()
}

#[uniffi::export]
pub fn new_non_fungible_global_id_sample_other() -> NonFungibleGlobalId {
    InternalNonFungibleGlobalId::sample_other().into()
}
