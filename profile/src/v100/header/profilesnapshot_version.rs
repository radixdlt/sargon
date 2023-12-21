use std::fmt::Display;

use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::FromRepr;

/// The
#[derive(
    Serialize_repr,
    Deserialize_repr,
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[repr(u16)] // most likely will will not do more than 65536 iterations.
pub enum ProfileSnapshotVersion {
    /// The version we went live with on Babylon mainnet 2023-09-28,
    /// shipped with iOS 1.0.0 (7) and Android v 1.0.0.
    V100 = 100,
}

impl Default for ProfileSnapshotVersion {
    fn default() -> Self {
        Self::V100
    }
}

impl ProfileSnapshotVersion {
    pub fn discriminant(&self) -> u16 {
        *self as u16
    }
}

impl Display for ProfileSnapshotVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.discriminant())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_common::assert_json::{
        assert_json_value_eq_after_roundtrip, assert_json_value_fails,
    };

    use super::ProfileSnapshotVersion;

    #[test]
    fn json() {
        assert_json_value_eq_after_roundtrip(&ProfileSnapshotVersion::V100, json!(100));
        assert_json_value_fails::<ProfileSnapshotVersion>(json!(99));
        assert_json_value_fails::<ProfileSnapshotVersion>(json!("99"));
        assert_json_value_fails::<ProfileSnapshotVersion>(json!("100"));
        assert_json_value_fails::<ProfileSnapshotVersion>(json!("v100"));
        assert_json_value_fails::<ProfileSnapshotVersion>(json!("V100"));
    }

    #[test]
    fn from_repr() {
        assert_eq!(
            ProfileSnapshotVersion::V100,
            ProfileSnapshotVersion::from_repr(100).unwrap()
        )
    }

    #[test]
    fn from_repr_unknown_version() {
        assert!(ProfileSnapshotVersion::from_repr(99).is_none())
    }

    #[test]
    fn discriminant() {
        assert_eq!(ProfileSnapshotVersion::V100.discriminant(), 100)
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", ProfileSnapshotVersion::V100), "100")
    }
}
