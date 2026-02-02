use crate::prelude::*;
use sargon::HierarchicalDeterministicPublicKey as InternalHierarchicalDeterministicPublicKey;
use sargon::NonFungibleGlobalId as InternalNonFungibleGlobalId;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct NonFungibleGlobalId {
    // N.B. we WANT This to be a `NonFungibleResourceAddress` type, alas, it
    // cannot, since that validation does not happen part of Engine, so it is
    // possible (maybe even likely) that some Non Fungible tokens have addresses
    // which are "fungible" (i.e. entity type `GlobalFungibleResourceManager`
    // instead of `GlobalNonFungibleResourceManager`).
    //
    // For more info see slack:
    // https://rdxworks.slack.com/archives/C01HK4QFXNY/p1709633826502809?thread_ts=1709633374.199459&channel=C01HK4QFXNY&message_ts=1709633826.502809
    pub(crate) resource_address: ResourceAddress,
    pub(crate) non_fungible_local_id: NonFungibleLocalId,
}

#[uniffi::export]
pub fn new_non_fungible_global_id_from_string(
    string: String,
) -> Result<NonFungibleGlobalId> {
    InternalNonFungibleGlobalId::from_str(&string).into_result()
}

#[uniffi::export]
pub fn new_non_fungible_global_id_sample() -> NonFungibleGlobalId {
    InternalNonFungibleGlobalId::sample().into()
}

#[uniffi::export]
pub fn new_non_fungible_global_id_sample_other() -> NonFungibleGlobalId {
    InternalNonFungibleGlobalId::sample_other().into()
}

#[uniffi::export]
pub fn non_fungible_global_id_to_string(
    global_id: &NonFungibleGlobalId,
) -> String {
    global_id.into_internal().to_string()
}

#[uniffi::export]
pub fn non_fungible_global_id_formatted(
    global_id: &NonFungibleGlobalId,
    format: AddressFormat,
) -> String {
    global_id.into_internal().formatted(format.into_internal())
}

#[uniffi::export]
pub fn non_fungible_global_from_hierarchical_deterministic_public_key(
    public_key: &HierarchicalDeterministicPublicKey,
) -> NonFungibleGlobalId {
    InternalNonFungibleGlobalId::from(public_key.into_internal()).into()
}
