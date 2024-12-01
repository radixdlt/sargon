use crate::prelude::*;

use sargon::MatrixOfFactorSources as InternalMatrixOfFactorSources;


#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct MatrixOfFactorSources {
    pub primary_role: PrimaryRoleWithFactorSources,
    pub recovery_role: RecoveryRoleWithFactorSources,
    pub confirmation_role: ConfirmationRoleWithFactorSources,

    pub number_of_days_until_auto_confirm: u16,
}

impl From<InternalMatrixOfFactorSources> for MatrixOfFactorSources {
    fn from(value: InternalMatrixOfFactorSources) -> Self {
        todo!()
    }
}
impl From<MatrixOfFactorSources> for InternalMatrixOfFactorSources {
    fn from(value: MatrixOfFactorSources) -> Self {
        todo!()
    }
}
