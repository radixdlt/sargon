struct PrimaryRoleWithFactorInstances {
    threshold_factors: Vec<FactorInstance>,
    threshold: u16,
    override_factors: Vec<FactorInstance>,
}

struct MatrixOfFactorSourceInstances {
    primary_role: PrimaryRoleWithFactorInstances,
    recovery_role: RecoveryRoleWithFactorInstances,
    confirmation_role: ConfirmationRoleWithFactorInstances,
}

struct SecurityStructureOfFactorInstances {
    metadata: SecurityStructureMetadata,
    number_of_epochs_until_auto_confirmation: u64,
    matrix_of_factors: MatrixOfFactorSourceInstances
}