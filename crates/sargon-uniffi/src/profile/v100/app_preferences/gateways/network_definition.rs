use crate::prelude::*;

use crate::NetworkID::{self, *};

/// A version of the Radix Network, for a NetworkID with an identifier (name) and display description (display name)
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{} ({})", self.display_description, self.id.discriminant())]
pub struct NetworkDefinition {
    /// A String identifier (always lowercase) with the name of the Network that MUST match what Gateway returns.
    pub logical_name: String,

    /// The canonical identifier of this network.
    pub id: NetworkID,

    /// A name of the network intended for display purposes only.
    pub display_description: String,
}

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
