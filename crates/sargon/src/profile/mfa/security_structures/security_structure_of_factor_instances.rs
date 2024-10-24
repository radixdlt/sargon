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

impl HasSampleValues for MatrixOfFactorInstances {
    fn sample() -> Self {
        Self {
            primary_role: PrimaryRoleWithFactorInstances::sample(),
            recovery_role: RecoveryRoleWithFactorInstances::sample(),
            confirmation_role: ConfirmationRoleWithFactorInstances::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            primary_role: PrimaryRoleWithFactorInstances::sample_other(),
            recovery_role: RecoveryRoleWithFactorInstances::sample_other(),
            confirmation_role:
                ConfirmationRoleWithFactorInstances::sample_other(),
        }
    }
}

impl HasSampleValues for PrimaryRoleWithFactorInstances {
    fn sample() -> Self {
        Self {
            threshold_factors: vec![FactorInstance::sample()],
            threshold: 1,
            override_factors: vec![FactorInstance::sample()],
        }
    }

    fn sample_other() -> Self {
        Self {
            threshold_factors: vec![FactorInstance::sample_other()],
            threshold: 2,
            override_factors: vec![FactorInstance::sample_other()],
        }
    }
}

impl HasSampleValues for RecoveryRoleWithFactorInstances {
    fn sample() -> Self {
        Self {
            threshold_factors: vec![FactorInstance::sample()],
            threshold: 1,
            override_factors: vec![FactorInstance::sample()],
        }
    }

    fn sample_other() -> Self {
        Self {
            threshold_factors: vec![FactorInstance::sample_other()],
            threshold: 2,
            override_factors: vec![FactorInstance::sample_other()],
        }
    }
}

impl HasSampleValues for ConfirmationRoleWithFactorInstances {
    fn sample() -> Self {
        Self {
            threshold_factors: vec![FactorInstance::sample()],
            threshold: 1,
            override_factors: vec![FactorInstance::sample()],
        }
    }

    fn sample_other() -> Self {
        Self {
            threshold_factors: vec![FactorInstance::sample_other()],
            threshold: 2,
            override_factors: vec![FactorInstance::sample_other()],
        }
    }
}
