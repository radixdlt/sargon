use crate::prelude::*;

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
