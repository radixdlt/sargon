use crate::prelude::*;

#[uniffi::export]
pub fn factor_source_id_from_address_to_string(
    factor_source_id: &FactorSourceIDFromAddress,
) -> String {
    factor_source_id.to_string()
}

#[uniffi::export]
pub fn new_factor_source_id_from_address_sample() -> FactorSourceIDFromAddress {
    FactorSourceIDFromAddress::sample()
}

#[uniffi::export]
pub fn new_factor_source_id_from_address_sample_other(
) -> FactorSourceIDFromAddress {
    FactorSourceIDFromAddress::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceIDFromAddress;

    #[test]
    fn test_factor_source_id_from_address_to_string() {
        assert_eq!(
            factor_source_id_from_address_to_string(&SUT::sample()),
            SUT::sample().to_string()
        );
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_factor_source_id_from_address_sample(),
                new_factor_source_id_from_address_sample_other(),
                // duplicates should get removed
                new_factor_source_id_from_address_sample(),
                new_factor_source_id_from_address_sample_other(),
            ])
            .len(),
            2
        );
    }
}
