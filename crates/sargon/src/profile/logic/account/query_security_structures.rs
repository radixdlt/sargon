use crate::prelude::*;

decl_identified_vec_of!(
    /// A collection of [`SecurityStructureOfFactorSources`]
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

impl Profile {
    /// Returns all the SecurityStructuresOfFactorSources,
    /// by trying to map FactorSourceID level -> FactorSource Level
    pub fn security_structures_of_factor_sources(
        &self,
    ) -> Result<SecurityStructuresOfFactorSources> {
        self.app_preferences
            .security
            .security_structures_of_factor_source_ids
            .iter()
            .map(|id| {
                SecurityStructureOfFactorSources::try_from((
                    &id,
                    &self.factor_sources,
                ))
            })
            .collect::<Result<SecurityStructuresOfFactorSources>>()
    }
}
