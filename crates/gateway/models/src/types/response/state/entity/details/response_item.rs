use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
)]
pub struct StateEntityDetailsResponseItem {
    /// Bech32m-encoded human readable version of the address.
    pub address: Address,

    /// Fungible resources collection.
    pub fungible_resources: Option<FungibleResourcesCollection>,

    /// Non-fungible resources collection.
    pub non_fungible_resources: Option<NonFungibleResourcesCollection>,

    /// Entity metadata collection.
    pub metadata: EntityMetadataCollection,

    /// More details of this entity.
    pub details: Option<StateEntityDetailsResponseItemDetails>,
}

impl StateEntityDetailsResponseItem {
    pub fn new(
        address: Address,
        fungible_resources: impl Into<Option<FungibleResourcesCollection>>,
        non_fungible_resources: impl Into<Option<NonFungibleResourcesCollection>>,
        metadata: EntityMetadataCollection,
        details: impl Into<Option<StateEntityDetailsResponseItemDetails>>,
    ) -> StateEntityDetailsResponseItem {
        StateEntityDetailsResponseItem {
            address,
            fungible_resources: fungible_resources.into(),
            non_fungible_resources: non_fungible_resources.into(),
            metadata,
            details: details.into(),
        }
    }
}

impl StateEntityDetailsResponseItem {
    pub fn can_be_transferred(&self) -> bool {
        let Some(details) = &self.details else {
            return false;
        };
        details.can_be_transferred()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = StateEntityDetailsResponseItem;

    #[test]
    fn can_be_transferred() {
        // Test case: details is None
        let sut = SUT::new(
            Address::sample(),
            None,
            None,
            EntityMetadataCollection::empty(),
            None,
        );
        assert!(!sut.can_be_transferred());

        // Test case: details can be transferred
        let details = StateEntityDetailsResponseItemDetails::FungibleResource(
            StateEntityDetailsResponseFungibleResourceDetails::new(
                ComponentEntityRoleAssignments::sample_allow_all(),
            ),
        );
        let sut = SUT::new(
            Address::sample(),
            None,
            None,
            EntityMetadataCollection::empty(),
            details,
        );
        assert!(sut.can_be_transferred());

        // Test case: details cannot be transferred
        let details = StateEntityDetailsResponseItemDetails::FungibleResource(
            StateEntityDetailsResponseFungibleResourceDetails::new(
                ComponentEntityRoleAssignments::sample_deny_all(),
            ),
        );
        let sut = SUT::new(
            Address::sample(),
            None,
            None,
            EntityMetadataCollection::empty(),
            details,
        );
        assert!(!sut.can_be_transferred());
    }
}
