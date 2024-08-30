use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum AccountLockerClaimableResource {
    Fungible {
        resource_address: ResourceAddress,
        amount: Decimal192,
    },
    NonFungible {
        resource_address: ResourceAddress,
        ids: Vec<NonFungibleLocalId>,
    },
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
            ids: vec![
                NonFungibleLocalId::sample(),
                NonFungibleLocalId::sample_other(),
            ],
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
