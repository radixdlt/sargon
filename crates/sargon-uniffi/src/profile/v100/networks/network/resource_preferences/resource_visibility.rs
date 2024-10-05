use crate::prelude::*;

/// Indicates the visibility of a resource.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    enum_iterator::Sequence,
    derive_more::Display,
    uniffi::Enum,
)]
pub enum ResourceVisibility {
    Hidden,
    Visible,
}