use crate::prelude::*;

/// The set of instructions that is only allowed in manifests created by the
/// wallet itself.
#[derive(Clone, Debug, PartialEq, Eq, Hash, derive_more::Display)]
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

impl ReservedInstruction {
    pub fn from_ret_reserved_instructions_output(
        ret: RetReservedInstructionsOutput,
    ) -> IndexSet<Self> {
        let mut result = IndexSet::new();

        if ret.has_account_lock_fee_invocations() {
            result.insert(Self::AccountLockFee);
        }

        if ret.has_account_securify_invocations() {
            result.insert(Self::AccountSecurify);
        }

        if ret.has_identity_securify_invocations() {
            result.insert(Self::IdentitySecurify);
        }

        if ret.has_access_controller_invocations() {
            result.insert(Self::AccessControllerMethod);
        }

        if ret.has_account_lock_owner_keys_metadata_field_invocations() {
            result.insert(Self::AccountLockOwnerKeysMetadataField);
        }

        if ret.has_account_update_owner_keys_metadata_field_invocations() {
            result.insert(Self::AccountUpdateOwnerKeysMetadataField);
        }

        if ret.has_identity_lock_owner_keys_metadata_field_invocations() {
            result.insert(Self::IdentityLockOwnerKeysMetadataField);
        }

        if ret.has_identity_update_owner_keys_metadata_field_invocations() {
            result.insert(Self::IdentityUpdateOwnerKeysMetadataField);
        }

        result
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
    use radix_common::prelude::ManifestNamedAddress;

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
        let ret = RetReservedInstructionsOutput {
            account_lock_fee_invocations: IndexSet::from([
                ScryptoManifestGlobalAddress::Named(ManifestNamedAddress(0)),
            ]),
            account_securify_invocations: IndexSet::from([
                ScryptoManifestGlobalAddress::Named(ManifestNamedAddress(0)),
            ]),
            identity_securify_invocations: IndexSet::from([
                ScryptoManifestGlobalAddress::Named(ManifestNamedAddress(0)),
            ]),
            access_controller_invocations: IndexSet::from([
                ScryptoManifestGlobalAddress::Named(ManifestNamedAddress(0)),
            ]),
            account_lock_owner_keys_metadata_field_invocations: IndexSet::from(
                [ScryptoManifestGlobalAddress::Named(ManifestNamedAddress(0))],
            ),
            account_update_owner_keys_metadata_field_invocations:
                IndexSet::from([ScryptoManifestGlobalAddress::Named(
                    ManifestNamedAddress(0),
                )]),
            identity_lock_owner_keys_metadata_field_invocations: IndexSet::from(
                [ScryptoManifestGlobalAddress::Named(ManifestNamedAddress(0))],
            ),
            identity_update_owner_keys_metadata_field_invocations:
                IndexSet::from([ScryptoManifestGlobalAddress::Named(
                    ManifestNamedAddress(0),
                )]),
        };

        let reserved_instructions =
            SUT::from_ret_reserved_instructions_output(ret);

        assert_eq!(
            reserved_instructions,
            IndexSet::from([
                ReservedInstruction::AccountLockFee,
                ReservedInstruction::AccountSecurify,
                ReservedInstruction::IdentitySecurify,
                ReservedInstruction::AccessControllerMethod,
                ReservedInstruction::AccountLockOwnerKeysMetadataField,
                ReservedInstruction::AccountUpdateOwnerKeysMetadataField,
                ReservedInstruction::IdentityLockOwnerKeysMetadataField,
                ReservedInstruction::IdentityUpdateOwnerKeysMetadataField,
            ])
        );
    }
}
