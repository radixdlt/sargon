use crate::prelude::*;

#[uniffi::export]
pub fn new_entity_flag_sample() -> EntityFlag {
    EntityFlag::sample()
}

#[uniffi::export]
pub fn new_entity_flag_sample_other() -> EntityFlag {
    EntityFlag::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EntityFlag;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_entity_flag_sample(),
                new_entity_flag_sample_other(),
                // duplicates should get removed
                new_entity_flag_sample(),
                new_entity_flag_sample_other(),
            ])
            .len(),
            2
        );
    }
}
