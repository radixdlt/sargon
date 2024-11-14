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
    /// The entity is marked as hidden by user. Entity should still be kept in Profile
    /// The user can "unhide" the entity and continue involving it in transactions on ledger.
    DeletedByUser,

    /// The entity is marked as tombstoned by the user. Entity should still be kept in Profile
    /// Such an entity cannot be involved in any transaction anymore.
    TombstonedByUser,
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
    fn json_roundtrip_deleted() {
        assert_json_value_eq_after_roundtrip(
            &SUT::DeletedByUser,
            json!("deletedByUser"),
        );
        assert_json_roundtrip(&SUT::DeletedByUser);
    }

    #[test]
    fn json_roundtrip_tombstoned() {
        assert_json_value_eq_after_roundtrip(
            &SUT::TombstonedByUser,
            json!("tombstonedByUser"),
        );
        assert_json_roundtrip(&SUT::TombstonedByUser);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::DeletedByUser), "DeletedByUser");
        assert_eq!(format!("{}", SUT::TombstonedByUser), "TombstonedByUser");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SUT::DeletedByUser), "DeletedByUser");
        assert_eq!(format!("{:?}", SUT::TombstonedByUser), "TombstonedByUser");
    }
}
