use sargon::{MatrixBuilderValidation, RoleBuilderValidation};

use crate::prelude::*;

impl From<MatrixBuilderValidation> for CommonError {
    fn from(_value: MatrixBuilderValidation) -> Self {
        // CommonError::BuildError(format!("{:?}", val))
        CommonError::Unknown
    }
}

impl From<RoleBuilderValidation> for CommonError {
    fn from(_value: RoleBuilderValidation) -> Self {
        // CommonError::BuildError(format!("{:?}", val))
        CommonError::Unknown
    }
}
