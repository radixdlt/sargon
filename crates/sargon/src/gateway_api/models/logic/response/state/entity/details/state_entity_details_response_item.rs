use crate::prelude::*;

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
