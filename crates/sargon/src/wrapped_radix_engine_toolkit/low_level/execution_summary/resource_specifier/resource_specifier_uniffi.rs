use crate::prelude::*;

#[uniffi::export]
pub fn new_resource_specifier_sample() -> ResourceSpecifier {
    ResourceSpecifier::sample()
}

#[uniffi::export]
pub fn new_resource_specifier_sample_other() -> ResourceSpecifier {
    ResourceSpecifier::sample_other()
}

#[uniffi::export]
pub fn resource_specifier_get_address(
    specifier: &ResourceSpecifier,
) -> ResourceAddress {
    specifier.get_address()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceSpecifier;

    #[test]
    fn inequality() {
        assert_ne!(
            new_resource_indicator_sample(),
            new_resource_indicator_sample_other()
        );
    }

    #[test]
    fn get_address() {
        let sut = SUT::sample();
        assert_eq!(sut.get_address(), resource_specifier_get_address(&sut));
    }
}
