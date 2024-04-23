use crate::prelude::*;

/// A requested (by Dapp) quantity, e.g. "I want AT LEAST 3 account addresses" or
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

impl HasSampleValues for RequestedQuantity {
    fn sample() -> Self {
        Self::exactly(1)
    }

    fn sample_other() -> Self {
        Self::at_least(1)
    }
}

impl RequestedQuantity {
    pub fn assert_is_valid(&self) {
        if !self.is_valid() {
            panic!("Invalid quantity {self}")
        }
    }

    pub fn is_valid(&self) -> bool {
        match self.quantifier {
            RequestedNumberQuantifier::Exactly => self.quantity != 0,
            _ => true,
        }
    }

    /// Checks `len` can fulfill the [`RequestedQuantity`] (self), `len` is
    /// considered to be fulfilling the requested quantity:
    /// * if: quantifier == ::Exactly && len == quantity // ✅ fulfills
    /// * else if: quantifier == ::AtLeast && len >= quantity // ✅ fulfills
    /// * else false // ❌ does NOT fulfill
    pub fn is_fulfilled_by_ids(&self, len: usize) -> bool {
        let quantity = self.quantity as usize;
        match self.quantifier {
            RequestedNumberQuantifier::Exactly => len == quantity,
            RequestedNumberQuantifier::AtLeast => len >= quantity,
        }
    }

    pub fn exactly(quantity: u16) -> Self {
        let value = Self {
            quantifier: RequestedNumberQuantifier::Exactly,
            quantity,
        };
        value.assert_is_valid();
        value
    }

    pub fn at_least(quantity: u16) -> Self {
        let value = Self {
            quantifier: RequestedNumberQuantifier::AtLeast,
            quantity,
        };
        value.assert_is_valid();
        value
    }
}

impl HasSampleValues for RequestedQuantity {
    fn sample() -> Self {
        Self::exactly(1)
    }

    fn sample_other() -> Self {
        Self::at_least(1)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RequestedQuantity;

    #[test]
    fn display() {
        assert_eq!(format!("{}", RequestedQuantity::at_least(1)), "AtLeast: 1");
        assert_eq!(
            format!("{}", RequestedQuantity::exactly(1337)),
            "Exactly: 1337"
        );
    }

    #[test]
    fn at_least() {
        let sut = SUT::at_least(0);
        assert_eq!(sut.quantifier, RequestedNumberQuantifier::AtLeast);
        assert_eq!(sut.quantity, 0);
    }

    #[test]
    fn exactly() {
        let sut = SUT::exactly(1337);
        assert_eq!(sut.quantifier, RequestedNumberQuantifier::Exactly);
        assert_eq!(sut.quantity, 1337);
    }

    #[test]
    #[should_panic(expected = "Invalid quantity Exactly: 0")]
    fn exactly_0_is_invalid() {
        _ = SUT::exactly(0);
    }

    #[test]
    fn at_least_fulfills_true() {
        assert!(SUT::at_least(0).is_fulfilled_by_ids(0));
        assert!(SUT::at_least(0).is_fulfilled_by_ids(1));
        assert!(SUT::at_least(1).is_fulfilled_by_ids(1));
        assert!(SUT::at_least(1).is_fulfilled_by_ids(2));
    }

    #[test]
    fn at_least_fulfills_false() {
        assert!(!SUT::at_least(1).is_fulfilled_by_ids(0));
        assert!(!SUT::at_least(10).is_fulfilled_by_ids(0));
        assert!(!SUT::at_least(10).is_fulfilled_by_ids(9));
    }

    #[test]
    fn exactly_fulfills_true() {
        assert!(SUT::exactly(1).is_fulfilled_by_ids(1));
        assert!(SUT::exactly(10).is_fulfilled_by_ids(10));
    }

    #[test]
    fn exactly_fulfills_false() {
        assert!(!SUT::exactly(1).is_fulfilled_by_ids(0));
        assert!(!SUT::exactly(1).is_fulfilled_by_ids(2));
        assert!(!SUT::exactly(10).is_fulfilled_by_ids(9));
        assert!(!SUT::exactly(10).is_fulfilled_by_ids(11));
    }
}
