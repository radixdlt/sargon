use crate::prelude::*;

json_string_convertible!(FiatCurrency);

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
