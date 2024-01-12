use crate::prelude::*;

/// Use it with `GetIDPath::default()` to create the path `m/44'/1022'/365'`
/// which is used by all hierarchal deterministic factor sources to derive
/// the special root key which we hash to form the `FactorSourceIDFromHash`
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, uniffi::Record)]
pub struct GetIDPath {
    pub path: HDPath,
}

impl Derivation for GetIDPath {
    fn derivation_path(&self) -> DerivationPath {
        DerivationPath::CAP26 {
            value: self.clone().into(),
        }
    }
    fn hd_path(&self) -> &HDPath {
        &self.path
    }
    fn scheme(&self) -> DerivationPathScheme {
        DerivationPathScheme::Cap26
    }
}

impl Default for GetIDPath {
    fn default() -> Self {
        Self {
            path: HDPath::from_str("m/44H/1022H/365H").expect("Valid path"),
        }
    }
}

impl TryFrom<&HDPath> for GetIDPath {
    type Error = CommonError;

    fn try_from(value: &HDPath) -> Result<Self> {
        let expected_depth = 3;
        let (path, components) = HDPath::try_parse_base_hdpath(value, |v| {
            CommonError::InvalidDepthOfCAP26Path {
                expected: Self::PATH_DEPTH,
                found: v.found,
            }
        })?;
        if path.depth() != expected_depth {
            return Err(CommonError::InvalidDepthOfCAP26Path {
                expected: expected_depth,
                found: path.depth(),
            });
        }
        let value = HDPath::parse_try_map(&components, 2, Box::new(|v| Ok(v)))?;
        if value != Self::LAST_COMPONENT_VALUE {
            return Err(CommonError::InvalidGetIDPath(value));
        }
        let hd_path = HDPath::from_components(components);
        assert_eq!(Self { path: hd_path }, Self::default());
        return Ok(Self::default());
    }
}

impl GetIDPath {
    pub const PATH_DEPTH: usize = 3;
    pub const LAST_COMPONENT_VALUE: HDPathValue = 365;

    pub fn from_str(s: &str) -> Result<Self> {
        let (path, _) = HDPath::try_parse_base(s, |v| CommonError::InvalidDepthOfCAP26Path {
            expected: Self::PATH_DEPTH,
            found: v.found,
        })?;
        return Self::try_from(&path);
    }
}

impl Serialize for GetIDPath {
    /// Serializes this `GetIDPath` into JSON as a derivation path string on format `m/1022H/365H`
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for GetIDPath {
    /// Tries to deserializes a JSON string as derivation path string into a `GetIDPath`
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<GetIDPath, D::Error> {
        let s = String::deserialize(d)?;
        GetIDPath::from_str(&s).map_err(de::Error::custom)
    }
}

impl TryInto<GetIDPath> for &str {
    type Error = CommonError;

    fn try_into(self) -> Result<GetIDPath, Self::Error> {
        GetIDPath::from_str(self)
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn to_string() {
        assert_eq!(GetIDPath::default().to_string(), "m/44H/1022H/365H");
    }

    #[test]
    fn from_str() {
        assert_eq!(
            GetIDPath::default(),
            GetIDPath::from_str("m/44H/1022H/365H").unwrap()
        );
    }
    #[test]
    fn invalid_value() {
        assert_eq!(
            GetIDPath::from_str("m/44H/1022H/1337H"),
            Err(CommonError::InvalidGetIDPath(1337))
        );
    }
    #[test]
    fn invalid_depth() {
        assert_eq!(
            GetIDPath::from_str("m/44H/1022H"),
            Err(CommonError::InvalidDepthOfCAP26Path {
                expected: 6,
                found: 2
            })
        );
    }

    #[test]
    fn json_roundtrip() {
        let str = "m/44H/1022H/365H";
        let parsed: GetIDPath = str.try_into().unwrap();
        assert_json_value_eq_after_roundtrip(&parsed, json!(str));
    }

    #[test]
    fn json_roundtrip_invalid() {
        assert_json_value_fails::<GetIDPath>(json!("m/44H/1022H/99H"));
    }
}
