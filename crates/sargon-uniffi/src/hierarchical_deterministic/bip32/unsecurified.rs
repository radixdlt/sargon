use crate::prelude::*;
use sargon::Unsecurified as InternalUnsecurified;

use sargon::{
    FromGlobalKeySpace, FromLocalKeySpace, IsInLocalKeySpace,
    IsMappableToGlobalKeySpace, ToBIP32Str,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum Unsecurified {
    UnhardenedComponent(Unhardened),

    HardenedComponent(UnsecurifiedHardened),
}

impl From<InternalUnsecurified> for Unsecurified {
    fn from(value: InternalUnsecurified) -> Self {
        match value {
            InternalUnsecurified::Unhardened(unhardened) => {
                Self::UnhardenedComponent(unhardened.into())
            }
            InternalUnsecurified::Hardened(hardened) => {
                Self::HardenedComponent(hardened.into())
            }
        }
    }
}

impl From<Unsecurified> for InternalUnsecurified {
    fn from(value: Unsecurified) -> Self {
        match value {
            Unsecurified::UnhardenedComponent(unhardened) => {
                Self::Unhardened(unhardened.into())
            }
            Unsecurified::HardenedComponent(hardened) => {
                Self::Hardened(hardened.into())
            }
        }
    }
}

#[uniffi::export]
pub fn new_unsecurified_sample() -> Unsecurified {
    InternalUnsecurified::sample().into()
}

#[uniffi::export]
pub fn new_unsecurified_sample_other() -> Unsecurified {
    InternalUnsecurified::sample_other().into()
}
