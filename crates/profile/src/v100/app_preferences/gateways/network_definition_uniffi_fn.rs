use crate::prelude::*;

#[uniffi::export]
pub fn new_network_definition_lookup_by_name(
    logical_name: String,
) -> Result<NetworkDefinition> {
    NetworkDefinition::lookup_by_name(&logical_name)
}

#[uniffi::export]
pub fn new_network_definition_sample() -> NetworkDefinition {
    NetworkDefinition::sample()
}

#[uniffi::export]
pub fn new_network_definition_sample_other() -> NetworkDefinition {
    NetworkDefinition::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NetworkDefinition;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_network_definition_sample(),
                new_network_definition_sample_other(),
                // duplicates should get removed
                new_network_definition_sample(),
                new_network_definition_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_new_network_definition_lookup_by_name() {
        assert_eq!(
            new_network_definition_lookup_by_name(SUT::sample().logical_name)
                .unwrap(),
            SUT::sample()
        );
    }
}
