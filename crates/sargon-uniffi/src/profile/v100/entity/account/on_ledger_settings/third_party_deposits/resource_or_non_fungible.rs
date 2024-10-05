use crate::prelude::*;

/// The addresses that can be added as exception to the `DepositRule`
#[derive(
    Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum,
)]
pub enum ResourceOrNonFungible {
    Resource { value: ResourceAddress },

    NonFungible { value: NonFungibleGlobalId },
}

impl Identifiable for ResourceOrNonFungible {
    type ID = Self;

    fn id(&self) -> Self::ID {
        self.clone()
    }
}

#[uniffi::export]
pub fn new_resource_or_non_fungible_sample() -> ResourceOrNonFungible {
    ResourceOrNonFungible::sample()
}

#[uniffi::export]
pub fn new_resource_or_non_fungible_sample_other() -> ResourceOrNonFungible {
    ResourceOrNonFungible::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceOrNonFungible;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_resource_or_non_fungible_sample(),
                new_resource_or_non_fungible_sample_other(),
                // duplicates should get removed
                new_resource_or_non_fungible_sample(),
                new_resource_or_non_fungible_sample_other(),
            ])
            .len(),
            2
        );
    }
}
