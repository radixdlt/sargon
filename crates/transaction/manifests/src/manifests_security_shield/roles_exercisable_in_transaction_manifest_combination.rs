use crate::prelude::*;

use radix_common::prelude::{
    ManifestEncode as ScryptoManifestEncode,
    ManifestSborTuple as ScryptoManifestSborTuple,
};
use radix_engine_interface::blueprints::access_controller::{
    AccessControllerInitiateRecoveryAsPrimaryInput as ScryptoAccessControllerInitiateRecoveryAsPrimaryInput,
    AccessControllerInitiateRecoveryAsRecoveryInput as ScryptoAccessControllerInitiateRecoveryAsRecoveryInput,
    AccessControllerQuickConfirmPrimaryRoleRecoveryProposalInput as ScryptoAccessControllerQuickConfirmPrimaryRoleRecoveryProposalInput,
    AccessControllerQuickConfirmRecoveryRoleRecoveryProposalInput as ScryptoAccessControllerQuickConfirmRecoveryRoleRecoveryProposalInput,
    ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT as SCRYPTO_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT,
    ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT as SCRYPTO_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
    ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT as SCRYPTO_ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT,
    ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT as SCRYPTO_ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
};

/// A "selector" of which combination of Roles we can exercise with used
/// to build different flavours of TransactionManifest for the Security Shield
/// update.
///
/// Each combination of roles allows us to skip signing with certain factors
/// and still be able to recover + confirm the AccessController update.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, enum_iterator::Sequence)]
pub enum RolesExercisableInTransactionManifestCombination {
    /// Initiates recovery using `Primary` role and quick confirms using
    /// `Recovery` role explicitly.
    InitiateWithPrimaryCompleteWithRecovery,

    /// Initiates recovery using `Primary` role and quick confirms using
    /// `Confirmation` role explicitly.
    InitiateWithPrimaryCompleteWithConfirmation,

    /// Initiates recovery using `Recovery` role and quick confirms using
    /// `Confirmation` role explicitly.
    InitiateWithRecoveryCompleteWithConfirmation,

    /// Initiate recovery with `Recovery` role and use timed confirmation.
    ///
    /// Since this roles combination does not explicitly use the Confirmation
    /// Role we will not include any confirm Instruction in the manifest
    /// we build for this kind. Instead, if this is the TransactionManifest which
    /// we will be submitting to the network, the host (user) will need to wait
    /// until the transaction is confirmed and then update Profile to keep
    /// track of the fact that the entity is in between states of recovery.
    /// TODO: TBD probably a new variant of the `ProvisionalSecurifiedConfig`
    /// `WaitingForTimedRecovery(SecurityStructureOfFactorInstances)` or similar.
    ///
    /// Host will also need to schedule a notification for user so that host
    /// can call the `confirm_timed_recovery` method after the time has elapsed.
    InitiateWithRecoveryDelayedCompletion,

    /// ‼️ REQUIRES "Dugong" ‼️
    /// Initiate recovery with `Primary` role and use timed confirmation.
    ///
    /// Since this roles combination does not explicitly use the Confirmation
    /// Role we will not include any confirm Instruction in the manifest
    /// we build for this kind. Instead, if this is the TransactionManifest which
    /// we will be submitting to the network, the host (user) will need to wait
    /// until the transaction is confirmed and then update Profile to keep
    /// track of the fact that the entity is in between states of recovery.
    /// TODO: TBD probably a new variant of the `ProvisionalSecurifiedConfig`
    /// `WaitingForTimedRecovery(SecurityStructureOfFactorInstances)` or similar.
    ///
    /// Host will also need to schedule a notification for user so that host
    /// can call the `confirm_timed_recovery` method after the time has elapsed.
    ///
    /// ‼️ REQUIRES "Dugong" ‼️
    InitiateWithPrimaryDelayedCompletion,
    // TODO:
    // FUTURE IMPROVEMENTS,
    // User can't initiate themselves and needs to send a request to an external source (e.g. a friend or custodian)
    // ExternalInitiateWithPrimary
    // ExternalInitiateWithRecovery
}

impl Default for RolesExercisableInTransactionManifestCombination {
    fn default() -> Self {
        // we default to a combination containing Primary since it allows
        // use to top up XRD vault of AccessController
        Self::InitiateWithPrimaryCompleteWithRecovery
    }
}

pub trait CallMethodInput:
    ScryptoManifestEncode + ScryptoManifestSborTuple
{
}
impl<T: ScryptoManifestEncode + ScryptoManifestSborTuple> CallMethodInput
    for T
{
}

impl RolesExercisableInTransactionManifestCombination {
    pub fn all() -> IndexSet<Self> {
        enum_iterator::all::<Self>().collect()
    }

    /// If this combination of roles references the `Primary` role or not.
    pub fn can_exercise_primary_role(&self) -> bool {
        matches!(
            self,
            Self::InitiateWithPrimaryCompleteWithRecovery
                | Self::InitiateWithPrimaryCompleteWithConfirmation
                | Self::InitiateWithPrimaryDelayedCompletion
        )
    }

    /// If we can set the ROLA key for this combination of roles.
    pub fn can_set_rola_key(&self) -> bool {
        self.can_exercise_primary_role()
    }

    fn initiate_recovery_with_primary(&self) -> bool {
        matches!(
            self,
            Self::InitiateWithPrimaryCompleteWithRecovery
                | Self::InitiateWithPrimaryCompleteWithConfirmation
                | Self::InitiateWithPrimaryDelayedCompletion
        )
    }
    fn initiate_recovery_with_recovery(&self) -> bool {
        matches!(
            self,
            Self::InitiateWithRecoveryCompleteWithConfirmation
                | Self::InitiateWithRecoveryDelayedCompletion
        )
    }

    fn quick_confirm_with_recovery(&self) -> bool {
        matches!(self, Self::InitiateWithPrimaryCompleteWithRecovery)
    }

    fn quick_confirm_with_confirmation(&self) -> bool {
        matches!(
            self,
            Self::InitiateWithPrimaryCompleteWithConfirmation
                | Self::InitiateWithRecoveryCompleteWithConfirmation
        )
    }

    /// Returns method identifier and input for **initiating** recovery
    /// on an AccessController - depending on which roles we can exercise.
    pub fn input_for_initialization(
        &self,
        factors_and_time: &AccessControllerFactorsAndTimeInput,
    ) -> (&'static str, Box<dyn CallMethodInput>) {
        if self.initiate_recovery_with_primary() {
            (
                SCRYPTO_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT,
                Box::new(
                    ScryptoAccessControllerInitiateRecoveryAsPrimaryInput::from(
                        factors_and_time,
                    ),
                ),
            )
        } else if self.initiate_recovery_with_recovery() {
            (
                SCRYPTO_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
                Box::new(
                    ScryptoAccessControllerInitiateRecoveryAsRecoveryInput::from(
                        factors_and_time,
                    ),
                ),
            )
        } else {
            unreachable!(
                "unable to calculate input_for_initialization - this is a programmer error"
            )
        }
    }

    /// Returns method identifier and input for **confirm** recovery
    /// on an AccessController - depending on which roles we can exercise.
    ///
    /// **MUST** use the analogous method which was used to initiate recovery.
    pub fn input_for_quick_confirm(
        &self,
        factors_and_time: &AccessControllerFactorsAndTimeInput,
    ) -> Option<(&'static str, Box<dyn CallMethodInput>)> {
        if self.quick_confirm_with_confirmation()
            || self.quick_confirm_with_recovery()
        {
            if self.initiate_recovery_with_primary() {
                Some((
                    SCRYPTO_ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT,
                    Box::new(ScryptoAccessControllerQuickConfirmPrimaryRoleRecoveryProposalInput::from(factors_and_time))
                ))
            } else if self.initiate_recovery_with_recovery() {
                Some((
                    SCRYPTO_ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
                    Box::new(ScryptoAccessControllerQuickConfirmRecoveryRoleRecoveryProposalInput::from(factors_and_time))
                ))
            } else {
                unreachable!(
                    "unable to calculate input_for_quick_confirm - this is a programmer error"
                )
            }
        } else {
            // Time based cannot happen yet - host (user) need to wait the specified
            // amount of time (factors_and_time.time) before calling this method.
            // So host will need to schedule a notification for user so that host
            // can call this method after the time has elapsed.
            None
        }
    }
}

pub trait TransactionManifestExplicitlyReferencesPrimaryRole {
    fn explicitly_references_primary_role(&self) -> bool;
}
impl TransactionManifestExplicitlyReferencesPrimaryRole
    for TransactionManifest
{
    fn explicitly_references_primary_role(&self) -> bool {
        let has = |identifier: &str| -> bool {
            self.instructions()
                .iter()
                .any(|instruction| matches!(instruction, ScryptoInstruction::CallMethod(method) if method.method_name == identifier))
        };

        has(SCRYPTO_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT)
    }
}
