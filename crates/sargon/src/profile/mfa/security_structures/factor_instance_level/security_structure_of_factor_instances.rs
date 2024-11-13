use crate::prelude::*;

decl_matrix_of_factors!(
    /// A matrix of FactorInstances
    FactorInstance
);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SecurityStructureOfFactorInstances {
    /// The ID of the `SecurityStructureOfFactorSourceIDs` in
    /// `profile.app_preferences.security.security_structures_of_factor_source_ids`
    /// which was used to derive the factor instances in this structure. Or rather:
    /// The id of `SecurityStructureOfFactorSources`.
    pub security_structure_id: SecurityStructureID,

    /// The structure of factors to use for certain roles, Primary, Recovery
    /// and Confirmation role.
    pub matrix_of_factors: MatrixOfFactorInstances,
}

impl SecurityStructureOfFactorInstances {
    pub fn new(
        security_structure_id: SecurityStructureID,
        matrix_of_factors: MatrixOfFactorInstances,
    ) -> Self {
        Self {
            security_structure_id,
            matrix_of_factors,
        }
    }
}

impl Identifiable for SecurityStructureOfFactorInstances {
    type ID = <SecurityStructureMetadata as Identifiable>::ID;

    fn id(&self) -> Self::ID {
        self.security_structure_id
    }
}

impl HasSampleValues for SecurityStructureOfFactorInstances {
    fn sample() -> Self {
        Self {
            security_structure_id: SecurityStructureID::sample(),
            matrix_of_factors: MatrixOfFactorInstances::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            security_structure_id: SecurityStructureID::sample_other(),
            matrix_of_factors: MatrixOfFactorInstances::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityStructureOfFactorInstances;

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
