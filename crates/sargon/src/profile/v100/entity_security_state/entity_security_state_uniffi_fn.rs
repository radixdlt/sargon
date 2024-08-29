use crate::prelude::*;

#[uniffi::export]
pub fn new_entity_security_state_sample() -> EntitySecurityState {
    EntitySecurityState::sample()
}

#[uniffi::export]
pub fn new_entity_security_state_sample_other() -> EntitySecurityState {
    EntitySecurityState::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EntitySecurityState;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_entity_security_state_sample(),
                new_entity_security_state_sample_other(),
                // duplicates should get removed
                new_entity_security_state_sample(),
                new_entity_security_state_sample_other(),
            ])
            .len(),
            2
        );
    }
}
