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

#[uniffi::export]
pub fn new_threshold_sample() -> Threshold {
    InternalThreshold::sample().into()
}

#[uniffi::export]
pub fn new_threshold_sample_other() -> Threshold {
    InternalThreshold::sample_other().into()
}
