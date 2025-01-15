use crate::prelude::*;

/// A kind of factor list, either threshold, or override kind.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum FactorListKind {
    Threshold,
    Override,
}

impl HasSampleValues for FactorListKind {
    fn sample() -> Self {
        FactorListKind::Threshold
    }

    fn sample_other() -> Self {
        FactorListKind::Override
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorListKind;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
