use crate::prelude::*;

decl_identified_vec_of!(
    /// A collection of [`SchematicOfSecurityShield`](`SecurityStructureConfigurationReference`s)
    SecurityStructuresOfFactorSourceIDs,
    SecurityStructureOfFactorSourceIDs
);

impl HasSampleValues for SecurityStructuresOfFactorSourceIDs {
    fn sample() -> Self {
        Self::from_iter([
            SecurityStructureOfFactorSourceIDs::sample(),
            SecurityStructureOfFactorSourceIDs::sample_other(),
        ])
    }
    fn sample_other() -> Self {
        Self::from_iter([SecurityStructureOfFactorSourceIDs::sample_other()])
    }
}
