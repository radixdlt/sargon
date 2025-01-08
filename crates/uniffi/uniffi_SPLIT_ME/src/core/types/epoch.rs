pub use crate::prelude::*;
use sargon::Epoch as InternalEpoch;

uniffi::custom_newtype!(Epoch, u64);
/// A type-safe consensus epoch number.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion)]
pub struct Epoch(pub u64);
