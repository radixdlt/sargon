use crate::prelude::*;

#[uniffi::export]
pub fn new_fungible_resource_indicator_sample() -> FungibleResourceIndicator {
    FungibleResourceIndicator::sample()
}

#[uniffi::export]
pub fn new_fungible_resource_indicator_sample_other(
) -> FungibleResourceIndicator {
    FungibleResourceIndicator::sample_other()
}

#[uniffi::export]
pub fn fungible_resource_indicator_get_amount(
    indicator: &FungibleResourceIndicator,
) -> Decimal192 {
    indicator.get_amount()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FungibleResourceIndicator;

    #[test]
    fn inequality() {
        assert_ne!(
            new_fungible_resource_indicator_sample(),
            new_fungible_resource_indicator_sample_other()
        );
    }

    #[test]
    fn get_amount() {
        let sut = SUT::sample();
        assert_eq!(
            sut.get_amount(),
            fungible_resource_indicator_get_amount(&sut)
        );
    }
}
