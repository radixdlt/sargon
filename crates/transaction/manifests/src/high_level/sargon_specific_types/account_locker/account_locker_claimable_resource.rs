use crate::prelude::*;
use std::cmp::min;

/// A claimable resource in an account locker.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl AccountLockerClaimableResource {
    pub fn number_of_items(&self) -> u64 {
        match self {
            Self::Fungible { .. } => 1,
            Self::NonFungible {
                number_of_items, ..
            } => *number_of_items,
        }
    }

    /// Coerces the number of items to be at most the given maximum.
    ///
    /// If the resource is fungible, it will be returned as is,
    /// because it's always considered to be a single item regardless of the amount.
    ///
    /// If the resource is non_fungible, the number of items will be clamped to the given maximum.
    pub fn coerce_number_of_items_at_most(&self, maximum: u64) -> Self {
        assert!(maximum > 0, "Invalid input, maximum must be greater than 0");
        match self {
            Self::Fungible { .. } => self.clone(),
            Self::NonFungible {
                resource_address,
                number_of_items: count,
            } => Self::NonFungible {
                resource_address: *resource_address,
                number_of_items: min(*count, maximum),
            },
        }
    }
}

impl HasSampleValues for AccountLockerClaimableResource {
    fn sample() -> Self {
        Self::Fungible {
            resource_address: ResourceAddress::sample(),
            amount: Decimal192::ten(),
        }
    }

    fn sample_other() -> Self {
        Self::NonFungible {
            resource_address: ResourceAddress::sample_other(),
            number_of_items: 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountLockerClaimableResource;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn number_of_items() {
        assert_eq!(SUT::sample().number_of_items(), 1);
        assert_eq!(SUT::sample_other().number_of_items(), 2);
    }

    #[test]
    fn coerce_number_of_items_at_most() {
        let mut sut = SUT::sample();

        let actual = sut.coerce_number_of_items_at_most(10);
        assert_eq!(actual, sut);

        sut = SUT::NonFungible {
            resource_address: ResourceAddress::sample_other(),
            number_of_items: 5,
        };
        let actual = sut.coerce_number_of_items_at_most(1);
        let expected = SUT::NonFungible {
            resource_address: ResourceAddress::sample_other(),
            number_of_items: 1,
        };
        assert_eq!(actual, expected);
    }
}
