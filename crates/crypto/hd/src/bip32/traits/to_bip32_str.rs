use crate::prelude::*;

/// [CAP43][doc] uses `iS` instead of `{i+2^30}H` - for values in securified key space.
///
/// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3880058888/CAP-43+Sargon+HD+Path+string+notation
pub trait ToCAP43String: Sized {
    fn to_cap43_string(&self) -> String;
    fn to_cap43_string_debug(&self) -> String;
}

impl<T> ToCAP43String for T
where
    T: IsPathComponentStringConvertible + IsInLocalKeySpace,
{
    fn to_cap43_string(&self) -> String {
        format!(
            "{}{}",
            u32::from(self.index_in_local_key_space()),
            T::VERBOSE_SYNTAX_SUFFIX
        )
    }
    fn to_cap43_string_debug(&self) -> String {
        format!(
            "{}{}",
            u32::from(self.index_in_local_key_space()),
            T::SHORTHAND_SYNTAX_SUFFIX
        )
    }
}
