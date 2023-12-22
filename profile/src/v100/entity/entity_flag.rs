use std::fmt::Display;

use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use strum::FromRepr;

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
    Sequence,
)]
#[serde(rename_all = "camelCase")]
pub enum EntityFlag {
    /// The entity is marked as deleted by user. Entity should still be kept in Profile
    DeletedByUser,
}

impl EntityFlag {
    /// Human readable form of the flag
    pub fn discriminant(&self) -> String {
        format!("{}", self)
    }
}

impl Display for EntityFlag {
    fn fmt(
        &self,
        f: &mut radix_engine_common::prelude::fmt::Formatter<'_>,
    ) -> radix_engine_common::prelude::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_json_roundtrip, assert_json_value_eq_after_roundtrip};
    use serde_json::json;

    use super::EntityFlag;

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(&EntityFlag::DeletedByUser, json!("deletedByUser"));
        assert_json_roundtrip(&EntityFlag::DeletedByUser);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", EntityFlag::DeletedByUser), "DeletedByUser");

        // `discriminant` uses Display
        assert_eq!(EntityFlag::DeletedByUser.discriminant(), "DeletedByUser");
    }
}
