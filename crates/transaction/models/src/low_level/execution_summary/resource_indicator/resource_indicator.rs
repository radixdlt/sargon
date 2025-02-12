use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, EnumAsInner)]
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

impl ResourceIndicator {
    pub fn get_address(&self) -> ResourceAddress {
        match self {
            ResourceIndicator::Fungible {
                resource_address,
                indicator: _,
            } => *resource_address,
            ResourceIndicator::NonFungible {
                resource_address,
                indicator: _,
            } => *resource_address,
        }
    }

    pub fn get_non_fungible_indicator(
        &self,
    ) -> Option<NonFungibleResourceIndicator> {
        match self {
            ResourceIndicator::Fungible {
                resource_address: _,
                indicator: _,
            } => None,
            ResourceIndicator::NonFungible {
                resource_address: _,
                indicator,
            } => Some(indicator.clone()),
        }
    }
}

impl ResourceIndicator {
    pub fn fungible(
        resource_address: impl Into<ResourceAddress>,
        indicator: impl Into<FungibleResourceIndicator>,
    ) -> Self {
        Self::Fungible {
            resource_address: resource_address.into(),
            indicator: indicator.into(),
        }
    }
    pub fn non_fungible(
        resource_address: impl Into<ResourceAddress>,
        indicator: impl Into<NonFungibleResourceIndicator>,
    ) -> Self {
        Self::NonFungible {
            resource_address: resource_address.into(),
            indicator: indicator.into(),
        }
    }
}

impl From<(RetInvocationIoItem, NetworkID)> for ResourceIndicator {
    fn from(value: (RetInvocationIoItem, NetworkID)) -> Self {
        let (ret, network_id) = value;
        match ret {
            RetInvocationIoItem::Fungible(
                resource_address,
                fungible_indicator,
            ) => Self::fungible(
                (resource_address, network_id),
                fungible_indicator,
            ),
            RetInvocationIoItem::NonFungible(
                resource_address,
                non_fungible_indicator,
            ) => Self::non_fungible(
                (resource_address, network_id),
                non_fungible_indicator,
            ),
        }
    }
}

impl ResourceIndicator {
    pub fn sample_stokenet() -> Self {
        Self::fungible(
            ResourceAddress::sample_stokenet_candy(),
            FungibleResourceIndicator::sample(),
        )
    }
}

impl HasSampleValues for ResourceIndicator {
    fn sample() -> Self {
        Self::fungible(
            ResourceAddress::sample(),
            FungibleResourceIndicator::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::non_fungible(
            NonFungibleResourceAddress::sample_other(),
            NonFungibleResourceIndicator::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {

    use radix_engine_toolkit::types::InstructionIndex;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceIndicator;

    #[test]
    fn get_address() {
        assert_eq!(SUT::sample().get_address(), ResourceAddress::sample());
        assert_eq!(
            SUT::sample_other().get_address(),
            ResourceAddress::sample_mainnet_nft_other()
        );
    }

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
        let ret = RetInvocationIoItem::new_guaranteed_fungible(
            resource_address.into(),
            1.into(),
        );

        assert_eq!(SUT::from((ret.clone(), NetworkID::Mainnet)), SUT::sample());

        // Not equals for wrong network
        assert_ne!(SUT::from((ret, NetworkID::Stokenet)), SUT::sample());
    }

    #[test]
    fn from_ret_non_fungible() {
        let resource_address = NonFungibleResourceAddress::sample_other();

        let ret = RetInvocationIoItem::new_predicted_non_fungible(
            ResourceAddress::from(resource_address).into(),
            [NonFungibleLocalId::sample_other()]
                .into_iter()
                .map(ScryptoNonFungibleLocalId::from),
            InstructionIndex::of(0),
        );

        assert_eq!(
            SUT::from((ret.clone(), NetworkID::Mainnet)),
            SUT::sample_other()
        );

        // Not equals for wrong network
        assert_ne!(SUT::from((ret, NetworkID::Stokenet)), SUT::sample_other());
    }
}
