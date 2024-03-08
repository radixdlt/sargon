use crate::prelude::*;

use radix_engine_toolkit::transaction_types::ResourceIndicator as RetResourceIndicator;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum ResourceIndicator {
    Fungible {
        resource_address: ResourceAddress,
        indicator: FungibleResourceIndicator,
    },
    NonFungible {
        resource_address: ResourceAddress,
        indicator: NonFungibleResourceIndicator,
    },
}

impl From<(RetResourceIndicator, NetworkID)> for ResourceIndicator {
    fn from(value: (RetResourceIndicator, NetworkID)) -> Self {
        let (ret, network_id) = value;
        match ret {
            RetResourceIndicator::Fungible(
                resource_address,
                fungible_indicator,
            ) => Self::Fungible {
                resource_address: (resource_address, network_id).into(),
                indicator: fungible_indicator.into(),
            },
            RetResourceIndicator::NonFungible(
                resource_address,
                non_fungible_indicator,
            ) => Self::NonFungible {
                resource_address: (resource_address, network_id).into(),
                indicator: non_fungible_indicator.into(),
            },
        }
    }
}

impl HasSampleValues for ResourceIndicator {
    fn sample() -> Self {
        Self::Fungible {
            resource_address: ResourceAddress::sample(),
            indicator: FungibleResourceIndicator::sample(),
        }
    }

    fn sample_other() -> Self {
        Self::NonFungible {
            resource_address: ResourceAddress::sample_other(),
            indicator: NonFungibleResourceIndicator::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use radix_engine_common::prelude::NonFungibleLocalId as ScryptoNonFungibleLocalId;
    use radix_engine_toolkit::transaction_types::{
        FungibleResourceIndicator as RetFungibleResourceIndicator,
        NonFungibleResourceIndicator as RetNonFungibleResourceIndicator,
        Predicted as RetPredicted,
    };

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceIndicator;

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
    fn from_ret_fungible() {
        let resource_address = ResourceAddress::sample();
        let ret = RetResourceIndicator::Fungible(
            resource_address.into(),
            RetFungibleResourceIndicator::Guaranteed(1.into()),
        );

        assert_eq!(SUT::from((ret.clone(), NetworkID::Mainnet)), SUT::sample());

        // Not equals for wrong network
        assert_ne!(SUT::from((ret, NetworkID::Stokenet)), SUT::sample());
    }

    #[test]
    fn from_ret_non_fungible() {
        let resource_address = ResourceAddress::sample_other();

        let ret = RetResourceIndicator::NonFungible(
            resource_address.into(),
            RetNonFungibleResourceIndicator::ByAll {
                predicted_amount: RetPredicted {
                    value: 1.into(),
                    instruction_index: 0,
                },
                predicted_ids: RetPredicted {
                    value: [NonFungibleLocalId::sample_other()]
                        .into_iter()
                        .map(ScryptoNonFungibleLocalId::from)
                        .collect(),
                    instruction_index: 1,
                },
            },
        );

        assert_eq!(
            SUT::from((ret.clone(), NetworkID::Mainnet)),
            SUT::sample_other()
        );

        // Not equals for wrong network
        assert_ne!(SUT::from((ret, NetworkID::Stokenet)), SUT::sample_other());
    }
}
