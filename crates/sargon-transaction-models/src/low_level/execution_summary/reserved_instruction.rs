use crate::prelude::*;

/// The set of instructions that is only allowed in manifests created by the
/// wallet itself.
#[derive(Clone, Debug, PartialEq, Eq, derive_more::Display)]
pub enum ReservedInstruction {
    AccountLockFee,
    AccountSecurify,
    IdentitySecurify,
    AccessControllerMethod,
    AccountLockOwnerKeysMetadataField,
    AccountUpdateOwnerKeysMetadataField,
    IdentityLockOwnerKeysMetadataField,
    IdentityUpdateOwnerKeysMetadataField,
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
            RetReservedInstruction::AccountLockOwnerKeysMetadataField => {
                Self::AccountLockOwnerKeysMetadataField
            }
            RetReservedInstruction::AccountUpdateOwnerKeysMetadataField => {
                Self::AccountUpdateOwnerKeysMetadataField
            }
            RetReservedInstruction::IdentityLockOwnerKeysMetadataField => {
                Self::IdentityLockOwnerKeysMetadataField
            }
            RetReservedInstruction::IdentityUpdateOwnerKeysMetadataField => {
                Self::IdentityUpdateOwnerKeysMetadataField
            }
        }
    }
}

impl HasSampleValues for ReservedInstruction {
    fn sample() -> Self {
        Self::AccountLockFee
    }

    fn sample_other() -> Self {
        Self::AccountSecurify
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ReservedInstruction;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn from_ret() {
        assert_eq!(
            SUT::from(RetReservedInstruction::AccessControllerMethod),
            SUT::AccessControllerMethod
        );
        assert_eq!(
            SUT::from(RetReservedInstruction::AccountLockFee),
            SUT::AccountLockFee
        );
        assert_eq!(
            SUT::from(RetReservedInstruction::AccountSecurify),
            SUT::AccountSecurify
        );
        assert_eq!(
            SUT::from(RetReservedInstruction::IdentitySecurify),
            SUT::IdentitySecurify
        );
    }
}
