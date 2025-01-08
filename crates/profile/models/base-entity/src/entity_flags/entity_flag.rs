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
    ///
    /// For compatibility, it is still serialised as `deletedByUser`
    #[serde(rename = "deletedByUser")]
    HiddenByUser,

    /// The entity is marked as tombstoned by the user. Entity should still be kept in Profile
    /// Such an entity cannot be involved in any transaction anymore.
    TombstonedByUser,
}

impl HasSampleValues for EntityFlag {
    fn sample() -> Self {
        Self::HiddenByUser
    }

    fn sample_other() -> Self {
        Self::TombstonedByUser
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
    fn json_roundtrip_hidden() {
        assert_json_value_eq_after_roundtrip(
            &SUT::HiddenByUser,
            json!("deletedByUser"),
        );
        assert_json_roundtrip(&SUT::HiddenByUser);
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
        assert_eq!(format!("{}", SUT::HiddenByUser), "HiddenByUser");
        assert_eq!(format!("{}", SUT::TombstonedByUser), "TombstonedByUser");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SUT::HiddenByUser), "HiddenByUser");
        assert_eq!(format!("{:?}", SUT::TombstonedByUser), "TombstonedByUser");
    }
}
