use sargon::{MatrixBuilderValidation, RoleBuilderValidation};

use crate::prelude::*;

impl From<MatrixBuilderValidation> for CommonError {
    fn from(val: MatrixBuilderValidation) -> Self {
        // CommonError::BuildError(format!("{:?}", val))
        CommonError::Unknown
    }
}

impl From<RoleBuilderValidation> for CommonError {
    fn from(val: RoleBuilderValidation) -> Self {
        // CommonError::BuildError(format!("{:?}", val))
        CommonError::Unknown
    }
}
