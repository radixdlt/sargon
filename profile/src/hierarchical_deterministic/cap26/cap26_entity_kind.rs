use std::fmt::Display;

use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::FromRepr;

use crate::HDPathValue;
use enum_as_inner::EnumAsInner;

/// Account or Identity (used by Personas) part of a CAP26 derivation
/// path.
#[derive(
    Serialize_repr,
    Deserialize_repr,
    FromRepr,
    Clone,
    Copy,
    Debug,
    EnumAsInner,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    uniffi::Enum,
)]
#[repr(u32)]
pub enum CAP26EntityKind {
    /// An account entity type
    Account = 525,

    /// Used by Persona
    Identity = 618,
}

impl Display for CAP26EntityKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl CAP26EntityKind {
    /// The raw representation of this entity kind, an `HDPathValue`.
    pub fn discriminant(&self) -> HDPathValue {
        *self as HDPathValue
    }

    fn description(&self) -> String {
        match self {
            Self::Account => "Account".to_string(),
            Self::Identity => "Identity".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::{assert_json_roundtrip, assert_json_value_eq_after_roundtrip, CAP26EntityKind};
    use serde_json::json;

    #[test]
    fn discriminant() {
        assert_eq!(CAP26EntityKind::Account.discriminant(), 525);
        assert_eq!(CAP26EntityKind::Identity.discriminant(), 618);
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", CAP26EntityKind::Account), "Account");
        assert_eq!(format!("{}", CAP26EntityKind::Identity), "Identity");
    }

    #[test]
    fn equality() {
        assert_eq!(CAP26EntityKind::Account, CAP26EntityKind::Account);
        assert_eq!(CAP26EntityKind::Identity, CAP26EntityKind::Identity);
    }
    #[test]
    fn inequality() {
        assert_ne!(CAP26EntityKind::Account, CAP26EntityKind::Identity);
    }

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter([CAP26EntityKind::Account, CAP26EntityKind::Account].into_iter())
                .len(),
            1
        );
    }

    #[test]
    fn ord() {
        assert!(CAP26EntityKind::Account < CAP26EntityKind::Identity);
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(&CAP26EntityKind::Account, json!(525));
        assert_json_roundtrip(&CAP26EntityKind::Account);
    }
}
