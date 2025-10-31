use crate::prelude::*;

/// Fiat currency to measure and display the value of some XRD or other Radix assets value/worth in.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum FiatCurrency {
    /// American dollars.
    #[serde(rename = "USD")]
    USD,

    /// Swedish krona.
    #[serde(rename = "SEK")]
    SEK,
}

impl Default for FiatCurrency {
    /// American dollars.
    fn default() -> Self {
        Self::USD
    }
}

impl HasSampleValues for FiatCurrency {
    fn sample() -> Self {
        Self::USD
    }

    fn sample_other() -> Self {
        Self::SEK
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FiatCurrency;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn usd_is_default() {
        assert_eq!(AppDisplay::default().fiat_currency_price_target, SUT::USD);
    }

    #[test]
    fn from_json_str() {
        assert_eq!("usd".deserialize::<FiatCurrency>().unwrap(), SUT::USD);
    }
}
