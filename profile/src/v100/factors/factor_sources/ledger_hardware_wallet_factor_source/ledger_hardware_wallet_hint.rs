use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use super::ledger_hardware_wallet_model::LedgerHardwareWalletModel;

#[cfg(any(test, feature = "placeholder"))]
use wallet_kit_common::HasPlaceholder;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Getters)]
pub struct LedgerHardwareWalletHint {
    /// "Orange, scratched"
    name: String,

    /// E.g. `nanoS+`
    model: LedgerHardwareWalletModel,
}

impl LedgerHardwareWalletHint {
    pub fn new(name: &str, model: LedgerHardwareWalletModel) -> Self {
        Self {
            name: name.to_string(),
            model,
        }
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for LedgerHardwareWalletHint {
    fn placeholder() -> Self {
        Self::new("Orange, scratched", LedgerHardwareWalletModel::NanoSPlus)
    }

    fn placeholder_other() -> Self {
        Self::new("Old cracked", LedgerHardwareWalletModel::NanoS)
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::{assert_eq_after_json_roundtrip, HasPlaceholder};

    use super::LedgerHardwareWalletHint;

    #[test]
    fn equality() {
        assert_eq!(
            LedgerHardwareWalletHint::placeholder(),
            LedgerHardwareWalletHint::placeholder()
        );
        assert_eq!(
            LedgerHardwareWalletHint::placeholder_other(),
            LedgerHardwareWalletHint::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            LedgerHardwareWalletHint::placeholder(),
            LedgerHardwareWalletHint::placeholder_other()
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = LedgerHardwareWalletHint::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "name": "Orange, scratched",
                "model": "nanoS+"
            }
            "#,
        );
    }
}
