use crate::prelude::*;

use sargon::U31 as InternalU31;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct U31 {
    pub secret_magic: u32,
}

#[uniffi::export]
pub fn new_u31_sample() -> U31 {
    InternalU31::sample().into()
}

#[uniffi::export]
pub fn new_u31_sample_other() -> U31 {
    InternalU31::sample_other().into()
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
            secret_magic: value.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_u31() {
        assert_eq!(new_u31(0).unwrap(), U31 { secret_magic: 0 });
    }

    #[test]
    fn test_u31_samples() {
        assert_eq!(new_u31_sample(), new_u31_sample());
        assert_eq!(new_u31_sample_other(), new_u31_sample_other());
        assert_ne!(new_u31_sample(), new_u31_sample_other());
    }

    #[test]
    fn test_new_u31_overflow() {
        assert!(new_u31(0xffffffff).is_err());
    }

    #[test]
    fn test_u31_get_value() {
        assert_eq!(u31_get_value(new_u31(1337).unwrap()), 1337);
    }
}
