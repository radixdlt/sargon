use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
    EnumAsInner,
)]
#[serde(untagged)]
pub enum FungibleResourcesCollectionItem {
    Global(FungibleResourcesCollectionItemGloballyAggregated),
}

impl FungibleResourcesCollectionItem {
    pub fn global(
        resource_address: ResourceAddress,
        amount: impl Into<Decimal192>,
    ) -> Self {
        Self::Global(FungibleResourcesCollectionItemGloballyAggregated::new(
            resource_address,
            amount,
        ))
    }
}

impl HasSampleValues for FungibleResourcesCollectionItem {
    fn sample() -> Self {
        Self::Global(FungibleResourcesCollectionItemGloballyAggregated::sample())
    }

    fn sample_other() -> Self {
        Self::Global(
            FungibleResourcesCollectionItemGloballyAggregated::sample_other(),
        )
    }
}

impl FungibleResourcesCollectionItem {
    pub fn resource_address(&self) -> ResourceAddress {
        match self {
            Self::Global(item) => item.resource_address,
        }
    }
    
    pub fn amount(&self) -> Decimal192 {
        match self {
            Self::Global(item) => item.amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FungibleResourcesCollectionItem;

    #[test]
    fn inequality() {
        let resource_address = ResourceAddress::sample();
        let sut =
            SUT::Global(FungibleResourcesCollectionItemGloballyAggregated {
                amount: Decimal192::zero(),
                resource_address,
            });
        assert_eq!(sut.resource_address(), resource_address);
    }
}

#[cfg(test)]
mod address_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FetchResourcesOutput;

    #[test]
    fn resource_addresses() {
        let fungible = FungibleResourcesCollectionItem::sample();
        let non_fungible = NonFungibleResourcesCollectionItem::sample();
        let sut = SUT::new([fungible.clone()], [non_fungible.clone()]);

        assert_eq!(
            sut.resource_addresses(),
            vec![fungible.resource_address(), non_fungible.resource_address()]
        );
    }
}
