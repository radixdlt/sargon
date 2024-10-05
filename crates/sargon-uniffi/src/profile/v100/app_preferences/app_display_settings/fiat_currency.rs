use crate::prelude::*;

/// Fiat currency to measure and display the value of some XRD or other Radix assets value/worth in.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    uniffi::Enum,
)]
pub enum FiatCurrency {
    /// American dollars.
    USD,

    /// Swedish krona.
    SEK,
}

json_string_convertible!(FiatCurrency, "super invalid json string");

#[uniffi::export]
pub fn new_fiat_currency_sample() -> FiatCurrency {
    FiatCurrency::sample()
}

#[uniffi::export]
pub fn new_fiat_currency_sample_other() -> FiatCurrency {
    FiatCurrency::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FiatCurrency;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_fiat_currency_sample(),
                new_fiat_currency_sample_other(),
                // duplicates should get removed
                new_fiat_currency_sample(),
                new_fiat_currency_sample_other(),
            ])
            .len(),
            2
        );
    }
}
