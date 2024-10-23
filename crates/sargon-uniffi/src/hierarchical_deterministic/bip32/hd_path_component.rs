use crate::prelude::*;
use sargon::HDPathComponent as InternalHDPathComponent;
use sargon::Hardened as InternalHardened;
use sargon::SecurifiedU30 as InternalSecurifiedU30;
use sargon::Unhardened as InternalUnhardened;
use sargon::Unsecurified as InternalUnsecurified;
use sargon::UnsecurifiedHardened as InternalUnsecurifiedHardened;
use sargon::U30 as InternalU30;
use sargon::U31 as InternalU31;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum HDPathComponent {
    Unsecurified(Unsecurified),
    Securified(SecurifiedU30),
}

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum Hardened {
    Unsecurified(UnsecurifiedHardened),
    Securified(SecurifiedU30),
}

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum Unsecurified {
    Unhardened(Unhardened),

    Hardened(UnsecurifiedHardened),
}

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct Unhardened {
    secret_magic: U31,
}
impl From<Unhardened> for InternalUnhardened {
    fn from(value: Unhardened) -> InternalUnhardened {
        InternalUnhardened(value.secret_magic.into())
    }
}

impl From<InternalUnhardened> for Unhardened {
    fn from(value: InternalUnhardened) -> Unhardened {
        Unhardened {
            secret_magic: U31::from(value.0),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct SecurifiedU30 {
    secret_magic: U30,
}
impl From<SecurifiedU30> for InternalSecurifiedU30 {
    fn from(value: SecurifiedU30) -> InternalSecurifiedU30 {
        InternalSecurifiedU30(value.secret_magic.into())
    }
}

impl From<InternalSecurifiedU30> for SecurifiedU30 {
    fn from(value: InternalSecurifiedU30) -> SecurifiedU30 {
        SecurifiedU30 {
            secret_magic: U30::from(value.0),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct UnsecurifiedHardened {
    secret_magic: U30,
}
impl From<UnsecurifiedHardened> for InternalUnsecurifiedHardened {
    fn from(value: UnsecurifiedHardened) -> InternalUnsecurifiedHardened {
        InternalUnsecurifiedHardened(value.secret_magic.into())
    }
}

impl From<InternalUnsecurifiedHardened> for UnsecurifiedHardened {
    fn from(value: InternalUnsecurifiedHardened) -> UnsecurifiedHardened {
        UnsecurifiedHardened {
            secret_magic: U30::from(value.0),
        }
    }
}

use sargon::IsInLocalKeySpace;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct U31 {
    secret_magic: u32,
}
impl From<U31> for InternalU31 {
    fn from(value: U31) -> InternalU31 {
        InternalU31::try_from(value.secret_magic)
            .expect("InternalConversion should always work")
    }
}

impl From<InternalU31> for U31 {
    fn from(value: InternalU31) -> U31 {
        U31 {
            secret_magic: value.0.into(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct U30 {
    secret_magic: u32,
}
impl From<U30> for InternalU30 {
    fn from(value: U30) -> InternalU30 {
        InternalU30::try_from(value.secret_magic)
            .expect("InternalConversion should always work")
    }
}

impl From<InternalU30> for U30 {
    fn from(value: InternalU30) -> U30 {
        U30 {
            secret_magic: value.0.into(),
        }
    }
}
