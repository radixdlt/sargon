struct PrimaryRoleWithFactorSourceIDs {
    threshold_factors: Vec<FactorSourceID>,
    threshold: u16,
    override_factors: Vec<FactorSourceID>,
}

struct MatrixOfFactorSourceIDs {
    primary_role: PrimaryRoleWithFactorSourceIDs,
    recovery_role: RecoveryRoleWithFactorSourceIDs,
    confirmation_role: ConfirmationRoleWithFactorSourceIDs,
}

struct SecurityStructureOfFactorSourceIDs {
    metadata: SecurityStructureMetadata,
    number_of_epochs_until_auto_confirmation: u64,
    matrix_of_factors: MatrixOfFactorSourceIDs
}