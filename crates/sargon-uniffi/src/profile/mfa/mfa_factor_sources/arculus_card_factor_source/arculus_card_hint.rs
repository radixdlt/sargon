use crate::prelude::*;
use sargon::ArculusCardHint as InternalArculusCardHint;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct ArculusCardHint {
    /// E.g. "Black" or "Silver"
    pub name: String,

    pub model: ArculusCardModel,
}

#[uniffi::export]
pub fn arculus_card_model_to_string(model: ArculusCardModel) -> String {
    model.into_internal().to_string()
}
