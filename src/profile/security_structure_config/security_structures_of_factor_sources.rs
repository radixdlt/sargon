use crate::prelude::*;

decl_identified_vec_of!(
    /// A collection of [`SchematicOfSecurityShield`](`SecurityStructureConfigurationReference`s)
    SecurityStructuresOfFactorSources,
    SecurityStructureOfFactorSources
);

impl HasSampleValues for SecurityStructuresOfFactorSources {
    fn sample() -> Self {
        Self::from_iter([
            SecurityStructureOfFactorSources::sample(),
            SecurityStructureOfFactorSources::sample_other(),
        ])
    }
    fn sample_other() -> Self {
        Self::from_iter([SecurityStructureOfFactorSources::sample_other()])
    }
}
