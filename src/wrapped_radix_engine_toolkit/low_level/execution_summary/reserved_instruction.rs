use crate::prelude::*;

use radix_engine_toolkit::transaction_types::ReservedInstruction as RetReservedInstruction;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum ReservedInstruction {
    AccountLockFee,
    AccountSecurify,
    IdentitySecurify,
    AccessControllerMethod,
}

impl From<RetReservedInstruction> for ReservedInstruction {
    fn from(value: RetReservedInstruction) -> Self {
        match value {
            RetReservedInstruction::AccountLockFee => Self::AccountLockFee,
            RetReservedInstruction::AccountSecurify => Self::AccountSecurify,
            RetReservedInstruction::IdentitySecurify => Self::IdentitySecurify,
            RetReservedInstruction::AccessControllerMethod => {
                Self::AccessControllerMethod
            }
        }
    }
}
