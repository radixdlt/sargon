use crate::prelude::*;
use std::cmp::min;

/// A claimable resource in an account locker.
#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
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

#[uniffi::export]
pub fn new_account_locker_claimable_resource_sample(
) -> AccountLockerClaimableResource {
    AccountLockerClaimableResource::sample()
}

#[uniffi::export]
pub fn new_account_locker_claimable_resource_sample_other(
) -> AccountLockerClaimableResource {
    AccountLockerClaimableResource::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountLockerClaimableResource;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_account_locker_claimable_resource_sample(),
                new_account_locker_claimable_resource_sample_other(),
                // duplicates should get removed
                new_account_locker_claimable_resource_sample(),
                new_account_locker_claimable_resource_sample_other(),
            ])
            .len(),
            2
        );
    }
}
