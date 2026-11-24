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

    /// The delay the proposal names. The engine doesn't apply it: show
    /// `current_timed_recovery_delay_in_minutes` instead.
    pub timed_recovery_delay_in_minutes: Option<u32>,

    /// The access controller's current timed recovery delay, which it keeps
    /// under the new rules: the time after which a recovery role proposal
    /// can be confirmed without the confirmation role, if known.
    pub current_timed_recovery_delay_in_minutes: Option<u32>,

    /// The account or persona the access controller controls, if known.
    pub controlled_entity: Option<AddressOfAccountOrPersona>,
}

decl_conversion_tests_for!(AccessControllerRecoveryProposal);
