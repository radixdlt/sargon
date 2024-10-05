use crate::prelude::*;

pub(crate) const BIP32_HARDENED: u32 = 2147483648;

pub type HDPathValue = u32;

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[display("{}", self.bip32_string())]
#[debug("{}", self.bip32_string())]
pub struct HDPathComponent {
    pub value: HDPathValue,
}

#[uniffi::export]
pub fn hd_path_component_get_non_hardened_value(
    component: HDPathComponent,
) -> HDPathValue {
    component.index()
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
