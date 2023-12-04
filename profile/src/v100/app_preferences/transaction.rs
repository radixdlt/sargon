use std::{cell::Cell, ops::Deref};

use radix_engine_common::math::Decimal;
use radix_engine_toolkit_json::models::common::SerializableDecimal;
use serde::{Deserialize, Serialize};

/// User Preferences relating to submission of transactions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// The deposit guarantee that will automatically be added for
    /// all deposits in transactions.
    default_deposit_guarantee: Cell<SerializableDecimal>,
}

impl Transaction {
    /// Instantiates a new Transaction user preference with the
    /// specified `default_deposit_guarantee` value.
    pub fn new(default_deposit_guarantee: Decimal) -> Self {
        Self {
            default_deposit_guarantee: Cell::new(default_deposit_guarantee.into()),
        }
    }

    /// The deposit guarantee that will automatically be added for
    /// all deposits in transactions.
    pub fn default_deposit_guarantee(&self) -> Decimal {
        *self.default_deposit_guarantee.get().clone().deref()
    }
}

impl Default for Transaction {
    /// By default `1.0` is used.
    fn default() -> Self {
        Self::new(Decimal::one())
    }
}

#[cfg(test)]
mod tests {
    use transaction::prelude::dec;
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::Transaction;

    #[test]
    fn get_decimal() {
        let value = dec!("0.975");
        let sut = Transaction::new(value);
        assert_eq!(sut.default_deposit_guarantee(), value)
    }

    #[test]
    fn json_roundtrip() {
        let sut = Transaction::new(dec!("0.975"));
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "defaultDepositGuarantee": "0.975"
            }
            "#,
        )
    }
}
