use crate::prelude::*;

/// Flags used to mark state of an Account or Persona such as whether
/// user has marked it as deleted or not.
#[derive(
    Serialize,
    Deserialize,
    FromRepr,
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
)]
#[serde(rename_all = "camelCase")]
pub enum EntityFlag {
    /// The entity is marked as deleted by user. Entity should still be kept in Profile
    DeletedByUser,

    /// Just a temporary placeholder value used by Sample Values.
    PlaceholderSampleValueFlag,
}

impl HasSampleValues for EntityFlag {
    fn sample() -> Self {
        Self::DeletedByUser
    }

    fn sample_other() -> Self {
        Self::PlaceholderSampleValueFlag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EntityFlag;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(
            &SUT::DeletedByUser,
            json!("deletedByUser"),
        );
        assert_json_roundtrip(&SUT::DeletedByUser);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::DeletedByUser), "DeletedByUser");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SUT::DeletedByUser), "DeletedByUser");
    }
}
