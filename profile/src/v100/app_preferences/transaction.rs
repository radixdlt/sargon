use std::sync::Arc;

use radix_engine_toolkit_json::models::common::SerializableDecimal;
use serde::{Deserialize, Serialize};

use crate::Decimal;
#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

/// User Preferences relating to submission of transactions.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// The deposit guarantee that will automatically be added for
    /// all deposits in transactions.
    pub default_deposit_guarantee: Arc<Decimal>,
}

impl Transaction {
    /// Instantiates a new Transaction user preference with the
    /// specified `default_deposit_guarantee` value.
    pub fn new(default_deposit_guarantee: Decimal) -> Self {
        Self {
            default_deposit_guarantee: Arc::new(default_deposit_guarantee),
        }
    }
}

impl Default for Transaction {
    /// By default `1.0` is used.
    fn default() -> Self {
        Self {
            default_deposit_guarantee: Decimal::one(),
        }
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for Transaction {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::new(Decimal::try_from_str("0.975").unwrap())
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::new(Decimal::try_from_str("0.765").unwrap())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::{assert_eq_after_json_roundtrip, Decimal, HasPlaceholder};

    use super::Transaction;

    #[test]
    fn equality() {
        assert_eq!(Transaction::placeholder(), Transaction::placeholder());
        assert_eq!(
            Transaction::placeholder_other(),
            Transaction::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(Transaction::placeholder(), Transaction::placeholder_other());
    }

    #[test]
    fn get_decimal() {
        let value = Decimal::new("0.975".to_string()).unwrap();
        let sut = Transaction::new(value.deref().clone());
        assert_eq!(sut.default_deposit_guarantee, value)
    }

    #[test]
    fn json_roundtrip() {
        let sut = Transaction::placeholder();
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
            Transaction::default()
                .default_deposit_guarantee
                .to_string(),
            "1"
        );
    }

    #[test]
    fn set_default_deposit_guarantee() {
        let mut sut = Transaction::default();
        sut.default_deposit_guarantee = Decimal::new("0.237".to_string()).unwrap();
        assert_eq!(sut.default_deposit_guarantee.to_string(), "0.237");
    }
}
