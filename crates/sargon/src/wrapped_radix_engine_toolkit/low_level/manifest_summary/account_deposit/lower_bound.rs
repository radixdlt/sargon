use crate::prelude::*;
use radix_common::prelude::ManifestResourceConstraint;

/// Represents a lower bound on a non-negative decimal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LowerBound {
    /// Represents a lower bound of an infinitesimal amount above 0, and is included for
    /// clarity of intention. Considering Decimal has a limited precision of 10^(-18), it is roughly
    /// equivalent to an inclusive bound of 10^(-18), or Decimal::from_attos(1).
    NonZero,

    /// The amount is required to be non-negative before using this model.
    /// This can be validated via [`ManifestResourceConstraint::is_valid_for`].
    Inclusive { decimal: Decimal },
}

impl LowerBound {
    pub fn inclusive(decimal: impl Into<Decimal>) -> Self {
        Self::Inclusive {
            decimal: decimal.into(),
        }
    }
    pub fn non_zero() -> Self {
        Self::NonZero
    }
}

impl LowerBound {
    pub fn get_amount(&self) -> Decimal {
        ScryptoLowerBound::from(self.clone())
            .equivalent_decimal()
            .into()
    }
}

impl From<ScryptoLowerBound> for LowerBound {
    fn from(value: ScryptoLowerBound) -> Self {
        match value {
            ScryptoLowerBound::Inclusive(decimal) => Self::Inclusive {
                decimal: decimal.into(),
            },
            ScryptoLowerBound::NonZero => Self::NonZero,
        }
    }
}

impl From<LowerBound> for ScryptoLowerBound {
    fn from(value: LowerBound) -> Self {
        match value {
            LowerBound::Inclusive { decimal } => {
                ScryptoLowerBound::Inclusive(decimal.into())
            }
            LowerBound::NonZero => ScryptoLowerBound::NonZero,
        }
    }
}

impl HasSampleValues for LowerBound {
    fn sample() -> Self {
        Self::inclusive(1)
    }

    fn sample_other() -> Self {
        Self::non_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LowerBound;

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
    fn from_scrypto_inclusive() {
        let scrypto = ScryptoLowerBound::Inclusive(1.into());
        assert_eq!(SUT::from(scrypto), SUT::sample())
    }

    #[test]
    fn from_scrypto_none_zero() {
        let scrypto = ScryptoLowerBound::NonZero;
        assert_eq!(SUT::from(scrypto), SUT::sample_other())
    }

    #[test]
    fn to_scrypto_inclusive() {
        let lower_bound = LowerBound::inclusive(1);
        let scrypto: ScryptoLowerBound = lower_bound.into();
        assert_eq!(scrypto, ScryptoLowerBound::Inclusive(1.into()));
    }

    #[test]
    fn to_scrypto_none_zero() {
        let lower_bound = LowerBound::non_zero();
        let scrypto: ScryptoLowerBound = lower_bound.into();
        assert_eq!(scrypto, ScryptoLowerBound::NonZero);
    }

    #[test]
    fn get_amount() {
        assert_eq!(SUT::sample().get_amount(), Decimal::from(1));
    }
}
