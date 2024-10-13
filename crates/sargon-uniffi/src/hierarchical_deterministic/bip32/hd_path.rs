use crate::prelude::*;
use sargon::HDPath as InternalHDPath;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct HDPath {
    pub components: Vec<HDPathComponent>,
}
