#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Object)]
pub struct FactorSourceValidationStatus {
    pub role: sargon::RoleKind,
    pub factor_source_id: sargon::FactorSourceID,
    pub validation: rules::RoleBuilderMutateResult,
}

impl From<rules::FactorSourceInRoleBuilderValidationStatus>
    for FactorSourceValidationStatus
{
    fn from(val: rules::FactorSourceInRoleBuilderValidationStatus) -> Self {
        FactorSourceValidationStatus {
            role: val.role,
            factor_source_id: val.factor_source_id,
            validation: val.validation,
        }
    }
}
