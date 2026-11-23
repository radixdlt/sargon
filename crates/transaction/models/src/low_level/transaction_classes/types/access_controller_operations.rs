use crate::prelude::*;
use radix_engine_interface::blueprints::access_controller::RuleSet as ScryptoRuleSet;

/// A timed recovery started on an access controller: the recovery role
/// proposes rules without confirming them. Once the access controller's own
/// timed recovery delay has passed, anyone can confirm them.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessControllerTimedRecoveryProposal {
    pub access_controller: AccessControllerAddress,

    /// The rules that replace the access controller's current ones once the
    /// recovery is confirmed. Every key is shown as unknown until the OS marks
    /// the user's keys.
    pub proposed_rules: AccessControllerRuleSetForDisplay,

    /// The delay the proposal names. The engine doesn't apply it when the
    /// recovery is confirmed: the access controller keeps its own, see
    /// `current_timed_recovery_delay_in_minutes`.
    pub timed_recovery_delay_in_minutes: Option<u32>,

    /// Whether the primary role is also locked, so that the entity can't be
    /// used until the recovery is confirmed or the role unlocked.
    pub locks_primary_role: bool,

    /// The access controller's current timed recovery delay, after which the
    /// recovery can be confirmed without the confirmation role. Read from the
    /// ledger by the OS; `None` if it couldn't be read or there is none.
    pub current_timed_recovery_delay_in_minutes: Option<u32>,

    /// The account or persona the access controller controls, if known. Read
    /// from the ledger by the OS.
    pub controlled_entity: Option<AddressOfAccountOrPersona>,
}

/// How a recovery is confirmed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RecoveryConfirmationKind {
    /// Confirmed by a second role.
    Quick,
    /// Confirmed after the timed recovery delay, which needs no role.
    Timed,
}

/// The confirmation of a recovery already proposed on an access controller.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessControllerRecoveryConfirmation {
    pub access_controller: AccessControllerAddress,

    /// The role that proposed the recovery confirmed.
    pub proposer: RecoveryProposer,

    pub kind: RecoveryConfirmationKind,

    /// The rules the confirmation names, which become the access controller's
    /// rules: the engine fails the confirmation unless they're the proposed
    /// ones.
    pub proposed_rules: AccessControllerRuleSetForDisplay,

    /// The delay the confirmation names. The engine doesn't apply it.
    pub timed_recovery_delay_in_minutes: Option<u32>,

    /// The account or persona the access controller controls, if known.
    pub controlled_entity: Option<AddressOfAccountOrPersona>,
}

/// The recoveries stopped or cancelled on an access controller.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessControllerRecoveryCancellation {
    pub access_controller: AccessControllerAddress,

    /// The rules of the recovery role's proposal whose timed recovery is
    /// stopped, if any. Stopping only stops the timer: unless also cancelled,
    /// the proposal can still be confirmed with a second role.
    pub stopped_timed_recovery: Option<AccessControllerRuleSetForDisplay>,

    /// The roles whose proposals are cancelled. A cancellation doesn't name
    /// the proposal: it's whichever is pending when the transaction runs.
    pub cancelled_proposers: Vec<RecoveryProposer>,

    pub unlocks_primary_role: bool,

    /// The account or persona the access controller controls, if known.
    pub controlled_entity: Option<AddressOfAccountOrPersona>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrimaryRoleLockOperation {
    Lock,
    Unlock,
}

/// The primary role of an access controller locked or unlocked.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessControllerPrimaryRoleLockOperation {
    pub access_controller: AccessControllerAddress,
    pub operation: PrimaryRoleLockOperation,

    /// The account or persona the access controller controls, if known.
    pub controlled_entity: Option<AddressOfAccountOrPersona>,
}

/// XRD moved into or out of an access controller's recovery fee vault.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessControllerXrdTransfer {
    pub access_controller: AccessControllerAddress,
    pub xrd_amount: Decimal192,

    /// The account or persona the access controller controls, if known.
    pub controlled_entity: Option<AddressOfAccountOrPersona>,
}

/// XRD moved out of or into an account, to or from recovery fee vaults.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccountXrdTransfer {
    pub account: AccountAddress,
    pub xrd_amount: Decimal192,
}

fn rules_for_display(
    rule_set: ScryptoRuleSet,
    network_id: NetworkID,
) -> AccessControllerRuleSetForDisplay {
    AccessControllerRuleSet::from((rule_set, network_id)).for_display()
}

impl From<(RetAccessControllerTimedRecovery, NetworkID)>
    for AccessControllerTimedRecoveryProposal
{
    fn from(
        (recovery, n): (RetAccessControllerTimedRecovery, NetworkID),
    ) -> Self {
        Self {
            access_controller: (recovery.access_controller, n).into(),
            proposed_rules: rules_for_display(recovery.rule_set, n),
            timed_recovery_delay_in_minutes: recovery
                .timed_recovery_delay_in_minutes,
            locks_primary_role: recovery.locks_primary_role,
            current_timed_recovery_delay_in_minutes: None,
            controlled_entity: None,
        }
    }
}

impl From<RetRecoveryConfirmationKind> for RecoveryConfirmationKind {
    fn from(value: RetRecoveryConfirmationKind) -> Self {
        match value {
            RetRecoveryConfirmationKind::Quick => Self::Quick,
            RetRecoveryConfirmationKind::Timed => Self::Timed,
        }
    }
}

impl From<(RetAccessControllerRecoveryConfirmation, NetworkID)>
    for AccessControllerRecoveryConfirmation
{
    fn from(
        (confirmation, n): (RetAccessControllerRecoveryConfirmation, NetworkID),
    ) -> Self {
        Self {
            access_controller: (confirmation.access_controller, n).into(),
            proposer: confirmation.proposer.into(),
            kind: confirmation.kind.into(),
            proposed_rules: rules_for_display(confirmation.rule_set, n),
            timed_recovery_delay_in_minutes: confirmation
                .timed_recovery_delay_in_minutes,
            controlled_entity: None,
        }
    }
}

impl From<(RetAccessControllerRecoveryCancellation, NetworkID)>
    for AccessControllerRecoveryCancellation
{
    fn from(
        (cancellation, n): (RetAccessControllerRecoveryCancellation, NetworkID),
    ) -> Self {
        Self {
            access_controller: (cancellation.access_controller, n).into(),
            stopped_timed_recovery: cancellation
                .stopped_timed_recovery
                .map(|proposal| rules_for_display(proposal.rule_set, n)),
            cancelled_proposers: cancellation
                .cancelled_proposers
                .into_iter()
                .map(RecoveryProposer::from)
                .collect(),
            unlocks_primary_role: cancellation.unlocks_primary_role,
            controlled_entity: None,
        }
    }
}

impl From<RetPrimaryRoleLockOperation> for PrimaryRoleLockOperation {
    fn from(value: RetPrimaryRoleLockOperation) -> Self {
        match value {
            RetPrimaryRoleLockOperation::Lock => Self::Lock,
            RetPrimaryRoleLockOperation::Unlock => Self::Unlock,
        }
    }
}

impl From<(RetAccessControllerPrimaryRoleLockOperation, NetworkID)>
    for AccessControllerPrimaryRoleLockOperation
{
    fn from(
        (lock, n): (RetAccessControllerPrimaryRoleLockOperation, NetworkID),
    ) -> Self {
        Self {
            access_controller: (lock.access_controller, n).into(),
            operation: lock.operation.into(),
            controlled_entity: None,
        }
    }
}

impl From<(RetAccessControllerRecoveryFeeTransfer, NetworkID)>
    for AccessControllerXrdTransfer
{
    fn from(
        (transfer, n): (RetAccessControllerRecoveryFeeTransfer, NetworkID),
    ) -> Self {
        Self {
            access_controller: (transfer.access_controller, n).into(),
            xrd_amount: transfer.xrd_amount.into(),
            controlled_entity: None,
        }
    }
}

impl From<(RetAccountRecoveryFeeTransfer, NetworkID)> for AccountXrdTransfer {
    fn from((transfer, n): (RetAccountRecoveryFeeTransfer, NetworkID)) -> Self {
        Self {
            account: (transfer.account, n).into(),
            xrd_amount: transfer.xrd_amount.into(),
        }
    }
}

impl HasSampleValues for AccessControllerTimedRecoveryProposal {
    fn sample() -> Self {
        Self {
            access_controller: AccessControllerAddress::sample(),
            proposed_rules: AccessControllerRuleSetForDisplay::sample(),
            timed_recovery_delay_in_minutes: Some(20160),
            locks_primary_role: true,
            current_timed_recovery_delay_in_minutes: Some(20160),
            controlled_entity: Some(AccountAddress::sample().into()),
        }
    }

    fn sample_other() -> Self {
        Self {
            access_controller: AccessControllerAddress::sample_other(),
            proposed_rules: AccessControllerRuleSetForDisplay::sample_other(),
            timed_recovery_delay_in_minutes: None,
            locks_primary_role: false,
            current_timed_recovery_delay_in_minutes: None,
            controlled_entity: None,
        }
    }
}

impl HasSampleValues for AccessControllerRecoveryConfirmation {
    fn sample() -> Self {
        Self {
            access_controller: AccessControllerAddress::sample(),
            proposer: RecoveryProposer::Recovery,
            kind: RecoveryConfirmationKind::Timed,
            proposed_rules: AccessControllerRuleSetForDisplay::sample(),
            timed_recovery_delay_in_minutes: Some(20160),
            controlled_entity: Some(AccountAddress::sample().into()),
        }
    }

    fn sample_other() -> Self {
        Self {
            access_controller: AccessControllerAddress::sample_other(),
            proposer: RecoveryProposer::Primary,
            kind: RecoveryConfirmationKind::Quick,
            proposed_rules: AccessControllerRuleSetForDisplay::sample_other(),
            timed_recovery_delay_in_minutes: None,
            controlled_entity: None,
        }
    }
}

impl HasSampleValues for AccessControllerRecoveryCancellation {
    fn sample() -> Self {
        Self {
            access_controller: AccessControllerAddress::sample(),
            stopped_timed_recovery: Some(
                AccessControllerRuleSetForDisplay::sample(),
            ),
            cancelled_proposers: vec![RecoveryProposer::Recovery],
            unlocks_primary_role: true,
            controlled_entity: Some(AccountAddress::sample().into()),
        }
    }

    fn sample_other() -> Self {
        Self {
            access_controller: AccessControllerAddress::sample_other(),
            stopped_timed_recovery: None,
            cancelled_proposers: vec![RecoveryProposer::Primary],
            unlocks_primary_role: false,
            controlled_entity: None,
        }
    }
}

impl HasSampleValues for AccessControllerPrimaryRoleLockOperation {
    fn sample() -> Self {
        Self {
            access_controller: AccessControllerAddress::sample(),
            operation: PrimaryRoleLockOperation::Lock,
            controlled_entity: Some(AccountAddress::sample().into()),
        }
    }

    fn sample_other() -> Self {
        Self {
            access_controller: AccessControllerAddress::sample_other(),
            operation: PrimaryRoleLockOperation::Unlock,
            controlled_entity: None,
        }
    }
}

impl HasSampleValues for AccessControllerXrdTransfer {
    fn sample() -> Self {
        Self {
            access_controller: AccessControllerAddress::sample(),
            xrd_amount: Decimal192::ten(),
            controlled_entity: Some(AccountAddress::sample().into()),
        }
    }

    fn sample_other() -> Self {
        Self {
            access_controller: AccessControllerAddress::sample_other(),
            xrd_amount: Decimal192::one(),
            controlled_entity: None,
        }
    }
}

impl HasSampleValues for AccountXrdTransfer {
    fn sample() -> Self {
        Self {
            account: AccountAddress::sample(),
            xrd_amount: Decimal192::ten(),
        }
    }

    fn sample_other() -> Self {
        Self {
            account: AccountAddress::sample_other(),
            xrd_amount: Decimal192::one(),
        }
    }
}
