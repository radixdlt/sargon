use crate::prelude::*;

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
