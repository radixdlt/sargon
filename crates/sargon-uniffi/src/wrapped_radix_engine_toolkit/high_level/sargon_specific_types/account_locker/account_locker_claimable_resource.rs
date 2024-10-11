use crate::prelude::*;
use sargon::AccountLockerClaimableResource as InternalAccountLockerClaimableResource;

/// A claimable resource in an account locker.
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum AccountLockerClaimableResource {
    /// A fungible resource with a specific claimable amount
    Fungible {
        resource_address: ResourceAddress,
        amount: Decimal192,
    },
    /// A non-fungible resource with the total number of items that can be claimed
    NonFungible {
        resource_address: ResourceAddress,
        number_of_items: u64,
    },
}

impl From<InternalAccountLockerClaimableResource>
    for AccountLockerClaimableResource
{
    fn from(value: InternalAccountLockerClaimableResource) -> Self {
        match value {
            InternalAccountLockerClaimableResource::Fungible {
                resource_address,
                amount,
            } => AccountLockerClaimableResource::Fungible {
                resource_address: resource_address.into(),
                amount: amount.into(),
            },
            InternalAccountLockerClaimableResource::NonFungible {
                resource_address,
                number_of_items,
            } => AccountLockerClaimableResource::NonFungible {
                resource_address: resource_address.into(),
                number_of_items,
            },
        }
    }
}

impl Into<InternalAccountLockerClaimableResource>
    for AccountLockerClaimableResource
{
    fn into(self) -> InternalAccountLockerClaimableResource {
        match self {
            AccountLockerClaimableResource::Fungible {
                resource_address,
                amount,
            } => InternalAccountLockerClaimableResource::Fungible {
                resource_address: resource_address.into(),
                amount: amount.into(),
            },
            AccountLockerClaimableResource::NonFungible {
                resource_address,
                number_of_items,
            } => InternalAccountLockerClaimableResource::NonFungible {
                resource_address: resource_address.into(),
                number_of_items,
            },
        }
    }
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
