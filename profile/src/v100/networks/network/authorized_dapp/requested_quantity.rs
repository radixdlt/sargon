use crate::prelude::*;

/// A quantifier of a quantity, either `atLeast` or `exactly`, as in
/// "I want AT LEAST 3" or "I want EXACTLY 10".
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
pub enum RequestedNumberQuantifier {
    Exactly,
    AtLeast,
}

impl std::fmt::Display for RequestedNumberQuantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).expect("Should always be able to serialize RequestedNumberQuantifier into JSON string"))
    }
}

/// A requested (by dApp) quantity, e.g. "I want AT LEAST 3 account addresses" or
/// "I want EXACTLY 2 email addresses".
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{quantifier}{quantity}")]
pub struct RequestedQuantity {
    pub quantifier: RequestedNumberQuantifier,
    pub quantity: u16,
}

impl RequestedQuantity {
    pub fn is_valid(&self) -> bool {
        match self.quantifier {
            RequestedNumberQuantifier::Exactly => self.quantity != 0,
            _ => true,
        }
    }

    pub fn exactly(quantity: u16) -> Self {
        let value = Self {
            quantifier: RequestedNumberQuantifier::Exactly,
            quantity,
        };
        assert!(value.is_valid());
        value
    }
    pub fn at_least(quantity: u16) -> Self {
        let value = Self {
            quantifier: RequestedNumberQuantifier::AtLeast,
            quantity,
        };
        assert!(value.is_valid());
        value
    }
}
