use crate::prelude::*;
use sargon::ProfileID as InternalProfileID;

/// A stable and globally unique identifier of a Profile.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    uniffi::Record,
)]
pub struct ProfileID {
    value: Uuid,
}

impl From<InternalProfileID> for ProfileID {
    fn from(value: InternalProfileID) -> Self {
        Self {
            value: value.0,
        }
    }
}

impl Into<InternalProfileID> for ProfileID {
    fn into(self) -> InternalProfileID {
        InternalProfileID(self.value)
    }
}

#[uniffi::export]
pub fn new_profile_id_sample() -> ProfileID {
    InternalProfileID::sample().into()
}

#[uniffi::export]
pub fn new_profile_id_sample_other() -> ProfileID {
    InternalProfileID::sample_other().into()
}

#[cfg(test)]
mod uniffi_test {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileID;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_profile_id_sample(),
                new_profile_id_sample_other(),
                // duplicates should get removed
                new_profile_id_sample(),
                new_profile_id_sample_other(),
            ])
            .len(),
            2
        );
    }
}
