use crate::prelude::*;
use sargon::ArculusCardHint as InternalArculusCardHint;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct ArculusCardHint {
    /// A user-assigned name for the arculus card, intended to help users
    /// differentiate between multiple arculus cards.
    ///
    /// E.g. "Black" or "Silver"
    pub label: String,

    pub model: ArculusCardModel,
}

#[uniffi::export]
pub fn arculus_card_model_to_string(model: ArculusCardModel) -> String {
    model.into_internal().to_string()
}
