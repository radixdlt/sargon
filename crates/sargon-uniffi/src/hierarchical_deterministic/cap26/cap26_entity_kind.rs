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

delegate_display_debug_into!(CAP26EntityKind, InternalCAP26EntityKind);

#[uniffi::export]
pub fn cap26_entity_kind_to_string(kind: CAP26EntityKind) -> String {
    kind.into_internal().to_string()
}
