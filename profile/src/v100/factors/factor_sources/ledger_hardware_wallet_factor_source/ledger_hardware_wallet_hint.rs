use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct LedgerHardwareWalletHint {
    /// "Orange, scratched"
    pub name: String,

    /// E.g. `nanoS+`
    pub model: LedgerHardwareWalletModel,
}

impl LedgerHardwareWalletHint {
    pub fn new(name: &str, model: LedgerHardwareWalletModel) -> Self {
        Self {
            name: name.to_string(),
            model,
        }
    }
}

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
    use crate::prelude::*;

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
