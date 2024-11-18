use crate::prelude::*;
use sargon::AccountLockerClaimableResource as InternalAccountLockerClaimableResource;

/// A claimable resource in an account locker.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum AccountLockerClaimableResource {
    /// A fungible resource with a specific claimable amount
    Fungible {
        resource_address: ResourceAddress,
        amount: Decimal192,
    },
    /// A non_fungible resource with the total number of items that can be claimed
    NonFungible {
        resource_address: ResourceAddress,
        number_of_items: u64,
    },
}

#[uniffi::export]
pub fn new_account_locker_claimable_resource_sample(
) -> AccountLockerClaimableResource {
    InternalAccountLockerClaimableResource::sample().into()
}

#[uniffi::export]
pub fn new_account_locker_claimable_resource_sample_other(
) -> AccountLockerClaimableResource {
    InternalAccountLockerClaimableResource::sample_other().into()
}
