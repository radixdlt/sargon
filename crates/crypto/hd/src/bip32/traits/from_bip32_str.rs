use crate::prelude::*;

pub trait ValueInLocalKeyspaceFromBIP32Str: Sized {
    fn value_in_local_keyspace_from_cap43_string(
        s: impl AsRef<str>,
    ) -> Result<u32>;

    fn value_in_local_keyspace_from_cap43_string_with_acceptable_suffixes(
        s: impl AsRef<str>,
        acceptable_suffixes: Vec<&str>,
    ) -> Result<u32> {
        assert!(!acceptable_suffixes.is_empty());

        let s = s.as_ref();

        let suffix_min_len = acceptable_suffixes
            .iter()
            .map(|s| s.len())
            .min()
            .expect("at least one suffix");

        let min_len = suffix_min_len + 1;
        let e = CommonError::InvalidBIP32Path {
            bad_value: s.to_string(),
        };
        if s.len() < min_len {
            return Err(e);
        }
        if suffix_min_len > 0 {
            let suffix = &s[s.len() - suffix_min_len..];
            if !acceptable_suffixes.contains(&suffix) {
                return Err(e);
            }
        }
        s[..s.len() - suffix_min_len].parse().map_err(|_| e)
    }
}

/// CAP43 string [described here][doc]
///
/// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3880058888/CAP-43+Sargon+HD+Path+string+notation
pub trait FromCAP43String: Sized {
    fn from_cap43_string(s: impl AsRef<str>) -> Result<Self>;
}

impl<T: ValueInLocalKeyspaceFromBIP32Str + FromLocalKeySpace> FromCAP43String
    for T
{
    /// Parse a BIP32 path string into a `Self`.
    ///
    /// e.g:
    /// ```
    /// use hierarchical_deterministic::prelude::*;
    ///
    /// assert!(AccountPath::from_cap43_string("m/44'/1022'/1'/525'/1460'/1'").is_ok());
    /// ```
    fn from_cap43_string(s: impl AsRef<str>) -> Result<Self> {
        let value = Self::value_in_local_keyspace_from_cap43_string(s)?;
        Self::from_local_key_space(value)
    }
}

impl<T: IsPathComponentStringConvertible> ValueInLocalKeyspaceFromBIP32Str
    for T
{
    /// Parse a BIP32 path string into a `u32` raw value without any offsets.
    ///
    /// e.g:
    /// ```
    /// use hierarchical_deterministic::prelude::*;
    ///
    /// assert!(AccountPath::from_cap43_string("m/44'/1022'/1'/525'/1460'/1'").is_ok());
    /// ```
    fn value_in_local_keyspace_from_cap43_string(
        s: impl AsRef<str>,
    ) -> Result<u32> {
        Self::value_in_local_keyspace_from_cap43_string_with_acceptable_suffixes(
            s,
            T::ACCEPTABLE_SUFFIXES.to_vec(),
        )
    }
}
