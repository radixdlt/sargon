use crate::prelude::*;

pub trait FromBIP32Str: Sized {
    fn from_bip32_string(s: impl AsRef<str>) -> Result<Self>;
}

impl<T: IsPathComponentStringConvertible + FromLocalKeySpace> FromBIP32Str
    for T
{
    /// Parse a BIP32 path string into a `T`.
    ///
    /// e.g:
    /// ```
    /// extern crate sargon;
    /// use sargon::prelude::*;
    ///
    /// assert!(AccountPath::from_bip32_string("m/44'/1022'/1'/525'/1460'/1'").is_ok());
    /// ```
    fn from_bip32_string(s: impl AsRef<str>) -> Result<T> {
        let s = s.as_ref();
        let suffix_min_len = std::cmp::min(
            T::CANONICAL_SUFFIX.len(),
            T::NON_CANONICAL_SUFFIX.len(),
        );
        let min_len = suffix_min_len + 1;
        let e = CommonError::InvalidBIP32Path {
            bad_value: s.to_string(),
        };
        if s.len() < min_len {
            return Err(e);
        }
        if suffix_min_len > 0 {
            let suffix = &s[s.len() - suffix_min_len..];
            if !T::ACCEPTABLE_SUFFIXES.contains(&suffix) {
                return Err(e);
            }
        }
        let value: u32 =
            s[..s.len() - suffix_min_len].parse().map_err(|_| e)?;
        T::from_local_key_space(value)
    }
}
