use crate::prelude::*;

pub trait IsMappableToGlobalKeySpace {
    fn map_to_global_key_space(&self) -> u32;
}

impl<T: IsInLocalKeySpace + HasOffsetFromGlobalKeySpace>
    IsMappableToGlobalKeySpace for T
{
    fn map_to_global_key_space(&self) -> u32 {
        let index_in_local = self.index_in_local_key_space();
        let index_in_global = u32::from(index_in_local);
        index_in_global + T::offset_from_global_key_space()
    }
}
