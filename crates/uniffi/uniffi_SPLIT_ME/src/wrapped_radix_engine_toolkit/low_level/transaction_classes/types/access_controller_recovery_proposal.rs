use crate::prelude::*;
use sargon::AccessControllerRecoveryProposal as InternalAccessControllerRecoveryProposal;
use sargon::RecoveryProposer as InternalRecoveryProposer;

/// The role that initiated an access controller recovery proposal.
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
)]
pub enum RecoveryProposer {
    Primary,
    Recovery,
}

/// A recovery proposed on an access controller in a manifest.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AccessControllerRecoveryProposal {
    pub access_controller: AccessControllerAddress,

    /// The role that initiated the recovery.
    pub proposer: RecoveryProposer,

    /// The rules that replace the access controller's current ones once the
    /// recovery is confirmed, with the user's keys shown as their factor
    /// sources.
    pub proposed_rules: AccessControllerRuleSetForDisplay,

    /// The proposed time after which a recovery can be confirmed without the
    /// confirmation role, if any.
    pub timed_recovery_delay_in_minutes: Option<u32>,
}

decl_conversion_tests_for!(AccessControllerRecoveryProposal);
