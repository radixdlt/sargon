use crate::prelude::*;

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
