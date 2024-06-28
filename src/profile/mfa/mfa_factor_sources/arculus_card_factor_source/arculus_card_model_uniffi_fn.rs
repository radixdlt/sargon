use crate::prelude::*;

#[uniffi::export]
pub fn arculus_card_model_to_string(model: ArculusCardModel) -> String {
    model.to_string()
}
