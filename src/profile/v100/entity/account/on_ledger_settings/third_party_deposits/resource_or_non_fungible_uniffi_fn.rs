use crate::prelude::*;

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
