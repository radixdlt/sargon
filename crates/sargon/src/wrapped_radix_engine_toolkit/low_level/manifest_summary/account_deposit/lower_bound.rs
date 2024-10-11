use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum LowerBound {
    NonZero,
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
    pub fn get_amount(&self) -> Option<Decimal192> {
        match self {
            LowerBound::Inclusive { decimal } => Some(*decimal),
            LowerBound::NonZero => None,
        }
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
    fn get_amount() {
        assert_eq!(SUT::sample().get_amount(), Some(Decimal192::from(1)));
        assert_eq!(SUT::sample_other().get_amount(), None);
    }
}
