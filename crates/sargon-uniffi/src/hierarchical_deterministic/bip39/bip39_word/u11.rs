use crate::prelude::*;

#[derive(
    Zeroize,
    
    Clone,
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
pub struct U11 {
    pub inner: u16,
}