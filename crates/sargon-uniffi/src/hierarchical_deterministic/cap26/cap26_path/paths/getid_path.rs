use crate::prelude::*;
use sargon::HDPath as InternalHDPath;

/// Use it with `GetIDPath::default()` to create the path `m/44'/1022'/365'`
/// which is used by all hierarchal deterministic factor sources to derive
/// the special root key which we hash to form the `FactorSourceIDFromHash`
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Record,
)]
pub struct GetIDPath {
    pub path: HDPath,
}

impl From<InternalHDPath> for GetIDPath {
    fn from(value: InternalHDPath) -> Self {
        Self {
            path: value.into(),
        }
    }
}

impl Into<InternalHDPath> for GetIDPath {
    fn into(self) -> InternalHDPath {
        self.path.into()
    }
}