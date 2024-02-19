use crate::prelude::*;

/// User Preferences relating to submission of transactions.
#[derive(
    Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
pub struct TransactionPreferences {
    /// The deposit guarantee that will automatically be added for
    /// all deposits in transactions.
    pub default_deposit_guarantee: Decimal192,
}

impl TransactionPreferences {
    /// Instantiates a new Transaction user preference with the
    /// specified `default_deposit_guarantee` value.
    pub fn new(default_deposit_guarantee: Decimal192) -> Self {
        Self {
            default_deposit_guarantee,
        }
    }
}

impl Default for TransactionPreferences {
    /// By default `1.0` is used.
    fn default() -> Self {
        Self {
            default_deposit_guarantee: Decimal192::one(),
        }
    }
}

impl HasPlaceholder for TransactionPreferences {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::new("0.975".parse().unwrap())
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::new("0.765".parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn equality() {
        assert_eq!(
            TransactionPreferences::placeholder(),
            TransactionPreferences::placeholder()
        );
        assert_eq!(
            TransactionPreferences::placeholder_other(),
            TransactionPreferences::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            TransactionPreferences::placeholder(),
            TransactionPreferences::placeholder_other()
        );
    }

    #[test]
    fn get_decimal() {
        let value = Decimal::new("0.975".to_string()).unwrap();
        let sut = TransactionPreferences::new(value);
        assert_eq!(sut.default_deposit_guarantee, value)
    }

    #[test]
    fn json_roundtrip() {
        let sut = TransactionPreferences::placeholder();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "defaultDepositGuarantee": "0.975"
            }
            "#,
        )
    }

    #[test]
    fn default_is_1() {
        assert_eq!(
            TransactionPreferences::default()
                .default_deposit_guarantee
                .to_string(),
            "1"
        );
    }
}
