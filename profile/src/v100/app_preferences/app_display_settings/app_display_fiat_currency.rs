use serde::{Deserialize, Serialize};

/// Fiat currency to measure and display the value of some XRD or other Radix assets value/worth in.
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, uniffi::Enum,
)]
pub enum FiatCurrency {
    /// American dollars.
    #[serde(rename = "usd")]
    USD,
}

impl Default for FiatCurrency {
    /// American dollars.
    fn default() -> Self {
        Self::USD
    }
}

#[cfg(test)]
mod tests {
    use crate::AppDisplay;

    use super::FiatCurrency;

    #[test]
    fn usd_is_default() {
        assert_eq!(
            AppDisplay::default().fiat_currency_price_target,
            FiatCurrency::USD
        );
    }
}
