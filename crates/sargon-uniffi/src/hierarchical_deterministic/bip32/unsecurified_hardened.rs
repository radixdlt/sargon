use crate::prelude::*;
use sargon::UnsecurifiedHardened as InternalUnsecurifiedHardened;

use sargon::{
    FromGlobalKeySpace, FromLocalKeySpace, IsInLocalKeySpace,
    IsMappableToGlobalKeySpace, ToBIP32Str,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
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

#[uniffi::export]
pub fn new_unsecurified_hardened(u30: U30) -> UnsecurifiedHardened {
    UnsecurifiedHardened { secret_magic: u30 }
}

#[uniffi::export]
pub fn new_unsecurified_hardened_sample() -> UnsecurifiedHardened {
    InternalUnsecurifiedHardened::sample().into()
}

#[uniffi::export]
pub fn new_unsecurified_hardened_sample_other() -> UnsecurifiedHardened {
    InternalUnsecurifiedHardened::sample_other().into()
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
) -> u32 {
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
mod tests {
    use super::*;

    #[test]
    fn test_unsecurified_hardened_samples() {
        assert_eq!(
            new_unsecurified_hardened_sample(),
            new_unsecurified_hardened_sample()
        );
        assert_eq!(
            new_unsecurified_hardened_sample_other(),
            new_unsecurified_hardened_sample_other()
        );
        assert_ne!(
            new_unsecurified_hardened_sample(),
            new_unsecurified_hardened_sample_other()
        );
    }

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
    fn test_new_unsecurified_hardened_from_global_key_space_underflow() {
        assert!(new_unsecurified_hardened_from_global_key_space(1).is_err());
    }

    #[test]
    fn test_new_unsecurified_hardened_from_global_key_space_overflow() {
        assert!(new_unsecurified_hardened_from_global_key_space(
            0x4000_0000 + 0x8000_0000
        )
        .is_err());
    }

    #[test]
    fn test_unsecurified_hardened_index_in_local_key_space() {
        assert_eq!(
            unsecurified_hardened_index_in_local_key_space(
                new_unsecurified_hardened(new_u30(3).unwrap())
            ),
            3
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
