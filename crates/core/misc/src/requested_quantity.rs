use crate::prelude::*;

/// A requested (by Dapp) quantity, e.g. "I want AT LEAST 3 account addresses" or
/// "I want EXACTLY 2 email addresses".
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
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

    pub fn left_until_fulfilled(&self, actual: usize) -> i32 {
        let actual = actual as i32;
        let quantity = self.quantity as i32;
        let diff = quantity - actual;
        match self.quantifier {
            RequestedNumberQuantifier::Exactly => diff,
            RequestedNumberQuantifier::AtLeast => diff.max(0),
        }
    }

    /// Checks `actual` can fulfill the [`RequestedQuantity`] (self), `actual` is
    /// considered to be fulfilling the requested quantity:
    /// * if: quantifier == ::Exactly && actual == quantity // ✅ fulfills
    /// * else if: quantifier == ::AtLeast && actual >= quantity // ✅ fulfills
    /// * else false // ❌ does NOT fulfill
    pub fn is_fulfilled_by_quantity(&self, actual: usize) -> bool {
        self.left_until_fulfilled(actual) == 0
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(SUT::at_least(0).is_fulfilled_by_quantity(0));
        assert!(SUT::at_least(0).is_fulfilled_by_quantity(1));
        assert!(SUT::at_least(1).is_fulfilled_by_quantity(1));
        assert!(SUT::at_least(1).is_fulfilled_by_quantity(2));
    }

    #[test]
    fn at_least_fulfills_false() {
        assert!(!SUT::at_least(1).is_fulfilled_by_quantity(0));
        assert!(!SUT::at_least(10).is_fulfilled_by_quantity(0));
        assert!(!SUT::at_least(10).is_fulfilled_by_quantity(9));
    }

    #[test]
    fn exactly_fulfills_true() {
        assert!(SUT::exactly(1).is_fulfilled_by_quantity(1));
        assert!(SUT::exactly(10).is_fulfilled_by_quantity(10));
    }

    #[test]
    fn exactly_fulfills_false() {
        assert!(!SUT::exactly(1).is_fulfilled_by_quantity(0));
        assert!(!SUT::exactly(1).is_fulfilled_by_quantity(2));
        assert!(!SUT::exactly(10).is_fulfilled_by_quantity(9));
        assert!(!SUT::exactly(10).is_fulfilled_by_quantity(11));
    }

    #[test]
    fn left_until_fulfilled() {
        assert_eq!(SUT::exactly(5).left_until_fulfilled(1), 4);
        assert_eq!(SUT::exactly(9).left_until_fulfilled(3), 6);
        assert_eq!(SUT::exactly(5).left_until_fulfilled(6), -1);
        assert_eq!(SUT::exactly(9).left_until_fulfilled(17), -8);

        assert_eq!(SUT::at_least(5).left_until_fulfilled(1), 4);
        assert_eq!(SUT::at_least(9).left_until_fulfilled(2), 7);
        assert_eq!(SUT::at_least(5).left_until_fulfilled(7), 0);
        assert_eq!(SUT::at_least(13).left_until_fulfilled(18), 0);
    }
}
