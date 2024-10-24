use crate::prelude::*;

pub trait FromGlobalKeySpace: Sized {
    fn from_global_key_space(value: u32) -> Result<Self>;
}

impl<T: FromLocalKeySpace + HasOffsetFromGlobalKeySpace> FromGlobalKeySpace
    for T
{
    fn from_global_key_space(value: u32) -> Result<Self> {
        value
            .checked_sub(T::offset_from_global_key_space())
            .ok_or(CommonError::IndexInGlobalKeySpaceIsLowerThanOffset)
            .and_then(T::from_local_key_space)
    }
}
