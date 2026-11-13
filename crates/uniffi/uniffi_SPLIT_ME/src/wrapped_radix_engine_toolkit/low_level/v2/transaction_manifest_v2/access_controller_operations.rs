use crate::prelude::*;
use manifests::AccessControllerOperation as InternalAccessControllerOperation;
use manifests::AccessControllerOperationKind as InternalAccessControllerOperationKind;
use manifests::AccessControllerOperationsAnalysis;
use manifests::AccessControllerRoleRule as InternalAccessControllerRoleRule;
use manifests::AccessControllerRuleSetProposal as InternalAccessControllerRuleSetProposal;

/// A simplified view of one AccessController role rule, for showing a
/// proposed rule set to the user.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum AccessControllerRoleRule {
    /// Anyone can act in this role.
    AllowAll,

    /// Nobody can act in this role.
    DenyAll,

    /// At least `threshold` of `badges` must be proven.
    Threshold {
        threshold: u8,
        badges: Vec<ResourceOrNonFungible>,
    },

    /// A rule this summary cannot express. Hosts should flag it to the user.
    Complex,
}

/// The rule set carried by a recovery-family call.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AccessControllerRuleSetProposal {
    pub primary: AccessControllerRoleRule,
    pub recovery: AccessControllerRoleRule,
    pub confirmation: AccessControllerRoleRule,
    pub timed_recovery_delay_in_minutes: Option<u32>,
}

/// What one AccessController call in a manifest does.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum AccessControllerOperationKind {
    CreateProof,
    InitiateRecoveryAsPrimary {
        proposal: AccessControllerRuleSetProposal,
    },
    InitiateRecoveryAsRecovery {
        proposal: AccessControllerRuleSetProposal,
    },
    QuickConfirmPrimaryRoleRecoveryProposal {
        proposal: AccessControllerRuleSetProposal,
    },
    QuickConfirmRecoveryRoleRecoveryProposal {
        proposal: AccessControllerRuleSetProposal,
    },
    TimedConfirmRecovery {
        proposal: AccessControllerRuleSetProposal,
    },
    StopTimedRecovery {
        proposal: AccessControllerRuleSetProposal,
    },
    CancelPrimaryRoleRecoveryProposal,
    CancelRecoveryRoleRecoveryProposal,
    LockPrimaryRole,
    UnlockPrimaryRole,
    Unsupported {
        method_name: String,
    },
}

/// One AccessController call in a manifest.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AccessControllerOperation {
    pub access_controller: AccessControllerAddress,
    pub kind: AccessControllerOperationKind,
}

/// Every call on a statically addressed AccessController in `manifest`, in
/// manifest order, decoded for review: what each call does and, for recovery
/// calls, the rule set it proposes or acts on.
///
/// Hosts show this when a pre-authorization operates an external
/// AccessController, so the user sees e.g. which signers a rotation proposes.
#[uniffi::export]
pub fn subintent_manifest_access_controller_operations(
    manifest: &SubintentManifest,
) -> Vec<AccessControllerOperation> {
    manifest
        .into_internal()
        .access_controller_operations()
        .into_type()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_lock_primary_role() {
        let ac = sargon::AccessControllerAddress::sample_mainnet();
        let internal = sargon::SubintentManifest::new(
            format!(
                "CALL_METHOD Address(\"{}\") \"lock_primary_role\";\nYIELD_TO_PARENT;\n",
                ac
            ),
            sargon::NetworkID::Mainnet,
            sargon::Blobs::default(),
            sargon::ChildSubintentSpecifiers::default(),
        )
        .unwrap();
        let manifest = SubintentManifest::from(internal);

        let operations: Vec<InternalAccessControllerOperation> =
            subintent_manifest_access_controller_operations(&manifest)
                .into_iter()
                .map(|op| op.into_internal())
                .collect();
        assert_eq!(
            operations,
            vec![InternalAccessControllerOperation {
                access_controller: ac,
                kind: InternalAccessControllerOperationKind::LockPrimaryRole,
            }]
        );
    }

    #[test]
    fn role_rule_round_trips() {
        let internal = InternalAccessControllerRoleRule::Threshold {
            threshold: 2,
            badges: vec![
                sargon::ResourceOrNonFungible::sample(),
                sargon::ResourceOrNonFungible::sample_other(),
            ],
        };
        let ffi = AccessControllerRoleRule::from(internal.clone());
        assert_eq!(ffi.into_internal(), internal);
    }

    #[test]
    fn proposal_round_trips() {
        let internal = InternalAccessControllerRuleSetProposal {
            primary: InternalAccessControllerRoleRule::AllowAll,
            recovery: InternalAccessControllerRoleRule::DenyAll,
            confirmation: InternalAccessControllerRoleRule::Complex,
            timed_recovery_delay_in_minutes: Some(20160),
        };
        let operation = InternalAccessControllerOperation {
            access_controller: sargon::AccessControllerAddress::sample(),
            kind: InternalAccessControllerOperationKind::StopTimedRecovery {
                proposal: internal,
            },
        };
        let ffi = AccessControllerOperation::from(operation.clone());
        assert_eq!(ffi.into_internal(), operation);
    }
}
