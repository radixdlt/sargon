use crate::prelude::*;
use sargon::CAP26EntityKind as InternalCAP26EntityKind;

/// Account or Identity (used by Personas) part of a CAP26 derivation
/// path.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
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
                let internal = self.into_internal();
                write!(f, "{}", internal)
            }
        }

        impl std::fmt::Debug for $external_type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let internal = self.into_internal();
                write!(f, "{:?}", internal)
            }
        }
    };
}

pub(crate) use delegate_display_debug_into;

delegate_display_debug_into!(CAP26EntityKind, InternalCAP26EntityKind);
