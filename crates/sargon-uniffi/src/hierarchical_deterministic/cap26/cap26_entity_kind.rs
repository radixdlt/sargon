use crate::prelude::*;
use sargon::CAP26EntityKind as InternalCAP26EntityKind;

/// Account or Identity (used by Personas) part of a CAP26 derivation
/// path.
#[derive(
    Clone,
    
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

macro_rules! delegate_display_debug_into {
    ($external_type:ty, $internal_type:ty) => {
        impl std::fmt::Display for $external_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let internal: $internal_type = self.clone().into();
                write!(f, "{}", internal)
            }
        }

        impl std::fmt::Debug for $external_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let internal: $internal_type = self.clone().into();
                write!(f, "{:?}", internal)
            }
        }
    };
}

pub(crate) use delegate_display_debug_into;

delegate_display_debug_into!(CAP26EntityKind, InternalCAP26EntityKind);

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