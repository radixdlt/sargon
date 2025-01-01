use crate::prelude::*;
use sargon::{HasIndexInLocalKeySpace, SecurifiedU30 as InternalSecurifiedU30};

use sargon::{
    FromGlobalKeySpace, FromLocalKeySpace, IsMappableToGlobalKeySpace,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
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

#[uniffi::export]
pub fn new_securified_sample() -> SecurifiedU30 {
    InternalSecurifiedU30::sample().into()
}

#[uniffi::export]
pub fn new_securified_sample_other() -> SecurifiedU30 {
    InternalSecurifiedU30::sample_other().into()
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
pub fn securified_index_in_local_key_space(securified: SecurifiedU30) -> u32 {
    InternalSecurifiedU30::from(securified)
        .index_in_local_key_space()
        .into()
}

#[uniffi::export]
pub fn securified_index_in_global_key_space(securified: SecurifiedU30) -> u32 {
    InternalSecurifiedU30::from(securified).map_to_global_key_space()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_securified() {
        assert_eq!(
            new_securified(new_u30(4).unwrap()),
            new_securified_from_local_key_space(4).unwrap()
        );
    }

    #[test]
    fn test_securified_samples() {
        assert_eq!(new_securified_sample(), new_securified_sample());
        assert_eq!(
            new_securified_sample_other(),
            new_securified_sample_other()
        );
        assert_ne!(new_securified_sample(), new_securified_sample_other());
    }

    #[test]
    fn test_new_securified_from_global_key_space_underflow() {
        assert!(new_securified_from_global_key_space(1).is_err());
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
            5
        );
    }
}
