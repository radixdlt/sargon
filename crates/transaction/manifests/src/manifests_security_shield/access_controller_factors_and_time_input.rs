use crate::prelude::*;
use radix_engine_interface::blueprints::access_controller::{
    AccessControllerInitiateRecoveryAsPrimaryInput as ScryptoAccessControllerInitiateRecoveryAsPrimaryInput,
    AccessControllerInitiateRecoveryAsRecoveryInput as ScryptoAccessControllerInitiateRecoveryAsRecoveryInput,
    AccessControllerQuickConfirmPrimaryRoleRecoveryProposalInput as ScryptoAccessControllerQuickConfirmPrimaryRoleRecoveryProposalInput,
    AccessControllerQuickConfirmRecoveryRoleRecoveryProposalInput as ScryptoAccessControllerQuickConfirmRecoveryRoleRecoveryProposalInput,
    AccessControllerTimedConfirmRecoveryInput as ScryptoAccessControllerTimedConfirmRecoveryInput,
};

/// An ephemeral DTO that is used to create the input for the access controller
/// methods that require factors and time - which we create from a
/// `SecurityStructureOfFactorInstances`
#[derive(Debug, Clone)]
pub struct AccessControllerFactorsAndTimeInput {
    /// RuleSet is Scrypto representation of the security structure's
    /// MatrixOfFactors
    rule_set: ScryptoRuleSet,
    /// The timed recovery delay in minutes we get from `SecurityStructureOfFactorInstances`.
    timed_recovery_delay_in_minutes: u32,
}

impl AccessControllerFactorsAndTimeInput {
    pub fn new(
        security_structure_of_factor_instances: &SecurityStructureOfFactorInstances,
    ) -> Self {
        let rule_set = ScryptoRuleSet::from(
            security_structure_of_factor_instances
                .matrix_of_factors
                .clone(),
        );

        let timed_recovery_delay_in_minutes =
            security_structure_of_factor_instances
                .timed_recovery_delay_in_minutes();

        Self {
            rule_set,
            timed_recovery_delay_in_minutes,
        }
    }

    pub fn with_recovery_proposal(recovery_proposal: RecoveryProposal) -> Self {
        let rule_set = ScryptoRuleSet::from(recovery_proposal.clone());

        let timed_recovery_delay_in_minutes = recovery_proposal
            .timed_recovery_delay_minutes
            .expect("Recovery proposal must have timed recovery delay minutes");

        Self {
            rule_set,
            timed_recovery_delay_in_minutes,
        }
    }
}

impl From<&AccessControllerFactorsAndTimeInput>
    for ScryptoAccessControllerInitiateRecoveryAsRecoveryInput
{
    fn from(value: &AccessControllerFactorsAndTimeInput) -> Self {
        Self {
            rule_set: value.rule_set.clone(),
            timed_recovery_delay_in_minutes: Some(
                value.timed_recovery_delay_in_minutes,
            ),
        }
    }
}

impl From<&AccessControllerFactorsAndTimeInput>
    for ScryptoAccessControllerInitiateRecoveryAsPrimaryInput
{
    fn from(value: &AccessControllerFactorsAndTimeInput) -> Self {
        Self {
            rule_set: value.rule_set.clone(),
            timed_recovery_delay_in_minutes: Some(
                value.timed_recovery_delay_in_minutes,
            ),
        }
    }
}

impl From<&AccessControllerFactorsAndTimeInput>
    for ScryptoAccessControllerQuickConfirmRecoveryRoleRecoveryProposalInput
{
    fn from(value: &AccessControllerFactorsAndTimeInput) -> Self {
        Self {
            rule_set: value.rule_set.clone(),
            timed_recovery_delay_in_minutes: Some(
                value.timed_recovery_delay_in_minutes,
            ),
        }
    }
}

impl From<&AccessControllerFactorsAndTimeInput>
    for ScryptoAccessControllerQuickConfirmPrimaryRoleRecoveryProposalInput
{
    fn from(value: &AccessControllerFactorsAndTimeInput) -> Self {
        Self {
            rule_set: value.rule_set.clone(),
            timed_recovery_delay_in_minutes: Some(
                value.timed_recovery_delay_in_minutes,
            ),
        }
    }
}

impl From<&AccessControllerFactorsAndTimeInput>
    for ScryptoAccessControllerTimedConfirmRecoveryInput
{
    fn from(value: &AccessControllerFactorsAndTimeInput) -> Self {
        Self {
            rule_set: value.rule_set.clone(),
            timed_recovery_delay_in_minutes: Some(
                value.timed_recovery_delay_in_minutes,
            ),
        }
    }
}
