use crate::prelude::*;

/// Basic security control of an unsecured entity. When said entity
/// is "securified" it will no longer be controlled by this `UnsecuredEntityControl`
/// but rather by an `AccessControl`. It is a name space holding the
/// single factor instance which was used to create
#[derive(
    Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
pub struct UnsecuredEntityControl {
    // /// The factor instance which was used to create this unsecured entity, which
    // /// also controls this entity and is used for signing transactions.
    pub transaction_signing: HierarchicalDeterministicFactorInstance,

    /// The factor instance which can be used for ROLA.
    pub authentication_signing: Option<HierarchicalDeterministicFactorInstance>,
}

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
