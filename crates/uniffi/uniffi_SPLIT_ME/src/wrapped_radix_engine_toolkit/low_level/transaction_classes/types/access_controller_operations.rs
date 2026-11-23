use crate::prelude::*;
use sargon::AccessControllerPrimaryRoleLockOperation as InternalAccessControllerPrimaryRoleLockOperation;
use sargon::AccessControllerRecoveryCancellation as InternalAccessControllerRecoveryCancellation;
use sargon::AccessControllerRecoveryConfirmation as InternalAccessControllerRecoveryConfirmation;
use sargon::AccessControllerTimedRecoveryProposal as InternalAccessControllerTimedRecoveryProposal;
use sargon::AccessControllerXrdTransfer as InternalAccessControllerXrdTransfer;
use sargon::AccountXrdTransfer as InternalAccountXrdTransfer;
use sargon::PrimaryRoleLockOperation as InternalPrimaryRoleLockOperation;
use sargon::RecoveryConfirmationKind as InternalRecoveryConfirmationKind;

/// A timed recovery started on an access controller: the recovery role
/// proposes rules without confirming them. Once the access controller's own
/// timed recovery delay has passed, anyone can confirm them.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AccessControllerTimedRecoveryProposal {
    pub access_controller: AccessControllerAddress,

    /// The rules that replace the access controller's current ones once the
    /// recovery is confirmed, with the user's keys shown as their factor
    /// sources.
    pub proposed_rules: AccessControllerRuleSetForDisplay,

    /// The delay the proposal names. The engine doesn't apply it: show
    /// `current_timed_recovery_delay_in_minutes` instead.
    pub timed_recovery_delay_in_minutes: Option<u32>,

    /// Whether the primary role is also locked, so that the entity can't be
    /// used until the recovery is confirmed or the role unlocked.
    pub locks_primary_role: bool,

    /// The access controller's current timed recovery delay, after which the
    /// recovery can be confirmed without the confirmation role, if known.
    pub current_timed_recovery_delay_in_minutes: Option<u32>,

    /// The account or persona the access controller controls, if known.
    pub controlled_entity: Option<AddressOfAccountOrPersona>,
}

/// How a recovery is confirmed.
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
)]
pub enum RecoveryConfirmationKind {
    /// Confirmed by a second role.
    Quick,
    /// Confirmed after the timed recovery delay, which needs no role.
    Timed,
}

/// The confirmation of a recovery already proposed on an access controller.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AccessControllerRecoveryConfirmation {
    pub access_controller: AccessControllerAddress,

    /// The role that proposed the recovery confirmed.
    pub proposer: RecoveryProposer,

    pub kind: RecoveryConfirmationKind,

    /// The rules that become the access controller's rules, with the user's
    /// keys shown as their factor sources.
    pub proposed_rules: AccessControllerRuleSetForDisplay,

    /// The delay the confirmation names. The engine doesn't apply it.
    pub timed_recovery_delay_in_minutes: Option<u32>,

    /// The account or persona the access controller controls, if known.
    pub controlled_entity: Option<AddressOfAccountOrPersona>,
}

/// The recoveries stopped or cancelled on an access controller.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AccessControllerRecoveryCancellation {
    pub access_controller: AccessControllerAddress,

    /// The rules of the recovery role's proposal whose timed recovery is
    /// stopped, if any. Unless also cancelled, it can still be confirmed with
    /// a second role.
    pub stopped_timed_recovery: Option<AccessControllerRuleSetForDisplay>,

    /// The roles whose pending proposals are cancelled.
    pub cancelled_proposers: Vec<RecoveryProposer>,

    pub unlocks_primary_role: bool,

    /// The account or persona the access controller controls, if known.
    pub controlled_entity: Option<AddressOfAccountOrPersona>,
}

#[derive(
    Clone, Copy, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
)]
pub enum PrimaryRoleLockOperation {
    Lock,
    Unlock,
}

/// The primary role of an access controller locked or unlocked.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AccessControllerPrimaryRoleLockOperation {
    pub access_controller: AccessControllerAddress,
    pub operation: PrimaryRoleLockOperation,

    /// The account or persona the access controller controls, if known.
    pub controlled_entity: Option<AddressOfAccountOrPersona>,
}

/// XRD moved into or out of an access controller's recovery fee vault.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AccessControllerXrdTransfer {
    pub access_controller: AccessControllerAddress,
    pub xrd_amount: Decimal192,

    /// The account or persona the access controller controls, if known.
    pub controlled_entity: Option<AddressOfAccountOrPersona>,
}

/// XRD moved out of or into an account, to or from recovery fee vaults.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AccountXrdTransfer {
    pub account: AccountAddress,
    pub xrd_amount: Decimal192,
}

decl_conversion_tests_for!(AccessControllerTimedRecoveryProposal);
decl_conversion_tests_for!(AccessControllerRecoveryConfirmation);
decl_conversion_tests_for!(AccessControllerRecoveryCancellation);
decl_conversion_tests_for!(AccessControllerPrimaryRoleLockOperation);
decl_conversion_tests_for!(AccessControllerXrdTransfer);
decl_conversion_tests_for!(AccountXrdTransfer);
