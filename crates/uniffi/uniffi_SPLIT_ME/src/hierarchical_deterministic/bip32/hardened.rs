use crate::prelude::*;
use sargon::Hardened as InternalHardened;

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
)]
pub enum Hardened {
    Unsecurified(UnsecurifiedHardened),
    Securified(SecurifiedU30),
}

#[uniffi::export]
pub fn new_hardened_sample() -> Hardened {
    InternalHardened::sample().into()
}

#[uniffi::export]
pub fn new_hardened_sample_other() -> Hardened {
    InternalHardened::sample_other().into()
}
