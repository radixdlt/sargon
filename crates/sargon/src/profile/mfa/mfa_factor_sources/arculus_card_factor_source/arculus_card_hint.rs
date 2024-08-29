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
    uniffi::Record,
)]
#[display("{name} {model}")]
pub struct ArculusCardHint {
    /// E.g. "Black" or "Silver"
    pub name: String,

    pub model: ArculusCardModel,
}

impl ArculusCardHint {
    pub fn new(name: &str, model: ArculusCardModel) -> Self {
        Self {
            name: name.to_string(),
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
                "name": "Silver",
                "model": "arculusColdStorageWallet"
            }
            "#,
        );
    }
}
