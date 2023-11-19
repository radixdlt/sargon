use std::str::FromStr;

use itertools::Itertools;
use serde::{de, Deserializer, Serialize, Serializer};
use slip10::path::BIP32Path;

use super::hd_path_component::HDPathComponent;

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
            match bip32.pop() {
                Some(c) => vec.push(HDPathComponent::from_value(c)),
                None => break,
            }
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
    /// Serializes this `AccountAddress` into its bech32 address string as JSON.
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for HDPath {
    /// Tries to deserializes a JSON string as a bech32 address into an `AccountAddress`.
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<HDPath, D::Error> {
        let s = String::deserialize(d)?;
        HDPath::from_str(&s).map_err(de::Error::custom)
    }
}
