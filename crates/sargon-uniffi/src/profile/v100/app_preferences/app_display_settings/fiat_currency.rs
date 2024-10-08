use crate::prelude::*;
use sargon::FiatCurrency as InternalFiatCurrency;

/// Fiat currency to measure and display the value of some XRD or other Radix assets value/worth in.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum FiatCurrency {
    /// American dollars.
    USD,

    /// Swedish krona.
    SEK,
}

impl From<InternalFiatCurrency> for FiatCurrency {
    fn from(value: InternalFiatCurrency) -> Self {
        match value {
            InternalFiatCurrency::USD => Self::USD,
            InternalFiatCurrency::SEK => Self::SEK,
        }
    }
}

impl Into<InternalFiatCurrency> for FiatCurrency {
    fn into(self) -> InternalFiatCurrency {
        match self {
            Self::USD => InternalFiatCurrency::USD,
            Self::SEK => InternalFiatCurrency::SEK,
        }
    }
}

json_string_convertible!(FiatCurrency, "super invalid json string");

#[uniffi::export]
pub fn new_fiat_currency_sample() -> FiatCurrency {
    InternalFiatCurrency::sample().into()
}

#[uniffi::export]
pub fn new_fiat_currency_sample_other() -> FiatCurrency {
    InternalFiatCurrency::sample_other().into()
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
