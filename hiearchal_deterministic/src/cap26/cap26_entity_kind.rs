use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::FromRepr;

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
#[repr(u32)]
pub enum CAP26EntityKind {
    /// An account entity type
    Account = 525,

    /// Used by Persona
    Identity = 618,
}
