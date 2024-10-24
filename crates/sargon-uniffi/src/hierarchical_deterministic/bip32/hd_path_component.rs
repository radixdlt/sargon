use crate::prelude::*;
use sargon::FromGlobalKeySpace;
use sargon::FromLocalKeySpace;
use sargon::HDPathComponent as InternalHDPathComponent;
use sargon::Hardened as InternalHardened;
use sargon::SecurifiedU30 as InternalSecurifiedU30;
use sargon::Unhardened as InternalUnhardened;
use sargon::Unsecurified as InternalUnsecurified;
use sargon::UnsecurifiedHardened as InternalUnsecurifiedHardened;
use sargon::U30 as InternalU30;
use sargon::U31 as InternalU31;

use sargon::IsInLocalKeySpace;
use sargon::IsMappableToGlobalKeySpace;

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
)]
pub enum HDPathComponent {
    Unsecurified(Unsecurified),
    Securified(SecurifiedU30),
}

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
)]
pub enum Hardened {
    Unsecurified(UnsecurifiedHardened),
    Securified(SecurifiedU30),
}

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
)]
pub enum Unsecurified {
    Unhardened(Unhardened),

    Hardened(UnsecurifiedHardened),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct Unhardened {
    secret_magic: U31,
}

#[uniffi::export]
pub fn new_unhardened(u31: U31) -> Unhardened {
    Unhardened { secret_magic: u31 }
}

#[uniffi::export]
pub fn new_unhardened_from_local_key_space(value: u32) -> Result<Unhardened> {
    InternalUnhardened::from_local_key_space(value).into_result()
}

#[uniffi::export]
pub fn new_unhardened_from_global_key_space(value: u32) -> Result<Unhardened> {
    InternalUnhardened::from_global_key_space(value).into_result()
}

#[uniffi::export]
pub fn unhardened_index_in_local_key_space(unhardened: Unhardened) -> U31 {
    InternalUnhardened::from(unhardened)
        .index_in_local_key_space()
        .into()
}

#[uniffi::export]
pub fn unhardened_index_in_global_key_space(unhardened: Unhardened) -> u32 {
    InternalUnhardened::from(unhardened).map_to_global_key_space()
}

#[cfg(test)]
mod tests4 {
    use super::*;

    #[test]
    fn test_new_unhardened() {
        assert_eq!(
            new_unhardened(new_u31(8).unwrap()),
            new_unhardened_from_local_key_space(8).unwrap()
        );
    }

    #[test]
    fn test_new_unhardened_from_global_key_space() {
        assert_eq!(
            new_unhardened_from_global_key_space(9).unwrap(),
            new_unhardened_from_local_key_space(9).unwrap()
        );
    }

    #[test]
    fn test_unhardened_index_in_global_key_space() {
        assert_eq!(
            unhardened_index_in_global_key_space(new_unhardened(
                new_u31(4).unwrap()
            )),
            4
        );
    }

    #[test]
    fn test_unhardened_index_in_local_key_space() {
        assert_eq!(
            unhardened_index_in_local_key_space(new_unhardened(
                new_u31(4).unwrap()
            )),
            new_u31(4).unwrap()
        );
    }
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct SecurifiedU30 {
    secret_magic: U30,
}

#[uniffi::export]
pub fn new_securified(u30: U30) -> SecurifiedU30 {
    SecurifiedU30 { secret_magic: u30 }
}

#[uniffi::export]
pub fn new_securified_from_local_key_space(
    value: u32,
) -> Result<SecurifiedU30> {
    InternalSecurifiedU30::from_local_key_space(value).into_result()
}

#[uniffi::export]
pub fn new_securified_from_global_key_space(
    value: u32,
) -> Result<SecurifiedU30> {
    InternalSecurifiedU30::from_global_key_space(value).into_result()
}

#[uniffi::export]
pub fn securified_index_in_local_key_space(securified: SecurifiedU30) -> U31 {
    InternalSecurifiedU30::from(securified)
        .index_in_local_key_space()
        .into()
}

#[uniffi::export]
pub fn securified_index_in_global_key_space(securified: SecurifiedU30) -> u32 {
    InternalSecurifiedU30::from(securified).map_to_global_key_space()
}

#[cfg(test)]
mod tests3 {
    use super::*;

    #[test]
    fn test_new_securified() {
        assert_eq!(
            new_securified(new_u30(4).unwrap()),
            new_securified_from_local_key_space(4).unwrap()
        );
    }

    #[test]
    fn test_securified_index_in_global_key_space() {
        assert_eq!(
            securified_index_in_global_key_space(new_securified(
                new_u30(4).unwrap()
            )),
            4 + 0x8000_0000 + 0x4000_0000
        );
    }

    #[test]
    fn test_securified_index_in_local_key_space() {
        assert_eq!(
            securified_index_in_local_key_space(
                new_securified_from_global_key_space(
                    5 + 0x8000_0000 + 0x4000_0000
                )
                .unwrap()
            ),
            new_u31(5).unwrap()
        );
    }
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct UnsecurifiedHardened {
    secret_magic: U30,
}

#[uniffi::export]
pub fn new_unsecurified_hardened(u30: U30) -> UnsecurifiedHardened {
    UnsecurifiedHardened { secret_magic: u30 }
}

#[uniffi::export]
pub fn new_unsecurified_hardened_from_local_key_space(
    value: u32,
) -> Result<UnsecurifiedHardened> {
    InternalUnsecurifiedHardened::from_local_key_space(value).into_result()
}

#[uniffi::export]
pub fn new_unsecurified_hardened_from_global_key_space(
    value: u32,
) -> Result<UnsecurifiedHardened> {
    InternalUnsecurifiedHardened::from_global_key_space(value).into_result()
}

#[uniffi::export]
pub fn unsecurified_hardened_index_in_local_key_space(
    unsecurified_hardened: UnsecurifiedHardened,
) -> U31 {
    InternalUnsecurifiedHardened::from(unsecurified_hardened)
        .index_in_local_key_space()
        .into()
}

#[uniffi::export]
pub fn unsecurified_hardened_index_in_global_key_space(
    unsecurified_hardened: UnsecurifiedHardened,
) -> u32 {
    InternalUnsecurifiedHardened::from(unsecurified_hardened)
        .map_to_global_key_space()
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_unsecurified_hardened_index_in_global_key_space() {
        assert_eq!(
            unsecurified_hardened_index_in_global_key_space(
                new_unsecurified_hardened(new_u30(4).unwrap())
            ),
            4 + 0x8000_0000
        );
    }

    #[test]
    fn test_unsecurified_hardened_index_in_local_key_space() {
        assert_eq!(
            unsecurified_hardened_index_in_local_key_space(
                new_unsecurified_hardened(new_u30(3).unwrap())
            ),
            new_u31(3).unwrap()
        );
    }

    #[test]
    fn test_new_unsecurified_hardened_from_local_key_space() {
        assert_eq!(
            new_unsecurified_hardened_from_local_key_space(1)
                .unwrap()
                .secret_magic
                .secret_magic,
            1
        )
    }

    #[test]
    fn test_new_unsecurified_hardened_from_global_key_space() {
        assert_eq!(
            new_unsecurified_hardened_from_global_key_space(2 + 0x8000_0000)
                .unwrap()
                .secret_magic
                .secret_magic,
            2
        )
    }
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct U31 {
    secret_magic: u32,
}

#[uniffi::export]
pub fn new_u31(value: u32) -> Result<U31> {
    InternalU31::try_from(value).into_result()
}
#[uniffi::export]
pub fn u31_get_value(u31: U31) -> u32 {
    u31.secret_magic
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct U30 {
    secret_magic: u32,
}

#[uniffi::export]
pub fn new_u30(value: u32) -> Result<U30> {
    InternalU30::try_from(value).into_result()
}

#[uniffi::export]
pub fn u30_get_value(u30: U30) -> u32 {
    u30.secret_magic
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_u30() {
        assert_eq!(new_u30(0).unwrap(), U30 { secret_magic: 0 });
    }

    #[test]
    fn test_u30_get_value() {
        assert_eq!(u30_get_value(new_u30(1337).unwrap()), 1337);
    }

    #[test]
    fn test_new_u31() {
        assert_eq!(new_u31(0).unwrap(), U31 { secret_magic: 0 });
    }

    #[test]
    fn test_u31_get_value() {
        assert_eq!(u31_get_value(new_u31(1337).unwrap()), 1337);
    }
}
