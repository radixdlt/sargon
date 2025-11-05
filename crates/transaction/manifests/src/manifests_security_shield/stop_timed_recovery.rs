use crate::prelude::*;

use profile_supporting_types::AnySecurifiedEntity;
use radix_engine_interface::blueprints::access_controller::{
    AccessControllerCancelRecoveryRoleRecoveryProposalManifestInput as ScryptoAccessControllerCancelRecoveryRoleRecoveryProposalManifestInput,
    AccessControllerTimedConfirmRecoveryInput as ScryptoAccessControllerTimedConfirmRecoveryInput,
    ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT as SCRYPTO_ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
    ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT as SCRYPTO_ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
};

pub trait TransactionManifestCancelTimedRecovery {
    /// Stops timed recovery for the given `ac_address` based on the given `recovery_proposal`
    fn stop_timed_recovery(
        ac_address: AccessControllerAddress,
        recovery_proposal: RecoveryProposal,
    ) -> TransactionManifest;

    /// Stops timed recovery for the given `ac_address` and cancels the recovery attempt based on
    /// the given `recovery_proposal`
    fn stop_and_cancel_timed_recovery(
        ac_address: AccessControllerAddress,
        recovery_proposal: RecoveryProposal,
    ) -> TransactionManifest;

    /// Stops timed recovery for the given `ac_address` and cancels the recovery attempt based on
    /// the given `input`
    fn stop_and_cancel_timed_recovery_with_scrypto_input(
        ac_address: AccessControllerAddress,
        input: ScryptoAccessControllerTimedConfirmRecoveryInput,
    ) -> TransactionManifest;
}

impl TransactionManifestCancelTimedRecovery for TransactionManifest {
    fn stop_timed_recovery(
        ac_address: AccessControllerAddress,
        recovery_proposal: RecoveryProposal,
    ) -> TransactionManifest {
        let factors_and_time =
            AccessControllerFactorsAndTimeInput::with_recovery_proposal(
                recovery_proposal,
            );

        let mut builder = ScryptoTransactionManifestBuilder::new();
        builder = builder.call_method(
            ac_address.scrypto(),
            SCRYPTO_ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
            ScryptoAccessControllerTimedConfirmRecoveryInput::from(
                &factors_and_time,
            ),
        );

        TransactionManifest::sargon_built(builder, ac_address.network_id())
    }

    fn stop_and_cancel_timed_recovery(
        ac_address: AccessControllerAddress,
        recovery_proposal: RecoveryProposal,
    ) -> Self {
        let factors_and_time =
            AccessControllerFactorsAndTimeInput::with_recovery_proposal(
                recovery_proposal,
            );

        Self::stop_and_cancel_timed_recovery_with_scrypto_input(
            ac_address,
            ScryptoAccessControllerTimedConfirmRecoveryInput::from(
                &factors_and_time,
            ),
        )
    }

    fn stop_and_cancel_timed_recovery_with_scrypto_input(
        ac_address: AccessControllerAddress,
        input: ScryptoAccessControllerTimedConfirmRecoveryInput,
    ) -> Self {
        let mut builder = ScryptoTransactionManifestBuilder::new();
        builder = builder
            .call_method(
                ac_address.scrypto(),
                SCRYPTO_ACCESS_CONTROLLER_STOP_TIMED_RECOVERY_IDENT,
                input,
            )
            .call_method(
                ac_address.scrypto(),
                SCRYPTO_ACCESS_CONTROLLER_CANCEL_RECOVERY_ROLE_RECOVERY_PROPOSAL_IDENT,
                ScryptoAccessControllerCancelRecoveryRoleRecoveryProposalManifestInput {},
            );

        TransactionManifest::sargon_built(builder, ac_address.network_id())
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use prelude::{fixture_gw_model, fixture_rtm};
    use profile_supporting_types::{
        SecurifiedAccount, SecurifiedPersona, UnsecurifiedAccount,
    };
    use radix_engine::blueprints::access_controller::{
        RecoveryRoleRecoveryAttemptState, RecoveryRoleRecoveryState,
    };
    use radix_engine_interface::blueprints::access_controller::RecoveryProposal as ScryptoRecoveryProposal;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn stop_timed_recovery() {
        let ac_address_str = "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a";
        let ac_address =
            AccessControllerAddress::try_from_bech32(ac_address_str).unwrap();
        let mut securified_persona = SecurifiedPersona::sample().clone();
        let mut new_securified_entity_control =
            securified_persona.securified_entity_control.clone();
        new_securified_entity_control.access_controller_address =
            ac_address.clone();
        securified_persona.securified_entity_control =
            new_securified_entity_control;
        assert_eq!(
            securified_persona
                .securified_entity_control
                .access_controller_address()
                .to_string(),
            ac_address_str
        );

        let recovery_attempt = fixture_and_json::<RecoveryRoleRecoveryAttempt>(
            fixture_gw_model!("state/ac_state_details_recovery_attempt"),
        )
        .unwrap()
        .0;

        let manifest = SUT::stop_timed_recovery(
            ac_address,
            recovery_attempt.recovery_proposal.clone(),
        );

        let expected_manifest_str =
            fixture_rtm!("stop_persona_shield_timed_recovery");
        manifest_eq(manifest.clone(), expected_manifest_str);
        assert!(expected_manifest_str.contains(ac_address_str));

        let fee_payer_account = Account::sample();
        let manifest = SUT::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_securified_entity_paid_by_account(fee_payer_account, securified_persona.clone(), manifest.clone(), Decimal192::ten(), RolesExercisableInTransactionManifestCombination::manifest_end_user_gets_to_preview()).unwrap();

        let expected_manifest_str =
            fixture_rtm!("stop_persona_shield_timed_recovery_with_top_up_where_payer_is_entity_confirming_recovery");
        manifest_eq(manifest.clone(), expected_manifest_str);

        let manifest = SUT::modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(manifest, Decimal192::nine(), securified_persona.access_controller_address().clone());

        let expected_manifest_str =
            fixture_rtm!("stop_persona_shield_timed_recovery_with_top_up_where_payer_is_entity_confirming_recovery_with_xrd_lock");
        manifest_eq(manifest, expected_manifest_str);
    }

    #[test]
    fn stop_and_cancel_timed_recovery() {
        let ac_address_str = "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a";
        let ac_address =
            AccessControllerAddress::try_from_bech32(ac_address_str).unwrap();
        let mut securified_account = SecurifiedAccount::sample();
        let mut new_securified_entity_control =
            securified_account.securified_entity_control.clone();
        new_securified_entity_control.access_controller_address =
            ac_address.clone();
        securified_account.securified_entity_control =
            new_securified_entity_control;
        assert_eq!(
            securified_account
                .securified_entity_control
                .access_controller_address()
                .to_string(),
            ac_address_str
        );

        let recovery_attempt = fixture_and_json::<RecoveryRoleRecoveryAttempt>(
            fixture_gw_model!("state/ac_state_details_recovery_attempt"),
        )
        .unwrap()
        .0;

        let manifest = SUT::stop_and_cancel_timed_recovery(
            ac_address,
            recovery_attempt.recovery_proposal.clone(),
        );

        let expected_manifest_str =
            fixture_rtm!("stop_and_cancel_account_shield_timed_recovery");
        manifest_eq(manifest.clone(), expected_manifest_str);
        assert!(expected_manifest_str.contains(ac_address_str));

        let manifest = SUT::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_securified_entity_paid_by_account(securified_account.clone(), securified_account.clone(), manifest.clone(), Decimal192::ten(), RolesExercisableInTransactionManifestCombination::manifest_end_user_gets_to_preview()).unwrap();

        let expected_manifest_str =
            fixture_rtm!("stop_and_cancel_account_shield_timed_recovery_with_top_up_where_payer_is_entity_confirming_recovery");
        manifest_eq(manifest.clone(), expected_manifest_str);

        let manifest = SUT::modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(manifest, Decimal192::nine(), securified_account.access_controller_address().clone());

        let expected_manifest_str =
            fixture_rtm!("stop_and_cancel_account_shield_timed_recovery_with_top_up_where_payer_is_entity_confirming_recovery_with_xrd_lock");
        manifest_eq(manifest, expected_manifest_str);
    }

    #[test]
    fn stop_and_cancel_timed_recovery_transaction() {
        let mut ledger =
            LedgerSimulatorBuilder::new().without_kernel_trace().build();

        let unsecurified_acc = UnsecurifiedAccount::sample_sim_account();
        let sec_structure = SecurityStructureOfFactorInstances::sample_sim();

        // Securify the account by creating the AC
        let mut securified_account = ledger
            .securify_account(unsecurified_acc.clone(), sec_structure.clone());

        let updated_sec_structure =
            SecurityStructureOfFactorInstances::sample_other_sim();

        // Update the security structure by exercising the recovery role with delayed completion
        let (ac_substate, rule_set) = ledger.execute_recovery_transaction(
            securified_account.clone(),
            updated_sec_structure.clone(),
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion
        );

        let proposed_recovery_rule_set: ScryptoRuleSet =
            updated_sec_structure.matrix_of_factors.clone().into();
        let expected_recovery_attempt =
            RecoveryRoleRecoveryAttemptState::RecoveryAttempt(
                RecoveryRoleRecoveryState::TimedRecovery {
                    proposal: ScryptoRecoveryProposal {
                        rule_set: proposed_recovery_rule_set,
                        timed_recovery_delay_in_minutes: Some(
                            updated_sec_structure
                                .timed_recovery_delay_in_minutes(),
                        ),
                    },
                    timed_recovery_allowed_after: ScryptoInstant::new(
                        i64::from(
                            updated_sec_structure
                                .timed_recovery_delay_in_minutes()
                                * 60,
                        ),
                    ),
                },
            );
        let (_, _, _, recovery_role_recovery_attempt, _) = ac_substate.state;

        pretty_assertions::assert_eq!(
            recovery_role_recovery_attempt,
            expected_recovery_attempt
        );

        let factors_and_time =
            AccessControllerFactorsAndTimeInput::new(&updated_sec_structure);
        let scrypto_input =
            ScryptoAccessControllerTimedConfirmRecoveryInput::from(
                &factors_and_time,
            );

        // Cancel timed recovery
        let mut manifest = TransactionManifest::stop_and_cancel_timed_recovery_with_scrypto_input(
            securified_account.access_controller_address(),
            scrypto_input
        );

        manifest = TransactionManifest::modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(
            manifest,
            Decimal192::one(),
            securified_account.access_controller_address().clone(),
        );

        let notarized_tx =
            ledger.sign_and_notarize_transaction(manifest.into(), |intent| {
                securified_account.securified_entity_control.sign_intent_with_roles(intent, RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion)
            });
        ledger
            .execute_notarized_transaction(notarized_tx)
            .expect_commit_success();

        // Assert that the on ledger rule set matches the rule set before starting recovery
        let rule_set = ledger.read_access_controller_rule_set(
            securified_account.access_controller_address().clone(),
        );
        let state = ledger.read_access_controller_substate(
            securified_account.access_controller_address(),
        );

        let expected_rule_set =
            SecurityStructureOfFactorInstances::sample_sim()
                .matrix_of_factors
                .into();
        let (_, _, _, recovery_role_recovery_attempt_new_state, _) =
            state.state;
        pretty_assertions::assert_eq!(
            recovery_role_recovery_attempt_new_state,
            RecoveryRoleRecoveryAttemptState::NoRecoveryAttempt
        );
        pretty_assertions::assert_eq!(rule_set, expected_rule_set);
    }
}
