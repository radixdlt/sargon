use crate::prelude::*;

use super::decl_matrix_macro::matrix_conversion;

matrix_conversion!(
    /// Matrix of `FactorSource`s containing the primary, recovery, and confirmation roles with `FactorSource`s
    FactorSource
);

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = MatrixOfFactorSources;

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
