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
    uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
pub enum EntityFlag {
    /// The entity is marked as deleted by user. Entity should still be kept in Profile
    DeletedByUser,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(&EntityFlag::DeletedByUser, json!("deletedByUser"));
        assert_json_roundtrip(&EntityFlag::DeletedByUser);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", EntityFlag::DeletedByUser), "DeletedByUser");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", EntityFlag::DeletedByUser), "DeletedByUser");
    }
}
