use crate::prelude::*;

#[derive(
    Zeroize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
pub struct U11 {
    pub inner: u16,
}