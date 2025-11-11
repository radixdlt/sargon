use crate::prelude::*;

/// Fiat currency to measure and display the value of some XRD or other Radix assets value/worth in.
#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum FiatCurrency {
    /// American dollars.
    USD,

    /// Swedish krona.
    SEK,
}

impl Default for FiatCurrency {
    /// American dollars.
    fn default() -> Self {
        Self::USD
    }
}

impl<'de> serde::Deserialize<'de> for FiatCurrency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;

        match value.as_str() {
            v if v.eq_ignore_ascii_case("usd") => Ok(Self::USD),
            v if v.eq_ignore_ascii_case("sek") => Ok(Self::SEK),
            _ => Err(serde::de::Error::unknown_variant(&value, &["usd", "sek"])),
        }
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

    #[test]
    fn from_uppercase_json_str() {
        assert_eq!("USD".deserialize::<FiatCurrency>().unwrap(), SUT::USD);
        assert_eq!("SEK".deserialize::<FiatCurrency>().unwrap(), SUT::SEK);
    }
}
