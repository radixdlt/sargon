use crate::prelude::*;
use sargon::HDPathComponent as InternalHDPathComponent;

pub type HDPathValue = u32;

#[derive(
    Clone,
    
    PartialEq,
    Eq,
    Hash,
    
    
     uniffi::Record,
)]
pub struct HDPathComponent {
    pub value: HDPathValue,
}

impl From<InternalHDPathComponent> for HDPathComponent {
    fn from(value: InternalHDPathComponent) -> Self {
        Self { value: value.value }
    }
}

impl Into<InternalHDPathComponent> for HDPathComponent {
    fn into(self) -> InternalHDPathComponent {
        InternalHDPathComponent {
            value: self.value,
        }
    }
}

#[uniffi::export]
pub fn hd_path_component_get_non_hardened_value(
    component: HDPathComponent,
) -> HDPathValue {
    component.into::<InternalHDPathComponent>().index()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HDPathComponent;

    #[test]
    fn test_hd_path_component_get_non_hardened_value() {
        let sut = SUT::harden(5);
        assert_eq!(hd_path_component_get_non_hardened_value(sut), 5);
    }
}
