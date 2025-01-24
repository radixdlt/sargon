use crate::prelude::*;

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
#[derive(Clone, Debug, PartialEq, Eq, Hash, enum_iterator::Sequence)]
pub enum TransactionManifestApplySecurityShieldKind {
    /// (Primary Recovery Confirmation)
    PrimaryAndRecoveryWithExplicitConfirmation,

    /// (Primary Recovery Time)
    PrimaryAndRecoveryWithTimedAutoConfirm,

    /// (Primary Confirmation)
    PrimaryAndExplicitConfirmation,

    /// (Primary Time) ‼️ REQUIRES "Dugong" ‼️
    PrimaryWithTimedAutoConfirm,

    /// (Recovery Confirmation)
    RecoveryAndExplicitConfirmation,

    /// (Recovery Time)
    RecoveryWithTimedAutoConfirm,
}

use radix_common::prelude::ManifestEncode;
use radix_common::prelude::ManifestSborTuple;
// Trickery to allow `Box<dyn ResolvableArguments>` - which is not allowed it seems,
// but this solves it, since in Scrypto they impl ResolvableArguments for ManifestEncode + ManifestSborTuple
pub trait CallMethodInput: ManifestEncode + ManifestSborTuple {}
impl<T: ManifestEncode + ManifestSborTuple> CallMethodInput for T {}

impl TransactionManifestApplySecurityShieldKind {
    pub fn all() -> IndexSet<Self> {
        enum_iterator::all::<Self>().collect()
    }

    /// If this combination of roles references the `Primary` role or not.
    fn can_exercise_primary_role(&self) -> bool {
        match self {
            Self::PrimaryAndRecoveryWithExplicitConfirmation => true,
            Self::PrimaryAndRecoveryWithTimedAutoConfirm => true,
            Self::PrimaryAndExplicitConfirmation => true,
            Self::PrimaryWithTimedAutoConfirm => true,
            Self::RecoveryAndExplicitConfirmation => false,
            Self::RecoveryWithTimedAutoConfirm => false,
        }
    }

    /// If this combination of roles references the `Recovery` role or not.
    fn can_exercise_recovery_role(&self) -> bool {
        match self {
            Self::PrimaryAndRecoveryWithExplicitConfirmation => true,
            Self::PrimaryAndRecoveryWithTimedAutoConfirm => true,
            Self::PrimaryAndExplicitConfirmation => false,
            Self::PrimaryWithTimedAutoConfirm => false,
            Self::RecoveryAndExplicitConfirmation => true,
            Self::RecoveryWithTimedAutoConfirm => true,
        }
    }

    /// If this combination of roles references the `Confirmation` role or not.
    ///
    /// Explicitly means "not using time, but use quick confirmation"
    fn can_exercise_confirmation_role_explicitly(&self) -> bool {
        match self {
            Self::PrimaryAndRecoveryWithExplicitConfirmation => true,
            Self::PrimaryAndRecoveryWithTimedAutoConfirm => false,
            Self::PrimaryAndExplicitConfirmation => true,
            Self::PrimaryWithTimedAutoConfirm => false,
            Self::RecoveryAndExplicitConfirmation => true,
            Self::RecoveryWithTimedAutoConfirm => false,
        }
    }

    /// If we can set the ROLA key for this combination of roles.
    pub fn can_set_rola_key(&self) -> bool {
        self.can_exercise_primary_role()
    }

    /// Returns method identifier and input for **initiating** recovery
    /// on an AccessController - depending on which roles we can exercise.
    pub fn input_for_initialization(
        &self,
        factors_and_time: &AccessControllerFactorsAndTimeInput,
    ) -> (&'static str, Box<dyn CallMethodInput>) {
        if self.can_exercise_recovery_role() {
            (
                SCRYPTO_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_RECOVERY_IDENT,
                Box::new(
                    ScryptoAccessControllerInitiateRecoveryAsRecoveryInput::from(
                        factors_and_time,
                    ),
                ),
            )
        } else if self.can_exercise_primary_role() {
            (
                SCRYPTO_ACCESS_CONTROLLER_INITIATE_RECOVERY_AS_PRIMARY_IDENT,
                Box::new(
                    ScryptoAccessControllerInitiateRecoveryAsPrimaryInput::from(
                        factors_and_time,
                    ),
                ),
            )
        } else {
            unreachable!(
                "No kind exists which disallows for both primary and recovery"
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
        if self.can_exercise_confirmation_role_explicitly() {
            Some(if self.can_exercise_recovery_role() {
                (SCRYPTO_ACCESS_CONTROLLER_QUICK_CONFIRM_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT, Box::new(ScryptoAccessControllerQuickConfirmRecoveryRoleRecoveryProposalInput::from(factors_and_time)))
            } else {
                (SCRYPTO_ACCESS_CONTROLLER_QUICK_CONFIRM_PRIMARY_ROLE_RECOVERY_PROPOSAL_IDENT, Box::new(ScryptoAccessControllerQuickConfirmPrimaryRoleRecoveryProposalInput::from(factors_and_time)))
            })
        } else {
            // Time based cannot happen yet - host (user) need to wait the specified
            // amount of time (factors_and_time.time) before calling this method.
            // So host will need to schedule a notification for user so that host
            // can call this method after the time has elapsed.
            None
        }
    }
}
