use crate::prelude::*;

/// Flags used to mark state of an Account or Persona such as whether
/// user has marked it as deleted or not.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    enum_iterator::Sequence,
    derive_more::Display,
    uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
pub enum EntityFlag {
    /// The entity is marked as deleted by user. Entity should still be kept in Profile
    DeletedByUser,

    /// Just a temporary placeholder value used by Sample Values.
    PlaceholderSampleValueFlag,
}

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
