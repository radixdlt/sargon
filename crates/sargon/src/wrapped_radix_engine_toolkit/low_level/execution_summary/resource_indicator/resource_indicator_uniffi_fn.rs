use crate::prelude::*;

#[uniffi::export]
pub fn new_resource_indicator_sample() -> ResourceIndicator {
    ResourceIndicator::sample()
}

#[uniffi::export]
pub fn new_resource_indicator_sample_other() -> ResourceIndicator {
    ResourceIndicator::sample_other()
}

#[uniffi::export]
pub fn resource_indicator_get_address(
    indicator: &ResourceIndicator,
) -> ResourceAddress {
    indicator.get_address()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceIndicator;

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
        assert_eq!(sut.get_address(), resource_indicator_get_address(&sut));
    }
}
