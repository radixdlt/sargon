use crate::prelude::*;

#[uniffi::export]
pub fn new_unsecured_entity_control_sample() -> UnsecuredEntityControl {
    UnsecuredEntityControl::sample()
}

#[uniffi::export]
pub fn new_unsecured_entity_control_sample_other() -> UnsecuredEntityControl {
    UnsecuredEntityControl::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = UnsecuredEntityControl;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_unsecured_entity_control_sample(),
                new_unsecured_entity_control_sample_other(),
                // duplicates should get removed
                new_unsecured_entity_control_sample(),
                new_unsecured_entity_control_sample_other(),
            ])
            .len(),
            2
        );
    }
}
