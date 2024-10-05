pub use crate::prelude::*;

// Generate the FfiConverter needed by UniFFI for newtype `Epoch`.
uniffi::custom_newtype!(Epoch, u64);

/// A type-safe consensus epoch number.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    derive_more::Display,
    derive_more::Debug,
)]
pub struct Epoch(pub u64);