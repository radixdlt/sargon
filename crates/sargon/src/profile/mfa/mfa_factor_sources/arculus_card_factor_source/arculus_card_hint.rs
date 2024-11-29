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
#[display("{label} {model}")]
pub struct ArculusCardHint {
    /// A user-assigned name for the arculus card, intended to help users
    /// differentiate between multiple arculus cards.
    /// 
    /// E.g. "Black" or "Silver"
    pub label: String,

    pub model: ArculusCardModel,
}

impl ArculusCardHint {
    pub fn new(label: &str, model: ArculusCardModel) -> Self {
        Self {
            label: label.to_string(),
            model,
        }
    }
}

impl HasSampleValues for ArculusCardHint {
    fn sample() -> Self {
        Self::new("Silver", ArculusCardModel::default())
    }

    fn sample_other() -> Self {
        Self::new("Black", ArculusCardModel::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(ArculusCardHint::sample(), ArculusCardHint::sample());
        assert_eq!(
            ArculusCardHint::sample_other(),
            ArculusCardHint::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(ArculusCardHint::sample(), ArculusCardHint::sample_other());
    }

    #[test]
    fn json_roundtrip() {
        let model = ArculusCardHint::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "label": "Silver",
                "model": "arculusColdStorageWallet"
            }
            "#,
        );
    }
}
