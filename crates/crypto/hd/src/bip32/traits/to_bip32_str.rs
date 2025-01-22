use crate::prelude::*;

pub trait ToBIP32Str: Sized {
    fn to_bip32_string(&self) -> String;
    fn to_bip32_string_debug(&self) -> String;
}

impl<T> ToBIP32Str for T
where
    T: IsPathComponentStringConvertible + IsInLocalKeySpace,
{
    fn to_bip32_string(&self) -> String {
        format!(
            "{}{}",
            u32::from(self.index_in_local_key_space()),
            T::VERBOSE_SYNTAX_SUFFIX
        )
    }
    fn to_bip32_string_debug(&self) -> String {
        format!(
            "{}{}",
            u32::from(self.index_in_local_key_space()),
            T::SHORTHAND_SYNTAX_SUFFIX
        )
    }
}
