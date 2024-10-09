use crate::prelude::*;
use sargon::HDPath as InternalHDPath;

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
pub struct HDPath {
    pub components: Vec<HDPathComponent>,
}

impl From<InternalHDPath> for HDPath {
    fn from(value: InternalHDPath) -> Self {
        Self {
            components: value.components.into_iter().map(Into::into).collect(),
        }
    }
}

impl Into<InternalHDPath> for HDPath {
    fn into(self) -> InternalHDPath {
        InternalHDPath {
            components: self.components.into_iter().map(Into::into).collect(),
        }
    }
}