use crate::prelude::*;
use sargon::Unsecurified as InternalUnsecurified;

use sargon::{
    FromGlobalKeySpace, FromLocalKeySpace, IsInLocalKeySpace,
    IsMappableToGlobalKeySpace, ToBIP32Str,
};

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
)]
pub enum Unsecurified {
    Unhardened(Unhardened),

    Hardened(UnsecurifiedHardened),
}

#[uniffi::export]
pub fn new_unsecurified_sample() -> Unsecurified {
    InternalUnsecurified::sample().into()
}

#[uniffi::export]
pub fn new_unsecurified_sample_other() -> Unsecurified {
    InternalUnsecurified::sample_other().into()
}
