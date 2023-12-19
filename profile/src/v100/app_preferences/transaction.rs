use std::cell::RefCell;

use radix_engine_common::math::Decimal;
use radix_engine_toolkit_json::models::common::SerializableDecimal;
use serde::{Deserialize, Serialize};

/// User Preferences relating to submission of transactions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// The deposit guarantee that will automatically be added for
    /// all deposits in transactions.
    default_deposit_guarantee: RefCell<SerializableDecimal>,
}

impl Transaction {
    /// Instantiates a new Transaction user preference with the
    /// specified `default_deposit_guarantee` value.
    pub fn new(default_deposit_guarantee: Decimal) -> Self {
        Self {
            default_deposit_guarantee: RefCell::new(default_deposit_guarantee.into()),
        }
    }

    /// The deposit guarantee that will automatically be added for
    /// all deposits in transactions.
    pub fn default_deposit_guarantee(&self) -> Decimal {
        *self.default_deposit_guarantee.borrow().clone()
    }

    pub fn set_default_deposit_guarantee(&self, new: Decimal) {
        *self.default_deposit_guarantee.borrow_mut() = new.into()
    }
}

impl Default for Transaction {
    /// By default `1.0` is used.
    fn default() -> Self {
        Self::new(Decimal::one())
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl Transaction {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::new(transaction::prelude::dec!("0.975"))
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_other() -> Self {
        Self::new(transaction::prelude::dec!("0.765"))
    }
}

#[cfg(test)]
mod tests {
    use transaction::prelude::dec;
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::Transaction;

    #[test]
    fn inequality() {
        assert_ne!(Transaction::placeholder(), Transaction::placeholder_other());
    }

    #[test]
    fn get_decimal() {
        let value = dec!("0.975");
        let sut = Transaction::new(value);
        assert_eq!(sut.default_deposit_guarantee(), value)
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
                .default_deposit_guarantee()
                .to_string(),
            "1"
        );
    }

    #[test]
    fn set_default_deposit_guarantee() {
        let sut = Transaction::default();
        sut.set_default_deposit_guarantee(dec!("0.237"));
        assert_eq!(sut.default_deposit_guarantee().to_string(), "0.237");
    }
}
