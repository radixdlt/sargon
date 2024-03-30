use crate::prelude::*;

#[uniffi::export]
pub fn new_non_fungible_resource_indicator_sample(
) -> NonFungibleResourceIndicator {
    NonFungibleResourceIndicator::sample()
}

#[uniffi::export]
pub fn new_non_fungible_resource_indicator_sample_other(
) -> NonFungibleResourceIndicator {
    NonFungibleResourceIndicator::sample_other()
}

#[uniffi::export]
pub fn non_fungible_resource_indicator_get_ids(
    indicator: &NonFungibleResourceIndicator,
) -> Vec<NonFungibleLocalId> {
    indicator.ids()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleResourceIndicator;

    #[test]
    fn inequality() {
        assert_ne!(
            new_non_fungible_resource_indicator_sample(),
            new_non_fungible_resource_indicator_sample_other()
        );
    }

    #[test]
    fn get_ids() {
        let sut = SUT::sample();
        assert_eq!(sut.ids(), non_fungible_resource_indicator_get_ids(&sut));
    }
}
