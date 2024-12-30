use crate::prelude::*;

/// Represents the bounds for a simple fungible resource, which can
/// be exact, at most, at least, between, or unknown amount.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimpleCountedResourceBounds {
    Exact {
        amount: Decimal,
    },
    AtMost {
        amount: Decimal,
    },
    AtLeast {
        amount: Decimal,
    },
    Between {
        min_amount: Decimal,
        max_amount: Decimal,
    },
    UnknownAmount,
}

impl SimpleCountedResourceBounds {
    pub fn exact(amount: impl Into<Decimal>) -> Self {
        Self::Exact {
            amount: amount.into(),
        }
    }

    pub fn at_most(amount: impl Into<Decimal>) -> Self {
        Self::AtMost {
            amount: amount.into(),
        }
    }

    pub fn at_least(amount: impl Into<Decimal>) -> Self {
        Self::AtLeast {
            amount: amount.into(),
        }
    }

    /// # Panics
    /// Panics if `max_amount` is less than `min_amount`.
    pub fn between(
        min_amount: impl Into<Decimal>,
        max_amount: impl Into<Decimal>,
    ) -> Self {
        let max_amount = max_amount.into();
        let min_amount = min_amount.into();

        assert!(
            max_amount >= min_amount,
            "Max amount MUST be greater than or equal min amount."
        );

        Self::Between {
            min_amount,
            max_amount,
        }
    }

    pub fn unknown_amount() -> Self {
        Self::UnknownAmount
    }
}

impl From<ScryptoSimpleFungibleResourceBounds> for SimpleCountedResourceBounds {
    fn from(value: ScryptoSimpleFungibleResourceBounds) -> Self {
        match value {
            ScryptoSimpleFungibleResourceBounds::Exact(amount) => {
                Self::exact(amount)
            }
            ScryptoSimpleFungibleResourceBounds::AtMost(amount) => {
                Self::at_most(amount)
            }
            ScryptoSimpleFungibleResourceBounds::AtLeast(amount) => {
                Self::at_least(amount)
            }
            ScryptoSimpleFungibleResourceBounds::Between(
                min_amount,
                max_amount,
            ) => Self::between(min_amount, max_amount),
            ScryptoSimpleFungibleResourceBounds::UnknownAmount => {
                Self::unknown_amount()
            }
        }
    }
}

impl HasSampleValues for SimpleCountedResourceBounds {
    fn sample() -> Self {
        Self::sample_exact()
    }

    fn sample_other() -> Self {
        Self::sample_unknown_amount()
    }
}

impl SimpleCountedResourceBounds {
    fn sample_exact() -> Self {
        Self::exact(1337)
    }

    #[allow(dead_code)]
    fn sample_at_most() -> Self {
        Self::at_most(3)
    }

    #[allow(dead_code)]
    fn sample_at_least() -> Self {
        Self::at_least(2)
    }

    #[allow(dead_code)]
    fn sample_between() -> Self {
        Self::between(1, 3)
    }

    fn sample_unknown_amount() -> Self {
        Self::unknown_amount()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SimpleCountedResourceBounds;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
        assert_ne!(SUT::sample_between(), SUT::sample_at_least());
        assert_ne!(SUT::sample_at_most(), SUT::sample_at_least());
    }

    #[test]
    fn from_scrypto_exact() {
        let scrypto = ScryptoSimpleFungibleResourceBounds::Exact(1337.into());
        assert_eq!(SUT::from(scrypto), SUT::sample_exact());
    }

    #[test]
    fn from_scrypto_at_most() {
        let scrypto = ScryptoSimpleFungibleResourceBounds::AtMost(3.into());
        assert_eq!(SUT::from(scrypto), SUT::sample_at_most());
    }

    #[test]
    fn from_scrypto_at_least() {
        let scrypto = ScryptoSimpleFungibleResourceBounds::AtLeast(2.into());
        assert_eq!(SUT::from(scrypto), SUT::sample_at_least());
    }

    #[test]
    fn from_scrypto_between() {
        let scrypto = ScryptoSimpleFungibleResourceBounds::Between(
            Decimal::from(1).into(),
            Decimal::from(3).into(),
        );
        assert_eq!(SUT::from(scrypto), SUT::sample_between());
    }

    #[test]
    fn from_scrypto_unknown_amount() {
        let scrypto = ScryptoSimpleFungibleResourceBounds::UnknownAmount;
        assert_eq!(SUT::from(scrypto), SUT::sample_unknown_amount());
    }

    #[test]
    #[should_panic(
        expected = "Max amount MUST be greater than or equal min amount."
    )]
    fn between_should_panic_when_max_less_than_min() {
        SUT::between(3, 1);
    }
}
