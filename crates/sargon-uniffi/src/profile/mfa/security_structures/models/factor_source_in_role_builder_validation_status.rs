use sargon::AsShieldBuilderViolation;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, uniffi::Object)]
pub struct FactorSourceValidationStatus {
    pub role: RoleKind,
    pub factor_source_id: FactorSourceID,
    pub reason_if_invalid: Option<FactorSourceValidationStatusReasonIfInvalid>,
}

#[derive(Clone, Debug, PartialEq, uniffi::Enum)]
pub enum FactorSourceValidationStatusReasonIfInvalid {
    BasicViolation(String),
    NonBasic(SecurityShieldBuilderInvalidReason),
}

// #[uniffi::export]
// impl FactorSourceValidationStatus {
//     pub fn reason_if_invalid(
//         &self,
//     ) -> Option<FactorSourceValidationStatusReasonIfInvalid> {
//         self.reason_if_invalid.clone()
//     }

//     pub fn role(&self) -> RoleKind {
//         self.role
//     }

//     pub fn factor_source_id(&self) -> FactorSourceID {
//         self.factor_source_id.clone()
//     }
// }
impl From<sargon::FactorSourceInRoleBuilderValidationStatus>
    for FactorSourceValidationStatus
{
    fn from(val: sargon::FactorSourceInRoleBuilderValidationStatus) -> Self {
        let reason_if_invalid: Option<
            FactorSourceValidationStatusReasonIfInvalid,
        > = {
            match val.validation {
                Ok(_) => None,
                Err(sargon::RoleBuilderValidation::BasicViolation(b)) => Some(
                    FactorSourceValidationStatusReasonIfInvalid::BasicViolation(
                        format!("{:?}", b),
                    ),
                ),
                Err(sargon::RoleBuilderValidation::ForeverInvalid(v)) => v
                    .as_shield_validation()
                    .map(SecurityShieldBuilderInvalidReason::from)
                    .map(|x| {
                        FactorSourceValidationStatusReasonIfInvalid::NonBasic(x)
                    }),
                Err(sargon::RoleBuilderValidation::NotYetValid(v)) => (
                    val.role, v,
                )
                    .as_shield_validation()
                    .map(SecurityShieldBuilderInvalidReason::from)
                    .map(|x| {
                        FactorSourceValidationStatusReasonIfInvalid::NonBasic(x)
                    }),
            }
        };
        FactorSourceValidationStatus {
            role: val.role.into(),
            factor_source_id: val.factor_source_id.into(),
            reason_if_invalid,
        }
    }
}
