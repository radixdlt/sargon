use crate::prelude::*;

use enum_as_inner::EnumAsInner;
use profile_supporting_types::AnySecurifiedEntity;
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
    /// Initiates recovery using `Recovery` role and quick confirms using
    /// `Primary` role explicitly.
    InitiateWithRecoveryCompleteWithPrimary,

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

    /// Initiates recovery using `Primary` role and quick confirms using
    /// `Confirmation` role explicitly.
    InitiateWithPrimaryCompleteWithConfirmation,

    /// ‼️ Requires future network upgrade "Dugong"
    ///
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
    /// ‼️ Requires future network upgrade "Dugong"
    InitiateWithPrimaryDelayedCompletion,
    // TODO:
    // FUTURE IMPROVEMENTS,
    // User can't initiate themselves and needs to send a request to an external source (e.g. a friend or custodian)
    // ExternalInitiateWithPrimary
    // ExternalInitiateWithRecovery
}

pub trait CallMethodInput:
    ScryptoManifestEncode + ScryptoManifestSborTuple
{
}
impl<T: ScryptoManifestEncode + ScryptoManifestSborTuple> CallMethodInput
    for T
{
}

/// Depending on the role used to initiate recovery, and if we are quick confirming
/// the recovery proposal we need to either place SET_METADATA instruction which
/// updates the ROLA key with the new factors before the recovery proposal - requiring
/// the OLD PrimaryRole factors to auth the transaction - or we can quick confirm
/// we will place the SET_METADATA instruction after we have (quick) confirmed the
/// recovery proposal.
#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner)]
pub enum OrderOfInstructionSettingRolaKey {
    /// Place SET_METADATA instruction before initiating recovery - so that
    /// we can auth with the OLD factors of Primary role.
    BeforeInitRecovery,
    /// Place SET_METADATA instruction after quick confirming the recovery proposal
    /// => requiring us to auth with the NEW factors of Primary role.
    AfterQuickConfirm,

    /// No need to set ROLA key
    NotNeeded,

    /// Cannot set ROLA key in this TX - must do it in a future TX when we confirm recovery
    MustSetInFutureTxForConfirmRecovery,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum RoleInitiatingRecovery {
    Primary,
    Recovery,
}

impl RolesExercisableInTransactionManifestCombination {
    pub fn manifest_end_user_gets_to_preview() -> Self {
        Self::InitiateWithRecoveryCompleteWithConfirmation
    }

    pub fn all() -> IndexSet<Self> {
        enum_iterator::all::<Self>().collect()
    }

    pub fn order_of_instruction_setting_rola(
        &self,
        security_structure_of_factor_instances: &SecurityStructureOfFactorInstances,
        entity: &AnySecurifiedEntity,
    ) -> OrderOfInstructionSettingRolaKey {
        if security_structure_of_factor_instances
            .authentication_signing_factor_instance
            == entity.current_authentication_signing_factor_instance()
        {
            // The factor of ROLA key is the same as the current factor of the entity
            // => we can skip setting the ROLA key.
            return OrderOfInstructionSettingRolaKey::NotNeeded;
        }

        if !self.can_quick_confirm() {
            // N.B.
            // In case of role initiating the recovery being `Primary` we CAN in fact
            // set the ROLA key in the same transaction as we initiate recovery, however,
            // if we CANCEL the recovery proposal (instead of CONFIRM) in the future
            // transaction we would be in a bad state (not using the old nor the new shield).
            return OrderOfInstructionSettingRolaKey::MustSetInFutureTxForConfirmRecovery;
        }

        match self.role_initiating_recovery() {
            RoleInitiatingRecovery::Primary => {
                // We can exercise the Primary Role => no need
                // to set the ROLA key after recovery and use new factors, instead
                // we can use existing factors
                OrderOfInstructionSettingRolaKey::BeforeInitRecovery
            }
            RoleInitiatingRecovery::Recovery => {
                // we can quick confirm so we can always set the ROLA key using
                // the NEW factors - disregarding of which role initiated recovery.
                OrderOfInstructionSettingRolaKey::AfterQuickConfirm
            }
        }
    }

    pub fn can_exercise_primary_role(&self) -> bool {
        match self {
            Self::InitiateWithPrimaryCompleteWithConfirmation
            | Self::InitiateWithPrimaryDelayedCompletion
            | Self::InitiateWithRecoveryCompleteWithPrimary => true,
            Self::InitiateWithRecoveryCompleteWithConfirmation
            | Self::InitiateWithRecoveryDelayedCompletion => false,
        }
    }

    fn role_initiating_recovery(&self) -> RoleInitiatingRecovery {
        match self {
            Self::InitiateWithPrimaryCompleteWithConfirmation
            | Self::InitiateWithPrimaryDelayedCompletion => {
                RoleInitiatingRecovery::Primary
            }
            Self::InitiateWithRecoveryCompleteWithPrimary
            | Self::InitiateWithRecoveryCompleteWithConfirmation
            | Self::InitiateWithRecoveryDelayedCompletion => {
                RoleInitiatingRecovery::Recovery
            }
        }
    }

    pub fn can_quick_confirm(&self) -> bool {
        match self {
            Self::InitiateWithPrimaryCompleteWithConfirmation
            | Self::InitiateWithRecoveryCompleteWithConfirmation
            | Self::InitiateWithRecoveryCompleteWithPrimary => true,
            Self::InitiateWithRecoveryDelayedCompletion
            | Self::InitiateWithPrimaryDelayedCompletion => false,
        }
    }

    /// Returns method identifier and input for **initiating** recovery
    /// on an AccessController - depending on which roles we can exercise.
    pub fn input_for_initialization(
        &self,
        factors_and_time: &AccessControllerFactorsAndTimeInput,
    ) -> (&'static str, Box<dyn CallMethodInput>) {
        match self.role_initiating_recovery() {
            RoleInitiatingRecovery::Primary => {
                (
                    SCRYPTO_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT,
                    Box::new(
                        ScryptoAccessControllerInitiateRecoveryAsPrimaryInput::from(
                            factors_and_time,
                        ),
                    ),
                )
            }
            RoleInitiatingRecovery::Recovery => {
                (
                    SCRYPTO_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
                    Box::new(
                        ScryptoAccessControllerInitiateRecoveryAsRecoveryInput::from(
                            factors_and_time,
                        ),
                    ),
                )
            }
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
        if self.can_quick_confirm() {
            match self.role_initiating_recovery() {
                RoleInitiatingRecovery::Primary => Some((
                    SCRYPTO_ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT,
                    Box::new(ScryptoAccessControllerQuickConfirmPrimaryRoleRecoveryProposalInput::from(factors_and_time))
                )),
                RoleInitiatingRecovery::Recovery => Some((
                    SCRYPTO_ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
                    Box::new(ScryptoAccessControllerQuickConfirmRecoveryRoleRecoveryProposalInput::from(factors_and_time))
                ))
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
