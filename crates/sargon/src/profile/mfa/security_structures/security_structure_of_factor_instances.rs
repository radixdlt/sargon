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
        Self::new(
            PrimaryRoleWithFactorInstances::sample(),
            RecoveryRoleWithFactorInstances::sample(),
            ConfirmationRoleWithFactorInstances::sample(),
        )
        .unwrap()
    }

    fn sample_other() -> Self {
        Self::new(
            PrimaryRoleWithFactorInstances::sample_other(),
            RecoveryRoleWithFactorInstances::sample_other(),
            ConfirmationRoleWithFactorInstances::sample_other(),
        )
        .unwrap()
    }
}

impl HasSampleValues for PrimaryRoleWithFactorInstances {
    fn sample() -> Self {
        Self::new([FactorInstance::sample()], 1, [FactorInstance::sample()])
            .unwrap()
    }

    fn sample_other() -> Self {
        Self::new(
            [FactorInstance::sample_other()],
            2,
            [FactorInstance::sample_other()],
        )
        .unwrap()
    }
}

impl HasSampleValues for RecoveryRoleWithFactorInstances {
    fn sample() -> Self {
        Self::new([FactorInstance::sample()], 1, [FactorInstance::sample()])
            .unwrap()
    }

    fn sample_other() -> Self {
        Self::new(
            [FactorInstance::sample_other()],
            2,
            [FactorInstance::sample_other()],
        )
        .unwrap()
    }
}

impl HasSampleValues for ConfirmationRoleWithFactorInstances {
    fn sample() -> Self {
        Self::new([FactorInstance::sample()], 1, [FactorInstance::sample()])
            .unwrap()
    }

    fn sample_other() -> Self {
        Self::new(
            [FactorInstance::sample_other()],
            2,
            [FactorInstance::sample_other()],
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests_primary {
    use super::*;

    type Sut = PrimaryRoleWithFactorInstances;

    #[test]
    fn primary_role_non_securified_threshold_instances_is_err() {
        assert!(matches!(
            Sut::new(
                [
                    HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0).into()
                ], 
                1,
                [] // no override
            ),
            Err(CommonError::IndexUnsecurifiedExpectedSecurified)
        ));
    }
}
