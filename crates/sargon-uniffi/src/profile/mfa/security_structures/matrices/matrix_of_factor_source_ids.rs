use crate::prelude::*;

use sargon::ConfirmationRoleWithFactorSourceIds as InternalConfirmationRoleWithFactorSourceIds;
use sargon::MatrixOfFactorSourceIds as InternalMatrixOfFactorSourceIds;
use sargon::PrimaryRoleWithFactorSourceIds as InternalPrimaryRoleWithFactorSourceIds;
use sargon::RecoveryRoleWithFactorSourceIds as InternalRecoveryRoleWithFactorSourceIds;

// #[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
// pub struct PrimaryRoleWithFactorSourceIds {
//     pub threshold: u8,
//     pub threshold_factors: Vec<FactorSourceID>,
//     pub override_factors: Vec<FactorSourceID>,
// }

// impl From<InternalPrimaryRoleWithFactorSourceIds>
//     for PrimaryRoleWithFactorSourceIds
// {
//     fn from(value: InternalPrimaryRoleWithFactorSourceIds) -> Self {
//         todo!()
//     }
// }
// impl From<PrimaryRoleWithFactorSourceIds>
//     for InternalPrimaryRoleWithFactorSourceIds
// {
//     fn from(value: PrimaryRoleWithFactorSourceIds) -> Self {
//         todo!()
//     }
// }

// #[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
// pub struct RecoveryRoleWithFactorSourceIds {
//     pub threshold: u8,
//     pub threshold_factors: Vec<FactorSourceID>,
//     pub override_factors: Vec<FactorSourceID>,
// }

// impl From<InternalRecoveryRoleWithFactorSourceIds>
//     for RecoveryRoleWithFactorSourceIds
// {
//     fn from(value: InternalRecoveryRoleWithFactorSourceIds) -> Self {
//         todo!()
//     }
// }
// impl From<RecoveryRoleWithFactorSourceIds>
//     for InternalRecoveryRoleWithFactorSourceIds
// {
//     fn from(value: RecoveryRoleWithFactorSourceIds) -> Self {
//         todo!()
//     }
// }

// #[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
// pub struct ConfirmationRoleWithFactorSourceIds {
//     pub threshold: u8,
//     pub threshold_factors: Vec<FactorSourceID>,
//     pub override_factors: Vec<FactorSourceID>,
// }

// impl From<InternalConfirmationRoleWithFactorSourceIds>
//     for ConfirmationRoleWithFactorSourceIds
// {
//     fn from(value: InternalConfirmationRoleWithFactorSourceIds) -> Self {
//         todo!()
//     }
// }
// impl From<ConfirmationRoleWithFactorSourceIds>
//     for InternalConfirmationRoleWithFactorSourceIds
// {
//     fn from(value: ConfirmationRoleWithFactorSourceIds) -> Self {
//         todo!()
//     }
// }

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct MatrixOfFactorSourceIds {
    pub primary_role: PrimaryRoleWithFactorSourceIDs,
    pub recovery_role: RecoveryRoleWithFactorSourceIDs,
    pub confirmation_role: ConfirmationRoleWithFactorSourceIDs,

    pub number_of_days_until_auto_confirm: u16,
}

impl From<InternalMatrixOfFactorSourceIds> for MatrixOfFactorSourceIds {
    fn from(value: InternalMatrixOfFactorSourceIds) -> Self {
        todo!()
    }
}
impl From<MatrixOfFactorSourceIds> for InternalMatrixOfFactorSourceIds {
    fn from(value: MatrixOfFactorSourceIds) -> Self {
        unsafe {
            Self::unbuilt_with_roles_and_days(
                value.primary_role.into(),
                value.recovery_role.into(),
                value.confirmation_role.into(),
                value.number_of_days_until_auto_confirm,
            )
        }
    }
}
