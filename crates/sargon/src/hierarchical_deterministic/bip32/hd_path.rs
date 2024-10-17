use crate::prelude::*;

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    SerializeDisplay,
    DeserializeFromStr,
    derive_more::Display,
    derive_more::Debug,
)]
#[display("{}", self.bip32_string())]
#[debug("{}", self.bip32_string())]
pub struct HDPath {
    pub components: Vec<HDPathComponent>,
}

fn valid_bip32_char(c: char) -> bool {
    "mM/H'0123456789".contains(c)
}

impl FromStr for HDPath {
    type Err = crate::CommonError;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        let err = CommonError::InvalidBIP32Path {
            bad_value: path.to_owned(),
        };
        if !path.chars().all(valid_bip32_char) {
            return Err(err);
        }
        let mut path = path;
        if path.starts_with("m/") {
            path = &path[2..];
        }
        if path.starts_with("M/") {
            path = &path[2..];
        }

        let expected_component_count = path.matches('/').count() + 1;
        let replaced = path.replace('/', "\n");

        let components = replaced
            .lines()
            .filter_map(HDPathComponent::try_from_str)
            .collect_vec();
        if components.len() != expected_component_count {
            Err(err)
        } else {
            Ok(Self::from_components(components))
        }
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

impl HDPath {
    pub(crate) fn from_components<I>(components: I) -> Self
    where
        I: IntoIterator<Item = HDPathComponent>,
    {
        Self {
            components: components.into_iter().collect_vec(),
        }
    }

    pub(crate) fn depth(&self) -> usize {
        self.components.len()
    }

    pub(crate) fn parse_try_map<T, F>(
        path: &[HDPathComponent],
        index: usize,
        try_map: F,
    ) -> Result<T>
    where
        F: Fn(HDPathValue) -> Result<T>,
    {
        let got = &path[index];
        try_map(got.index())
    }

    pub(crate) fn parse<F>(
        path: &[HDPathComponent],
        index: usize,
        expected: HDPathComponent,
        err: F,
    ) -> Result<&HDPathComponent>
    where
        F: Fn(HDPathValue) -> CommonError,
    {
        let got = &path[index];
        if got != &expected {
            return Err(err(got.index()));
        }
        Ok(got)
    }

    #[cfg(not(tarpaulin_include))] // false negative
    pub(crate) fn try_parse_base_hdpath<F>(
        path: &HDPath,
        depth_error: F,
    ) -> Result<(HDPath, Vec<HDPathComponent>)>
    where
        F: FnOnce(usize) -> CommonError,
    {
        let expected_depth = 2;
        if path.depth() < expected_depth {
            return Err(depth_error(path.depth()));
        }
        let components = &path.components;

        _ = Self::parse(
            components,
            0,
            HDPathComponent::bip44_purpose(),
            |v| CommonError::BIP44PurposeNotFound { bad_value: v },
        )?;

        _ = Self::parse(
            components,
            1,
            HDPathComponent::bip44_cointype(),
            |v| CommonError::CoinTypeNotFound { bad_value: v },
        )?;
        Ok((path.clone(), components.clone()))
    }

    pub(crate) fn try_parse_base<F>(
        s: &str,
        depth_error: F,
    ) -> Result<(HDPath, Vec<HDPathComponent>)>
    where
        F: FnOnce(usize) -> CommonError,
    {
        HDPath::from_str(s)
            .map_err(|_| CommonError::InvalidBIP32Path {
                bad_value: s.to_string(),
            })
            .and_then(|p| Self::try_parse_base_hdpath(&p, depth_error))
    }
}

impl HDPath {
    fn bip32_string(&self) -> String {
        let rest = self.components.iter().map(|c| c.to_string()).join("/");
        format!("m/{}", rest)
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    impl HDPath {
        fn harden<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = HDPathValue>,
        {
            HDPath {
                components: iter
                    .into_iter()
                    .map(HDPathComponent::harden)
                    .collect_vec(),
            }
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
    fn display() {
        let path = HDPath::harden([44, 1022]);
        assert_eq!(format!("{}", path), "m/44H/1022H");
    }

    #[test]
    fn debug() {
        let path = HDPath::harden([44, 1022]);
        assert_eq!(format!("{:?}", path), "m/44H/1022H");
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
                bad_value: s.to_owned()
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
