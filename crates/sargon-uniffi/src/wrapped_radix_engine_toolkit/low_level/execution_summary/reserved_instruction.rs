use crate::prelude::*;
use sargon::ReservedInstruction as InternalReservedInstruction;

/// The set of instructions that is only allowed in manifests created by the
/// wallet itself.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum ReservedInstruction {
    AccountLockFee,
    AccountSecurify,
    IdentitySecurify,
    AccessControllerMethod,
}
