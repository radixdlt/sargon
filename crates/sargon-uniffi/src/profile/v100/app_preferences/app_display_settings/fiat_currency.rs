use crate::prelude::*;
use sargon::FiatCurrency as InternalFiatCurrency;

/// Fiat currency to measure and display the value of some XRD or other Radix assets value/worth in.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum FiatCurrency {
    /// American dollars.
    USD,

    /// Swedish krona.
    SEK,
}

json_string_convertible!(FiatCurrency);

#[uniffi::export]
pub fn new_fiat_currency_sample() -> FiatCurrency {
    InternalFiatCurrency::sample().into()
}

#[uniffi::export]
pub fn new_fiat_currency_sample_other() -> FiatCurrency {
    InternalFiatCurrency::sample_other().into()
}
