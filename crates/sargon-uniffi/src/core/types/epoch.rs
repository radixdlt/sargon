pub use crate::prelude::*;
use sargon::Epoch as InternalEpoch;

/// A type-safe consensus epoch number.
#[derive(
    Clone,
    
    PartialEq,
    Eq,
    Hash,
    
    
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