use crate::prelude::*;

/// A helper struct to group all the resources of a given account.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct FetchResourcesOutput {
    pub fungibles: Vec<FungibleResourcesCollectionItem>,
    pub non_fungibles: Vec<NonFungibleResourcesCollectionItem>,
}

impl FetchResourcesOutput {
    pub fn new(
        fungibles: impl IntoIterator<Item = FungibleResourcesCollectionItem>,
        non_fungibles: impl IntoIterator<Item = NonFungibleResourcesCollectionItem>,
    ) -> Self {
        Self {
            fungibles: Vec::from_iter(fungibles),
            non_fungibles: Vec::from_iter(non_fungibles),
        }
    }
}

impl FetchResourcesOutput {
    pub fn resource_addresses(&self) -> Vec<ResourceAddress> {
        self.fungibles
            .iter()
            .map(|item| item.resource_address())
            .chain(
                self.non_fungibles
                    .iter()
                    .map(|item| item.resource_address()),
            )
            .collect()
    }
}

#[cfg(test)]
mod tests {
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
