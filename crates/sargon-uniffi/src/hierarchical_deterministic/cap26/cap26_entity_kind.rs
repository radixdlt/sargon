use crate::prelude::*;

/// Account or Identity (used by Personas) part of a CAP26 derivation
/// path.
#[derive(
    FromRepr,
    Clone,
    Copy,
    EnumAsInner,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Enum,
)]
pub enum CAP26EntityKind {
    /// An Account entity type
    #[display("Account")]
    Account = 525,

    /// An Identity entity type (used by Personas)
    #[display("Identity")]
    Identity = 618,
}