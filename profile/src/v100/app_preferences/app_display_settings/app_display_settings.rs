use crate::prelude::*;

/// Settings related to displaying of information to the user inside the app.
///
/// **N.B. neither of these settings are in fact not yet used by clients.**
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
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

impl HasPlaceholder for AppDisplay {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self {
            is_currency_amount_visible: true,
            fiat_currency_price_target: FiatCurrency::default(),
        }
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
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
        assert_eq!(AppDisplay::placeholder(), AppDisplay::placeholder());
        assert_eq!(
            AppDisplay::placeholder_other(),
            AppDisplay::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(AppDisplay::placeholder(), AppDisplay::placeholder_other());
    }

    #[test]
    fn fiat_worth_is_visible_by_default() {
        assert_eq!(AppDisplay::default().is_currency_amount_visible, true);
    }

    #[test]
    fn json_roundtrip() {
        let sut = AppDisplay::placeholder();
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
