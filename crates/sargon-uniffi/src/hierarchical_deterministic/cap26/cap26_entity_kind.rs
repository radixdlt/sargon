use crate::prelude::*;
use sargon::CAP26EntityKind as InternalCAP26EntityKind;

/// Account or Identity (used by Personas) part of a CAP26 derivation
/// path.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum CAP26EntityKind {
    /// An Account entity type
    Account = 525,

    /// An Identity entity type (used by Personas)
    Identity = 618,
}

impl From<InternalCAP26EntityKind> for CAP26EntityKind {
    fn from(value: InternalCAP26EntityKind) -> Self {
        match value {
            InternalCAP26EntityKind::Account => Self::Account,
            InternalCAP26EntityKind::Identity => Self::Identity,
        }
    }
}

impl Into<InternalCAP26EntityKind> for CAP26EntityKind {
    fn into(self) -> InternalCAP26EntityKind {
        match self {
            Self::Account => InternalCAP26EntityKind::Account,
            Self::Identity => InternalCAP26EntityKind::Identity,
        }
    }
}