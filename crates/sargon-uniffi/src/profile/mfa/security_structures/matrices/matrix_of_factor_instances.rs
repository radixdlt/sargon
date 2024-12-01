use crate::prelude::*;

use sargon::ConfirmationRoleWithFactorInstances as InternalConfirmationRoleWithFactorInstances;
use sargon::MatrixOfFactorInstances as InternalMatrixOfFactorInstances;
use sargon::PrimaryRoleWithFactorInstances as InternalPrimaryRoleWithFactorInstances;
use sargon::RecoveryRoleWithFactorInstances as InternalRecoveryRoleWithFactorInstances;

// #[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
// pub struct PrimaryRoleWithFactorInstances {
//     pub threshold: u8,
//     pub threshold_factors: Vec<FactorSourceID>,
//     pub override_factors: Vec<FactorSourceID>,
// }

// impl From<InternalPrimaryRoleWithFactorInstances>
//     for PrimaryRoleWithFactorInstances
// {
//     fn from(value: InternalPrimaryRoleWithFactorInstances) -> Self {
//         todo!()
//     }
// }
// impl From<PrimaryRoleWithFactorInstances>
//     for InternalPrimaryRoleWithFactorInstances
// {
//     fn from(value: PrimaryRoleWithFactorInstances) -> Self {
//         todo!()
//     }
// }

// #[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
// pub struct RecoveryRoleWithFactorInstances {
//     pub threshold: u8,
//     pub threshold_factors: Vec<FactorSourceID>,
//     pub override_factors: Vec<FactorSourceID>,
// }

// impl From<InternalRecoveryRoleWithFactorInstances>
//     for RecoveryRoleWithFactorInstances
// {
//     fn from(value: InternalRecoveryRoleWithFactorInstances) -> Self {
//         todo!()
//     }
// }
// impl From<RecoveryRoleWithFactorInstances>
//     for InternalRecoveryRoleWithFactorInstances
// {
//     fn from(value: RecoveryRoleWithFactorInstances) -> Self {
//         todo!()
//     }
// }

// #[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
// pub struct ConfirmationRoleWithFactorInstances {
//     pub threshold: u8,
//     pub threshold_factors: Vec<FactorSourceID>,
//     pub override_factors: Vec<FactorSourceID>,
// }

// impl From<InternalConfirmationRoleWithFactorInstances>
//     for ConfirmationRoleWithFactorInstances
// {
//     fn from(value: InternalConfirmationRoleWithFactorInstances) -> Self {
//         todo!()
//     }
// }
// impl From<ConfirmationRoleWithFactorInstances>
//     for InternalConfirmationRoleWithFactorInstances
// {
//     fn from(value: ConfirmationRoleWithFactorInstances) -> Self {
//         todo!()
//     }
// }

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct MatrixOfFactorInstances {
    pub primary_role: PrimaryRoleWithFactorInstances,
    pub recovery_role: RecoveryRoleWithFactorInstances,
    pub confirmation_role: ConfirmationRoleWithFactorInstances,

    pub number_of_days_until_auto_confirm: u16,
}

impl From<InternalMatrixOfFactorInstances> for MatrixOfFactorInstances {
    fn from(value: InternalMatrixOfFactorInstances) -> Self {
        todo!()
    }
}
impl From<MatrixOfFactorInstances> for InternalMatrixOfFactorInstances {
    fn from(value: MatrixOfFactorInstances) -> Self {
        todo!()
    }
}
