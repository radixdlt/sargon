use crate::prelude::*;

/// Settings related to displaying of information to the user inside the app.
///
/// **N.B. neither of these settings are in fact not yet used by clients.**
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
)]
#[serde(rename_all = "camelCase")]
#[display("is_currency_amount_visible: {is_currency_amount_visible}")]
pub struct AppDisplay {
    /// If we should show the aggregate value of users portfolio in fiat currency
    /// of hide it.
    pub is_currency_amount_visible: bool,

    /// Which fiat currency the prices are measured in.
    pub fiat_currency_price_target: FiatCurrency,
}

impl Default for AppDisplay {
    fn default() -> Self {
        Self {
            is_currency_amount_visible: true,
            fiat_currency_price_target: FiatCurrency::default(),
        }
    }
}

impl HasSampleValues for AppDisplay {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self {
            is_currency_amount_visible: true,
            fiat_currency_price_target: FiatCurrency::default(),
        }
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self {
            is_currency_amount_visible: false,
            fiat_currency_price_target: FiatCurrency::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn equality() {
        assert_eq!(AppDisplay::sample(), AppDisplay::sample());
        assert_eq!(AppDisplay::sample_other(), AppDisplay::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(AppDisplay::sample(), AppDisplay::sample_other());
    }

    #[test]
    fn fiat_worth_is_visible_by_default() {
        assert!(AppDisplay::default().is_currency_amount_visible);
    }

    #[test]
    fn json_roundtrip() {
        let sut = AppDisplay::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "fiatCurrencyPriceTarget": "usd",
                "isCurrencyAmountVisible": true
            }
            "#,
        )
    }
}
