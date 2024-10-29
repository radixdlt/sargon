use crate::prelude::*;
use sargon::Unhardened as InternalUnhardened;

use sargon::{
    FromGlobalKeySpace, FromLocalKeySpace, IsInLocalKeySpace,
    IsMappableToGlobalKeySpace, ToBIP32Str,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
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

#[uniffi::export]
pub fn new_unhardened_sample() -> Unhardened {
    InternalUnhardened::sample().into()
}

#[uniffi::export]
pub fn new_unhardened_sample_other() -> Unhardened {
    InternalUnhardened::sample_other().into()
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
pub fn unhardened_index_in_local_key_space(unhardened: Unhardened) -> u32 {
    InternalUnhardened::from(unhardened)
        .index_in_local_key_space()
        .into()
}

#[uniffi::export]
pub fn unhardened_index_in_global_key_space(unhardened: Unhardened) -> u32 {
    InternalUnhardened::from(unhardened).map_to_global_key_space()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_unhardened() {
        assert_eq!(
            new_unhardened(new_u31(8).unwrap()),
            new_unhardened_from_local_key_space(8).unwrap()
        );
    }

    #[test]
    fn test_unhardened_samples() {
        assert_eq!(new_unhardened_sample(), new_unhardened_sample());
        assert_eq!(
            new_unhardened_sample_other(),
            new_unhardened_sample_other()
        );
        assert_ne!(new_unhardened_sample(), new_unhardened_sample_other());
    }

    #[test]
    fn test_new_unhardened_from_global_key_space() {
        assert!(new_unhardened_from_global_key_space(0x8000_0000).is_err());
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
            4
        );
    }
}
