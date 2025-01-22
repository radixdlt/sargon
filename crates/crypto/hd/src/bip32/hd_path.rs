use itertools::Itertools;

use crate::prelude::*;

#[allow(unused)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Debug,
    DeserializeFromStr,
    SerializeDisplay,
)]
#[display("{}", self.to_bip32_string())]
#[debug("{}", self.to_bip32_string_debug())]
pub struct HDPath {
    pub components: Vec<HDPathComponent>,
}
impl HDPath {
    pub const fn new(components: Vec<HDPathComponent>) -> Self {
        Self { components }
    }
    pub fn components(&self) -> &[HDPathComponent] {
        &self.components
    }
}

impl FromBIP32Str for HDPath {
    fn from_bip32_string(s: impl AsRef<str>) -> Result<Self> {
        let s = s.as_ref();
        let mut s = s;
        if s.starts_with(&format!("m{}", Self::SEPARATOR)) {
            s = &s[2..]
        }
        if s.starts_with(&format!("M{}", Self::SEPARATOR)) {
            s = &s[2..]
        }
        if s.starts_with(Self::SEPARATOR) {
            s = &s[1..]
        }
        let components = s
            .split(Self::SEPARATOR)
            .filter(|s| !s.is_empty())
            .map(HDPathComponent::from_bip32_string)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self::new(components))
    }
}

impl HDPath {
    pub const SEPARATOR: &str = "/";
    fn to_string_map_with<F>(&self, include_head: bool, map: F) -> String
    where
        F: Fn((usize, &HDPathComponent)) -> String,
    {
        let head = "m".to_owned();
        let mut path =
            self.components().iter().enumerate().map(map).collect_vec();
        if include_head {
            path.splice(0..0, vec![head]);
        }
        path.into_iter().join(Self::SEPARATOR)
    }

    pub fn to_bip32_string_with(
        &self,
        include_head: bool,
        canonicalize_entity_index: bool,
    ) -> String {
        self.to_string_map_with(include_head, |(i, c)| {
            if canonicalize_entity_index
                && i == CAP26_PATH_ENTITY_INDEX_POS
                && c.is_securified()
            {
                let securified = c.as_securified().unwrap();
                let global = u32::from(securified.index_in_local_key_space());
                let without_securified =
                    global + RELATIVELY_LOCAL_OFFSET_SECURIFIED;
                format!("{}H", without_securified)
            } else {
                format!("{}", c)
            }
        })
    }

    pub fn to_bip32_string_debug_with(&self, include_head: bool) -> String {
        self.to_string_map_with(include_head, |(_, c)| format!("{:?}", c))
    }

    /// This method returns the canonical bip32 representation of the path.
    /// In sargon, paths in the securified space are printed with the `S` notation after the index,
    /// for readability purposes. Such paths need to be canonicalized in bip32 notation meaning that
    /// an index of `i S` => `i + 2^31 + 2^30 H` for communication with other external APIs.
    pub fn to_canonical_bip32_string(&self) -> String {
        self.to_bip32_string_with(true, true)
    }
}
impl ToBIP32Str for HDPath {
    fn to_bip32_string(&self) -> String {
        self.to_bip32_string_with(true, false)
    }
    fn to_bip32_string_debug(&self) -> String {
        self.to_bip32_string_debug_with(true)
    }
}

impl FromStr for HDPath {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_bip32_string(s)
    }
}

impl HasSampleValues for HDPath {
    fn sample() -> Self {
        Self::from_str("m/44H/1022H/1H/525H/1460H/1H").unwrap()
    }

    fn sample_other() -> Self {
        Self::from_str("m/44H/1022H/0H/0/0H").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HDPath;

    #[test]
    fn account_path() {
        let hdpath = SUT::from_str("m/44H/1022H/1H/525H/1460H/0H").unwrap();
        assert_eq!(hdpath, AccountPath::sample().to_hd_path());
    }

    impl HDPath {
        fn harden<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = u32>,
        {
            HDPath::new(
                iter.into_iter()
                    .map(|i| HDPathComponent::unsecurified_hardened(i).unwrap())
                    .collect_vec(),
            )
        }
    }

    #[test]
    fn equality() {
        assert_eq!(HDPath::sample(), HDPath::sample());
        assert_eq!(HDPath::sample_other(), HDPath::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(HDPath::sample(), HDPath::sample_other());
    }

    #[test]
    fn display_two() {
        let path = HDPath::harden([44, 1022]);
        assert_eq!(format!("{}", path), "m/44H/1022H");
    }

    #[test]
    fn debug() {
        let path = HDPath::harden([44, 1022]);
        assert_eq!(format!("{:?}", path), "m/44'/1022'");
    }

    #[test]
    fn from_str() {
        assert_eq!(
            HDPath::from_str("m/44H/1022H").unwrap(),
            HDPath::harden([44, 1022])
        );
    }

    #[test]
    fn from_str_capital_m_is_ok() {
        assert_eq!(
            HDPath::from_str("M/44H/1022H").unwrap(),
            HDPath::harden([44, 1022])
        );
    }

    #[test]
    fn from_str_invalid_prefix() {
        let s = "x/44H/1022H";
        assert_eq!(
            HDPath::from_str(s),
            Err(CommonError::InvalidBIP32Path {
                bad_value: "x".to_owned()
            })
        );
    }

    #[test]
    fn from_str_invalid_separator() {
        let s = "m#44H#1022H";
        assert_eq!(
            HDPath::from_str(s),
            Err(CommonError::InvalidBIP32Path {
                bad_value: s.to_owned()
            })
        );
    }

    #[test]
    fn ord() {
        assert!(HDPath::harden([44, 2]) > HDPath::harden([44, 1]));
    }

    #[test]
    fn json_roundtrip_success() {
        let sut = HDPath::harden([44, 1022]);
        assert_json_value_eq_after_roundtrip(&sut, json!("m/44H/1022H"));
        assert_json_value_ne_after_roundtrip(&sut, json!("m/44H/33H"));
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<HDPath>(json!("x/44H"));
        assert_json_value_fails::<HDPath>(json!("m/44X"));
        assert_json_value_fails::<HDPath>(json!("super invalid path"));
    }
}
