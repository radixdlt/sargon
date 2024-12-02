use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Object)]
pub struct FactorSourceValidationStatus {
    pub role: RoleKind,
    pub factor_source_id: FactorSourceID,
    pub validation: sargon::RoleBuilderMutateResult,
}

#[uniffi::export]
impl FactorSourceValidationStatus {
    pub fn validation_err(&self) -> Option<CommonError> {
        if let Err(e) = self
            .validation
            .map_err(|e| Into::<CommonError>::into((self.role, e)))
        {
            Some(e)
        } else {
            None
        }
    }

    pub fn role(&self) -> RoleKind {
        self.role
    }

    pub fn factor_source_id(&self) -> FactorSourceID {
        self.factor_source_id.clone()
    }
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
