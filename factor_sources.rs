struct PrimaryRoleWithFactorSources {
    threshold_factors: Vec<FactorSource>,
    threshold: u16,
    override_factors: Vec<FactorSource>,
}

struct MatrixOfFactorSources {
    primary_role: PrimaryRoleWithFactorSources,
    recovery_role: RecoveryRoleWithFactorSources,
    confirmation_role: ConfirmationRoleWithFactorSources,
}

struct SecurityStructureOfFactorSources {
    metadata: SecurityStructureMetadata,
    number_of_epochs_until_auto_confirmation: u64,
    matrix_of_factors: MatrixOfFactorSources
}