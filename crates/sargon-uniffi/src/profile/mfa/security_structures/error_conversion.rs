use sargon::{MatrixBuilderValidation, RoleBuilderValidation};

use crate::prelude::*;

impl From<MatrixBuilderValidation> for CommonError {
    fn from(value: MatrixBuilderValidation) -> Self {
        let sargon_err = Into::<sargon::CommonError>::into(value);
        Into::<CommonError>::into(sargon_err)
    }
}

impl From<(RoleKind, sargon::RoleBuilderValidation)> for CommonError {
    fn from(value: (RoleKind, RoleBuilderValidation)) -> Self {
        let (role, violation) = value;
        let role = Into::<sargon::RoleKind>::into(role);
        let sargon_err = Into::<sargon::CommonError>::into((role, violation));
        Into::<CommonError>::into(sargon_err)
    }
}
