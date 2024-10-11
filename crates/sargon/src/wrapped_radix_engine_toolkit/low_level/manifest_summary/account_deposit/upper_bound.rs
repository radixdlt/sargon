use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum UpperBound {
    Inclusive { decimal: Decimal },
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
    pub fn get_amount(&self) -> Option<Decimal192> {
        match self {
            UpperBound::Inclusive { decimal } => Some(*decimal),
            UpperBound::Unbounded => None,
        }
    }
}

impl From<ScryptoUpperBound> for UpperBound {
    fn from(value: ScryptoUpperBound) -> Self {
        match value {
            ScryptoUpperBound::Inclusive(decimal) => Self::Inclusive {
                decimal: decimal.into(),
            },
            ScryptoUpperBound::Unbounded => Self::Unbounded,
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
    fn get_amount() {
        assert_eq!(SUT::sample().get_amount(), Some(Decimal192::from(1)));
        assert_eq!(SUT::sample_other().get_amount(), None);
    }
}
