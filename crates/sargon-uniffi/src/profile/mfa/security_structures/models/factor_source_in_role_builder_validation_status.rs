use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Object)]
pub struct FactorSourceValidationStatus {
    pub role: RoleKind,
    pub factor_source_id: FactorSourceID,
    pub validation: sargon::RoleBuilderMutateResult,
}

impl From<sargon::FactorSourceInRoleBuilderValidationStatus>
    for FactorSourceValidationStatus
{
    fn from(val: sargon::FactorSourceInRoleBuilderValidationStatus) -> Self {
        FactorSourceValidationStatus {
            role: val.role.into(),
            factor_source_id: val.factor_source_id.into(),
            validation: val.validation,
        }
    }
}
