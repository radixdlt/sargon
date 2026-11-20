use crate::prelude::*;
use radix_engine_interface::blueprints::access_controller::Proposer as ScryptoProposer;

/// The role that initiated an access controller recovery proposal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RecoveryProposer {
    Primary,
    Recovery,
}

impl From<ScryptoProposer> for RecoveryProposer {
    fn from(value: ScryptoProposer) -> Self {
        match value {
            ScryptoProposer::Primary => Self::Primary,
            ScryptoProposer::Recovery => Self::Recovery,
        }
    }
}

/// A recovery proposed on an access controller in a manifest.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessControllerRecoveryProposal {
    pub access_controller: AccessControllerAddress,

    /// The role that initiated the recovery.
    pub proposer: RecoveryProposer,

    /// The rules that replace the access controller's current ones once the
    /// recovery is confirmed. Every key is shown as unknown until the OS marks
    /// the user's keys.
    pub proposed_rules: AccessControllerRuleSetForDisplay,

    /// The proposed time after which a recovery can be confirmed without the
    /// confirmation role, if any.
    pub timed_recovery_delay_in_minutes: Option<u32>,

    /// The account or persona whose owner badge the access controller holds,
    /// if known. The classification doesn't know it; the OS reads it from the
    /// ledger.
    pub controlled_entity: Option<AddressOfAccountOrPersona>,
}

impl From<(RetAccessControllerRecoveryProposal, NetworkID)>
    for AccessControllerRecoveryProposal
{
    fn from(
        (proposal, network_id): (
            RetAccessControllerRecoveryProposal,
            NetworkID,
        ),
    ) -> Self {
        Self {
            access_controller: (proposal.access_controller, network_id).into(),
            proposer: proposal.proposer.into(),
            proposed_rules: AccessControllerRuleSet::from((
                proposal.rule_set,
                network_id,
            ))
            .for_display(),
            timed_recovery_delay_in_minutes: proposal
                .timed_recovery_delay_in_minutes,
            controlled_entity: None,
        }
    }
}

impl HasSampleValues for AccessControllerRecoveryProposal {
    fn sample() -> Self {
        Self {
            access_controller: AccessControllerAddress::sample(),
            proposer: RecoveryProposer::Recovery,
            proposed_rules: AccessControllerRuleSetForDisplay::sample(),
            timed_recovery_delay_in_minutes: Some(20160),
            controlled_entity: Some(AccountAddress::sample().into()),
        }
    }

    fn sample_other() -> Self {
        Self {
            access_controller: AccessControllerAddress::sample_other(),
            proposer: RecoveryProposer::Primary,
            proposed_rules: AccessControllerRuleSetForDisplay::sample_other(),
            timed_recovery_delay_in_minutes: None,
            controlled_entity: None,
        }
    }
}
