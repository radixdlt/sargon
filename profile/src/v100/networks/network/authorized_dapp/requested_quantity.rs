use crate::prelude::*;

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
#[display("{quantifier}: {quantity}")]
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

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn display() {
        assert_eq!(format!("{}", RequestedQuantity::at_least(1)), "AtLeast: 1");
        assert_eq!(
            format!("{}", RequestedQuantity::exactly(1337)),
            "Exactly: 1337"
        );
    }
}
