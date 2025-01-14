use crate::prelude::*;
use sargon::Threshold as InternalThreshold;

/// A kind of threshold, either All or a specific number of factors
/// must be used to perform some function with.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
)]
pub enum Threshold {
    /// All factors in the threshold factors list must be used to perform some function with
    All,
    /// A specific number of factors in the threshold factors list must be used to perform some function with
    Specific(u8),
}
