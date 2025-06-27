use crate::prelude::*;
use sargon::U11 as InternalU11;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record,
)]
pub struct U11 {
    pub inner: u16,
}
