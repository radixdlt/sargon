pub use crate::prelude::*;
use sargon::Epoch as InternalEpoch;

/// A type-safe consensus epoch number.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
pub struct Epoch {
    pub value: u64,
}

impl From<InternalEpoch> for Epoch {
    fn from(value: InternalEpoch) -> Self {
        Self { value: value.0 }
    }
}

impl Into<InternalEpoch> for Epoch {
    fn into(self) -> InternalEpoch {
        InternalEpoch(self.value)
    }
}