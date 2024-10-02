use crate::prelude::*;

/// Type of a wallet Radix Entity - Account or Identity (used by Personas).
///
/// CAP26 uses this type to create separate key spaces for Accounts and Identities
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
)]
#[repr(u32)] // it is u32 since used in Derivation Paths (CAP26) where each component is a u32.
pub enum AbstractEntityType {
    /// The entity type used by Accounts.
    Account,
    /// The entity type used by Personas.
    Identity,
}
