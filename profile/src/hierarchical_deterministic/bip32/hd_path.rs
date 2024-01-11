use crate::prelude::*;

use slip10::path::BIP32Path;

#[derive(Debug)]
pub struct InvalidValue<T: std::fmt::Debug> {
    pub expected: T,
    pub found: T,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, uniffi::Record)]
pub struct HDPath {
    pub components: Vec<HDPathComponent>,
}

impl std::fmt::Display for HDPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_bip32_string())
    }
}

impl Serialize for HDPath {
    /// Serializes this `HDPath` into its bech32 address string as JSON.
    #[cfg(not(tarpaulin_include))] // false negative
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for HDPath {
    /// Tries to deserializes a JSON string as a bech32 address into an `HDPath`.
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<HDPath, D::Error> {
        let s = String::deserialize(d)?;
        HDPath::from_str(&s).map_err(de::Error::custom)
    }
}

impl std::str::FromStr for HDPath {
    type Err = crate::CommonError;

    fn from_str(s: &str) -> Result<Self> {
        BIP32Path::from_str(s)
            .map(|p| Self::from(p))
            .map_err(|_| CommonError::InvalidBIP32Path(s.to_string()))
    }
}

impl HDPath {
    /// Upgrades a BIP32Path (extern crate, which is a bit limiting),
    /// to our type HDPath, which has a better API.
    pub(crate) fn from(path: BIP32Path) -> Self {
        let mut bip32 = path.clone();
        let mut vec: Vec<HDPathComponent> = Vec::new();
        for _ in 0..bip32.depth() {
            vec.push(HDPathComponent::from_value(bip32.pop().unwrap()))
        }
        assert!(vec.len() == path.depth() as usize);
        vec.reverse();
        return Self::from_components(vec);
    }

    pub(crate) fn from_components(components: Vec<HDPathComponent>) -> Self {
        Self { components }
    }

    pub(crate) fn depth(&self) -> usize {
        self.components.len()
    }

    pub(crate) fn parse_try_map<T, F>(
        path: &Vec<HDPathComponent>,
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
        path: &Vec<HDPathComponent>,
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

    pub(crate) fn try_parse_base_hdpath<F>(
        path: &HDPath,
        depth_error: F,
    ) -> Result<(HDPath, Vec<HDPathComponent>)>
    where
        F: FnOnce(InvalidValue<usize>) -> CommonError,
    {
        let expected_depth = 2;
        if path.depth() < expected_depth {
            return Err(depth_error(InvalidValue {
                expected: expected_depth,
                found: path.depth(),
            }));
        }
        let components = &path.components;

        _ = Self::parse(
            components,
            0,
            HDPathComponent::bip44_purpose(),
            Box::new(|v| CommonError::BIP44PurposeNotFound(v)),
        )?;

        _ = Self::parse(
            components,
            1,
            HDPathComponent::bip44_cointype(),
            Box::new(|v| CommonError::CoinTypeNotFound(v)),
        )?;
        return Ok((path.clone(), components.clone()));
    }

    pub(crate) fn try_parse_base<F>(
        s: &str,
        depth_error: F,
    ) -> Result<(HDPath, Vec<HDPathComponent>)>
    where
        F: FnOnce(InvalidValue<usize>) -> CommonError,
    {
        let path = HDPath::from_str(s).map_err(|_| CommonError::InvalidBIP32Path(s.to_string()))?;
        return Self::try_parse_base_hdpath(&path, depth_error);
    }
}

impl HDPath {
    fn to_bip32_string(&self) -> String {
        let rest = self
            .components
            .iter()
            .map(|c| c.clone().to_string())
            .join("/");
        return format!("m/{}", rest);
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;
    use serde_json::json;

    #[test]
    fn json_roundtrip() {
        let str = "m/44H/1022H";
        let parsed = HDPath::from_str(str).unwrap();
        assert_json_value_eq_after_roundtrip(&parsed, json!(str));
        assert_json_value_ne_after_roundtrip(&parsed, json!("m/44H/33H"));
        assert_json_value_fails::<HDPath>(json!("super invalid path"));
    }
}
