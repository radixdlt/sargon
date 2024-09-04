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
    /// A non-fungible resource with a specific claimable item count
    NonFungible {
        resource_address: ResourceAddress,
        count: u64,
    },
}

impl AccountLockerClaimableResource {
    pub fn resource_count(&self) -> u64 {
        match self {
            Self::Fungible { .. } => 1,
            Self::NonFungible { count, .. } => *count,
        }
    }

    /// Coerces the resource count to be at most the given maximum.
    ///
    /// If the resource is fungible, it will be returned as is,
    /// because it's always considered to be a single resource regardless of the amount.
    ///
    /// If the resource is non-fungible, the count will be clamped to the given maximum.
    pub fn coerce_resource_count_at_most(&self, maximum: u64) -> Self {
        assert!(maximum > 0, "Invalid input, maximum must be greater than 0");
        match self {
            Self::Fungible { .. } => self.clone(),
            Self::NonFungible {
                resource_address,
                count,
            } => Self::NonFungible {
                resource_address: *resource_address,
                count: min(*count, maximum),
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
            count: 2,
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
}
