use crate::prelude::*;
use sargon::ReservedInstruction as InternalReservedInstruction;


/// The set of instructions that is only allowed in manifests created by the
/// wallet itself.
#[derive(Clone,  PartialEq, Eq, uniffi::Enum)]
pub enum ReservedInstruction {
    AccountLockFee,
    AccountSecurify,
    IdentitySecurify,
    AccessControllerMethod,
    AccountUpdateSettings,
}

impl From<InternalReservedInstruction> for ReservedInstruction {
    fn from(value: InternalReservedInstruction) -> Self {
        match value {
            InternalReservedInstruction::AccountLockFee => ReservedInstruction::AccountLockFee,
            InternalReservedInstruction::AccountSecurify => ReservedInstruction::AccountSecurify,
            InternalReservedInstruction::IdentitySecurify => ReservedInstruction::IdentitySecurify,
            InternalReservedInstruction::AccessControllerMethod => ReservedInstruction::AccessControllerMethod,
            InternalReservedInstruction::AccountUpdateSettings => ReservedInstruction::AccountUpdateSettings,
        }
    }
}

impl Into<InternalReservedInstruction> for ReservedInstruction {
    fn into(self) -> InternalReservedInstruction {
        match self {
            ReservedInstruction::AccountLockFee => InternalReservedInstruction::AccountLockFee,
            ReservedInstruction::AccountSecurify => InternalReservedInstruction::AccountSecurify,
            ReservedInstruction::IdentitySecurify => InternalReservedInstruction::IdentitySecurify,
            ReservedInstruction::AccessControllerMethod => InternalReservedInstruction::AccessControllerMethod,
            ReservedInstruction::AccountUpdateSettings => InternalReservedInstruction::AccountUpdateSettings,
        }
    }
}