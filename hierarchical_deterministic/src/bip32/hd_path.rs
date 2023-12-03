use std::str::FromStr;

use itertools::Itertools;
use serde::{de, Deserializer, Serialize, Serializer};
use slip10::path::BIP32Path;
use wallet_kit_common::error::hdpath_error::HDPathError;

use super::hd_path_component::{HDPathComponent, HDPathValue};

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum BIP32Error {
    #[error("Invalid BIP32 path '{0}'.")]
    InvalidBIP32Path(String),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HDPath(Vec<HDPathComponent>);

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
        return Self(vec);
    }

    pub(crate) fn from_components(components: Vec<HDPathComponent>) -> Self {
        Self(components)
    }

    pub(crate) fn components(&self) -> &Vec<HDPathComponent> {
        &self.0
    }

    pub(crate) fn depth(&self) -> usize {
        self.0.len()
    }

    pub fn from_str(s: &str) -> Result<Self, BIP32Error> {
        BIP32Path::from_str(s)
            .map(|p| Self::from(p))
            .map_err(|_| BIP32Error::InvalidBIP32Path(s.to_string()))
    }

    pub(crate) fn parse_try_map<T, F>(
        path: &Vec<HDPathComponent>,
        index: usize,
        try_map: F,
    ) -> Result<T, HDPathError>
    where
        F: Fn(HDPathValue) -> Result<T, HDPathError>,
    {
        let got = &path[index];
        try_map(got.index())
    }

    pub(crate) fn parse<F>(
        path: &Vec<HDPathComponent>,
        index: usize,
        expected: HDPathComponent,
        err: F,
    ) -> Result<&HDPathComponent, HDPathError>
    where
        F: Fn(HDPathValue) -> HDPathError,
    {
        let got = &path[index];
        if got != &expected {
            return Err(err(got.index()));
        }
        Ok(got)
    }

    pub(crate) fn try_parse_base_hdpath(
        path: &HDPath,
        depth_error: HDPathError,
    ) -> Result<(HDPath, Vec<HDPathComponent>), HDPathError> {
        use HDPathError::*;
        if path.depth() < 2 {
            return Err(depth_error);
        }
        let components = path.components();

        _ = Self::parse(
            components,
            0,
            HDPathComponent::bip44_purpose(),
            Box::new(|v| BIP44PurposeNotFound(v)),
        )?;

        _ = Self::parse(
            components,
            1,
            HDPathComponent::bip44_cointype(),
            Box::new(|v| CoinTypeNotFound(v)),
        )?;
        return Ok((path.clone(), components.clone()));
    }

    pub(crate) fn try_parse_base(
        s: &str,
        depth_error: HDPathError,
    ) -> Result<(HDPath, Vec<HDPathComponent>), HDPathError> {
        let path = HDPath::from_str(s).map_err(|_| HDPathError::InvalidBIP32Path(s.to_string()))?;
        return Self::try_parse_base_hdpath(&path, depth_error);
    }
}

impl ToString for HDPath {
    fn to_string(&self) -> String {
        let rest = self
            .components()
            .into_iter()
            .map(|c| c.to_string())
            .join("/");
        return format!("m/{}", rest);
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

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_common::json::{
        assert_json_value_eq_after_roundtrip, assert_json_value_fails,
        assert_json_value_ne_after_roundtrip,
    };

    use super::HDPath;

    #[test]
    fn json_roundtrip() {
        let str = "m/44H/1022H";
        let parsed = HDPath::from_str(str).unwrap();
        assert_json_value_eq_after_roundtrip(&parsed, json!(str));
        assert_json_value_ne_after_roundtrip(&parsed, json!("m/44H/33H"));
        assert_json_value_fails::<HDPath>(json!("super invalid path"));
    }
}
