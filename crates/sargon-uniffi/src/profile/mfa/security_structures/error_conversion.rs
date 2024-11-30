use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, uniffi::Error)]
pub enum CommonError {
    #[error("Sargon")]
    Sargon(String),

    #[error("AlreadyBuilt")]
    AlreadyBuilt,

    #[error("Matrix builder RwLock poisoned")]
    MatrixBuilderRwLockPoisoned,

    #[error("Build error {0}")]
    BuildError(String),
}

impl From<MatrixBuilderValidation> for CommonError {
    fn from(val: MatrixBuilderValidation) -> Self {
        CommonError::BuildError(format!("{:?}", val))
    }
}

impl From<RoleBuilderValidation> for CommonError {
    fn from(val: RoleBuilderValidation) -> Self {
        CommonError::BuildError(format!("{:?}", val))
    }
}
