use crate::prelude::*;

#[uniffi::export]
pub fn new_factor_source_crypto_parameters_sample(
) -> FactorSourceCryptoParameters {
    FactorSourceCryptoParameters::sample()
}

#[uniffi::export]
pub fn new_factor_source_crypto_parameters_sample_other(
) -> FactorSourceCryptoParameters {
    FactorSourceCryptoParameters::sample_other()
}

#[uniffi::export]
pub fn new_factor_source_crypto_parameters_preset_babylon_only(
) -> FactorSourceCryptoParameters {
    FactorSourceCryptoParameters::babylon()
}

#[uniffi::export]
pub fn new_factor_source_crypto_parameters_preset_olympia_only(
) -> FactorSourceCryptoParameters {
    FactorSourceCryptoParameters::olympia()
}

#[uniffi::export]
pub fn new_factor_source_crypto_parameters_preset_babylon_olympia_compatible(
) -> FactorSourceCryptoParameters {
    FactorSourceCryptoParameters::babylon_olympia_compatible()
}

#[uniffi::export]
pub fn factor_source_crypto_parameters_supports_olympia(
    parameters: &FactorSourceCryptoParameters,
) -> bool {
    parameters.supports_olympia()
}

#[uniffi::export]
pub fn factor_source_crypto_parameters_supports_babylon(
    parameters: &FactorSourceCryptoParameters,
) -> bool {
    parameters.supports_babylon()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceCryptoParameters;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_factor_source_crypto_parameters_sample(),
                new_factor_source_crypto_parameters_sample_other(),
                new_factor_source_crypto_parameters_preset_babylon_olympia_compatible(),
                // duplicates should get removed
                new_factor_source_crypto_parameters_preset_babylon_only(), // same as sample
                new_factor_source_crypto_parameters_preset_olympia_only(), // same as sample_other
            ])
            .len(),
            3
        );
    }

    #[test]
    fn test_supports_babylon() {
        assert!(factor_source_crypto_parameters_supports_babylon(
            &new_factor_source_crypto_parameters_preset_babylon_only()
        ))
    }

    #[test]
    fn test_supports_olympia() {
        assert!(factor_source_crypto_parameters_supports_olympia(
            &new_factor_source_crypto_parameters_preset_olympia_only()
        ))
    }
}
