use crate::prelude::*;

#[derive(
    Zeroize,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    derive_more::Display,
    Ord,
    Hash,
    uniffi::Record,
)]
#[display("{inner}")]
pub struct U11 {
    pub inner: u16,
}