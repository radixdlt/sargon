use crate::prelude::*;
use radix_common::prelude::ManifestResourceConstraint;

/// Represents an upper bound on a non-negative decimal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpperBound {
    /// The amount is required to be non-negative before using this model.
    /// This can be validated via [`ManifestResourceConstraint::is_valid_for`].
    Inclusive { decimal: Decimal },

    /// `Unbounded` represents an upper bound above any possible decimal, and is included for
    /// clarity of intention. Considering Decimal has a max size, it is effectively equivalent to
    /// an inclusive bound of [`Decimal::max()`].
    Unbounded,
}

impl UpperBound {
    pub fn inclusive(decimal: impl Into<Decimal>) -> Self {
        Self::Inclusive {
            decimal: decimal.into(),
        }
    }
    pub fn unbounded() -> Self {
        Self::Unbounded
    }
}

impl UpperBound {
    pub fn get_amount(&self) -> Decimal {
        ScryptoUpperBound::from(self.clone())
            .equivalent_decimal()
            .into()
    }
}

impl From<ScryptoUpperBound> for UpperBound {
    fn from(value: ScryptoUpperBound) -> Self {
        match value {
            ScryptoUpperBound::Inclusive(decimal) => Self::inclusive(decimal),
            ScryptoUpperBound::Unbounded => Self::unbounded(),
        }
    }
}

impl From<UpperBound> for ScryptoUpperBound {
    fn from(value: UpperBound) -> Self {
        match value {
            UpperBound::Inclusive { decimal } => {
                ScryptoUpperBound::Inclusive(decimal.into())
            }
            UpperBound::Unbounded => ScryptoUpperBound::Unbounded,
        }
    }
}

impl HasSampleValues for UpperBound {
    fn sample() -> Self {
        Self::inclusive(1)
    }

    fn sample_other() -> Self {
        Self::unbounded()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = UpperBound;

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
        let scrypto = ScryptoUpperBound::Inclusive(1.into());
        assert_eq!(SUT::from(scrypto), SUT::sample())
    }

    #[test]
    fn from_scrypto_unbounded() {
        let scrypto = ScryptoUpperBound::Unbounded;
        assert_eq!(SUT::from(scrypto), SUT::sample_other())
    }

    #[test]
    fn to_scrypto_inclusive() {
        let upper_bound = SUT::sample();
        let scrypto: ScryptoUpperBound = upper_bound.into();
        assert_eq!(scrypto, ScryptoUpperBound::Inclusive(1.into()));
    }

    #[test]
    fn to_scrypto_unbounded() {
        let upper_bound = SUT::sample_other();
        let scrypto: ScryptoUpperBound = upper_bound.into();
        assert_eq!(scrypto, ScryptoUpperBound::Unbounded);
    }

    #[test]
    fn get_amount() {
        assert_eq!(SUT::sample().get_amount(), Decimal::from(1));
        assert_eq!(SUT::sample_other().get_amount(), Decimal::max());
    }
}
