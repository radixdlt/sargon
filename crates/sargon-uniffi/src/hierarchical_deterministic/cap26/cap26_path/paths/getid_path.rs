use crate::prelude::*;
use sargon::GetIDPath as InternalGetIDPath;

/// Use it with `GetIDPath::default()` to create the path `m/44'/1022'/365'`
/// which is used by all hierarchal deterministic factor sources to derive
/// the special root key which we hash to form the `FactorSourceIDFromHash`
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct GetIDPath {
    pub path: HDPath,
}

impl From<InternalGetIDPath> for GetIDPath {
    fn from(value: InternalGetIDPath) -> Self {
        Self {
            path: value.path.into(),
        }
    }
}

impl Into<InternalGetIDPath> for GetIDPath {
    fn into(self) -> InternalGetIDPath {
        InternalGetIDPath {
            path: self.path.into(),
        }
    }
}

#[uniffi::export]
pub fn default_get_id_path() -> GetIDPath {
    InternalGetIDPath::default().into()
}
