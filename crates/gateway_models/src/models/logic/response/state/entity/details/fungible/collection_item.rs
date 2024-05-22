use crate::prelude::*;

impl FungibleResourcesCollectionItem {
    pub fn resource_address(&self) -> ResourceAddress {
        match self {
            Self::Global(item) => item.resource_address,
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
