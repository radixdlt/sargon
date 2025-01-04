use crate::prelude::*;

pub trait AddViaGlobalKeySpace:
    IsMappableToGlobalKeySpace + FromGlobalKeySpace + IsInLocalKeySpace
{
    fn checked_add_one_to_global(&self) -> Result<Self> {
        self.checked_add_n_to_global(1)
    }

    fn checked_add_n_to_global(&self, n: u32) -> Result<Self> {
        let key_space_before = self.key_space();
        let global = self
            .map_to_global_key_space()
            .checked_add(n)
            .ok_or(CommonError::IndexOverflow)?;
        let sum = Self::from_global_key_space(global)?;
        let key_space_after = sum.key_space();
        if key_space_after != key_space_before {
            return Err(
                CommonError::CannotAddMoreToIndexSinceItWouldChangeKeySpace,
            );
        }
        Ok(sum)
    }
}
