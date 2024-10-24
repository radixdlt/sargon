use crate::prelude::*;
use sargon::HDPathComponent as InternalHDPathComponent;

pub type HDPathValue = u32;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct HDPathComponent {
    pub value: HDPathValue,
}

#[uniffi::export]
pub fn hd_path_component_get_non_hardened_value(
    component: HDPathComponent,
) -> HDPathValue {
    component.into_internal().index()
}
