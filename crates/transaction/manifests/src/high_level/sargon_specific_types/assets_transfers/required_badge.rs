use crate::prelude::*;

/// A badge that is required to authorize/validate the transaction execution.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RequiredBadge {
    Fungible {
        resource_address: ResourceAddress,
        amount: Decimal192,
    },
    NonFungible {
        resource_address: ResourceAddress,
        ids: Vec<NonFungibleLocalId>,
    },
}

impl RequiredBadge {
    pub fn resource_address(&self) -> &ResourceAddress {
        match self {
            Self::Fungible { resource_address, .. } => resource_address,
            Self::NonFungible { resource_address, .. } => resource_address,
        }
    }
}

impl HasSampleValues for RequiredBadge {
    fn sample() -> Self {
        Self::Fungible {
            resource_address: ResourceAddress::sample(),
            amount: Decimal192::one(),
        }
    }

    fn sample_other() -> Self {
        Self::NonFungible {
            resource_address: ResourceAddress::sample_other(),
            ids: vec![NonFungibleLocalId::sample()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RequiredBadge;

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
