use crate::prelude::*;

/// Represents the bounds for a simple fungible resource, which can
/// be exact, at most, at least, between, or unknown amount.
#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum SimpleFungibleResourceBounds {
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

impl SimpleFungibleResourceBounds {
    pub fn exact(amount: Decimal) -> Self {
        Self::Exact { amount }
    }

    pub fn at_most(amount: Decimal) -> Self {
        Self::AtMost { amount }
    }

    pub fn at_least(amount: Decimal) -> Self {
        Self::AtLeast { amount }
    }

    pub fn between(min_amount: Decimal, max_amount: Decimal) -> Self {
        Self::Between {
            min_amount,
            max_amount,
        }
    }

    pub fn unknown_amount() -> Self {
        Self::UnknownAmount
    }
}

impl From<ScryptoSimpleFungibleResourceBounds>
    for SimpleFungibleResourceBounds
{
    fn from(value: ScryptoSimpleFungibleResourceBounds) -> Self {
        match value {
            ScryptoSimpleFungibleResourceBounds::Exact(amount) => Self::Exact {
                amount: amount.into(),
            },
            ScryptoSimpleFungibleResourceBounds::AtMost(amount) => {
                Self::AtMost {
                    amount: amount.into(),
                }
            }
            ScryptoSimpleFungibleResourceBounds::AtLeast(amount) => {
                Self::AtLeast {
                    amount: amount.into(),
                }
            }
            ScryptoSimpleFungibleResourceBounds::Between(
                min_amount,
                max_amount,
            ) => Self::Between {
                min_amount: min_amount.into(),
                max_amount: max_amount.into(),
            },
            ScryptoSimpleFungibleResourceBounds::UnknownAmount => {
                Self::UnknownAmount
            }
        }
    }
}

impl HasSampleValues for SimpleFungibleResourceBounds {
    fn sample() -> Self {
        Self::sample_exact()
    }

    fn sample_other() -> Self {
        Self::sample_unknown_amount()
    }
}

impl SimpleFungibleResourceBounds {
    fn sample_exact() -> Self {
        Self::exact(Decimal::from(1337))
    }

    fn sample_at_most() -> Self {
        Self::at_most(Decimal::from(3))
    }

    fn sample_at_least() -> Self {
        Self::at_least(Decimal::from(2))
    }

    fn sample_between() -> Self {
        Self::between(Decimal::from(1), Decimal::from(3))
    }

    fn sample_unknown_amount() -> Self {
        Self::unknown_amount()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SimpleFungibleResourceBounds;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn from_scrypto_exact() {
        let scrypto = ScryptoSimpleFungibleResourceBounds::Exact(
            Decimal::from(1337).into(),
        );
        assert_eq!(SUT::from(scrypto), SUT::sample_exact());
    }

    #[test]
    fn from_scrypto_at_most() {
        let scrypto = ScryptoSimpleFungibleResourceBounds::AtMost(
            Decimal::from(3).into(),
        );
        assert_eq!(SUT::from(scrypto), SUT::sample_at_most());
    }

    #[test]
    fn from_scrypto_at_least() {
        let scrypto = ScryptoSimpleFungibleResourceBounds::AtLeast(
            Decimal::from(2).into(),
        );
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
}
