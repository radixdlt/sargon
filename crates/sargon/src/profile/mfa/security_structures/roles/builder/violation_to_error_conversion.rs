use crate::prelude::*;

impl From<BasicViolation> for CommonError {
    fn from(_value: BasicViolation) -> Self {
        todo!()
    }
}
impl From<ForeverInvalidReason> for CommonError {
    fn from(_value: ForeverInvalidReason) -> Self {
        todo!()
    }
}
impl From<NotYetValidReason> for CommonError {
    fn from(_value: NotYetValidReason) -> Self {
        todo!()
    }
}
impl From<(RoleKind, RoleBuilderValidation)> for CommonError {
    fn from(value: (RoleKind, RoleBuilderValidation)) -> Self {
        let (_role, violation) = value;
        match violation {
            RoleBuilderValidation::BasicViolation(val) => val.into(),
            RoleBuilderValidation::ForeverInvalid(val) => val.into(),
            RoleBuilderValidation::NotYetValid(val) => val.into(),
        }
    }
}
