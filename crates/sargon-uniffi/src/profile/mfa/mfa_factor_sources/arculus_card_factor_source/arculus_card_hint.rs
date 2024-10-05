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

#[uniffi::export]
pub fn arculus_card_model_to_string(model: ArculusCardModel) -> String {
    model.to_string()
}
