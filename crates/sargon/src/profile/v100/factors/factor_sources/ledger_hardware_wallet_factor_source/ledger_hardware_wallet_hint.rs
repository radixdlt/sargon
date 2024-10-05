use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
)]
#[display("{name} {model}")]
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

impl HasSampleValues for LedgerHardwareWalletHint {
    fn sample() -> Self {
        Self::new("Orange, scratched", LedgerHardwareWalletModel::NanoSPlus)
    }

    fn sample_other() -> Self {
        Self::new("Old cracked", LedgerHardwareWalletModel::NanoS)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            LedgerHardwareWalletHint::sample(),
            LedgerHardwareWalletHint::sample()
        );
        assert_eq!(
            LedgerHardwareWalletHint::sample_other(),
            LedgerHardwareWalletHint::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            LedgerHardwareWalletHint::sample(),
            LedgerHardwareWalletHint::sample_other()
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = LedgerHardwareWalletHint::sample();
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
