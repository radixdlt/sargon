use crate::prelude::*;
use sargon::ArculusCardHint as InternalArculusCardHint;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct ArculusCardHint {
    /// E.g. "Black" or "Silver"
    pub name: String,

    pub model: ArculusCardModel,
}

impl From<InternalArculusCardHint> for ArculusCardHint {
    fn from(value: InternalArculusCardHint) -> Self {
        Self {
            name: value.name,
            model: value.model.into(),
        }
    }
}

impl Into<InternalArculusCardHint> for ArculusCardHint {
    fn into(self) -> InternalArculusCardHint {
        InternalArculusCardHint {
            name: self.name,
            model: self.model.into(),
        }
    }
}

#[uniffi::export]
pub fn arculus_card_model_to_string(model: ArculusCardModel) -> String {
    model.into_internal().to_string()
}
