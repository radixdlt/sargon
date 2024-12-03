use sargon::AsShieldBuilderViolation;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, uniffi::Record)]
pub struct FactorSourceValidationStatus {
    pub role: RoleKind,
    pub factor_source_id: FactorSourceID,
    pub reason_if_invalid: Option<FactorSourceValidationStatusReasonIfInvalid>,
}

#[derive(Clone, Debug, PartialEq, uniffi::Enum)]
pub enum FactorSourceValidationStatusReasonIfInvalid {
    BasicViolation,
    NonBasic(SecurityShieldBuilderInvalidReason),
}

impl From<sargon::FactorSourceInRoleBuilderValidationStatus>
    for FactorSourceValidationStatus
{
    fn from(val: sargon::FactorSourceInRoleBuilderValidationStatus) -> Self {
        let reason_if_invalid: Option<
            FactorSourceValidationStatusReasonIfInvalid,
        > = {
            match val.validation {
                Ok(_) => None,
                Err(sargon::RoleBuilderValidation::BasicViolation(_)) => Some(
                    FactorSourceValidationStatusReasonIfInvalid::BasicViolation, // FactorSourceValidationStatusReasonIfInvalid::BasicViolation(
                                                                                 //     format!("{:?}", b),
                                                                                 // ),
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
