use crate::prelude::*;

use profile_supporting_types::AnySecurifiedEntity;
use radix_engine_interface::blueprints::access_controller::{
    AccessControllerTimedConfirmRecoveryInput as ScryptoAccessControllerTimedConfirmRecoveryInput,
    ACCESS_CONTROLLER_TIMED_CONFIRM_RECOVERY_IDENT as SCRYPTO_ACCESS_CONTROLLER_TIMED_CONFIRM_RECOVERY_IDENT,
};

pub trait TransactionManifestConfirmTimedRecovery {
    /// Confirms the timed recovery for the given `securified_entity`
    /// The `securified_entity` must have a `provisional_securified_config` set up.
    fn confirm_timed_recovery(
        securified_entity: impl Into<AnySecurifiedEntity>,
    ) -> TransactionManifest;
}

impl TransactionManifestConfirmTimedRecovery for TransactionManifest {
    fn confirm_timed_recovery(
        securified_entity: impl Into<AnySecurifiedEntity>,
    ) -> Self {
        let securified_entity = securified_entity.into();
        let securified_control =
            securified_entity.securified_entity_control.clone();

        let access_controller_address =
            securified_control.access_controller_address;
        let security_structure_of_factor_instances = securified_control
            .provisional_securified_config
            .expect("The provisional config must be present")
            .get_security_structure_of_factor_instances();

        let factors_and_time = AccessControllerFactorsAndTimeInput::new(
            &security_structure_of_factor_instances,
        );

        let mut builder = ScryptoTransactionManifestBuilder::new();
        builder = builder.call_method(
            access_controller_address.scrypto(),
            SCRYPTO_ACCESS_CONTROLLER_TIMED_CONFIRM_RECOVERY_IDENT,
            ScryptoAccessControllerTimedConfirmRecoveryInput::from(
                &factors_and_time,
            ),
        );

        TransactionManifest::sargon_built(
            builder,
            securified_entity.network_id(),
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use prelude::fixture_rtm;
    use profile_supporting_types::{
        SecurifiedAccount, UnsecurifiedAccount, UnsecurifiedPersona,
    };
    use scrypto_test::prelude::{
        RecoveryProposal, RecoveryRoleRecoveryAttemptState,
        RecoveryRoleRecoveryState,
    };

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn confirm_timed_recovery() {
        let mut securified_account = SecurifiedAccount::sample();
        assert_eq!(securified_account.securified_entity_control.access_controller_address().to_string(), "accesscontroller_rdx1cdgcq7yqee9uhyqrsp9kgud3a7h4dvz3dqmx26ws5dmjsu7g3zg23g");

        // Add provisional securified config
        securified_account
            .entity
            .security_state
            .set_provisional(Some(ProvisionalSecurifiedConfig::sample()));
        securified_account =
            SecurifiedAccount::new(securified_account.entity).unwrap();
        pretty_assertions::assert_eq!(
            securified_account
                .clone()
                .securified_entity_control
                .provisional_securified_config
                .unwrap()
                .get_security_structure_of_factor_instances(),
            SecurityStructureOfFactorInstances::sample()
        );

        let manifest = SUT::confirm_timed_recovery(securified_account.clone());

        let expected_manifest_str =
            fixture_rtm!("confirm_account_shield_timed_recovery");
        manifest_eq(manifest.clone(), expected_manifest_str);
        assert!(expected_manifest_str.contains("accesscontroller_rdx1cdgcq7yqee9uhyqrsp9kgud3a7h4dvz3dqmx26ws5dmjsu7g3zg23g"));

        let manifest = SUT::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_securified_entity_paid_by_account(securified_account.clone(), securified_account.clone(), manifest.clone(), Decimal192::ten(), RolesExercisableInTransactionManifestCombination::manifest_end_user_gets_to_preview()).unwrap();

        let expected_manifest_str =
            fixture_rtm!("confirm_account_shield_timed_recovery_with_top_up_where_payer_is_entity_confirming_recovery");
        manifest_eq(manifest.clone(), expected_manifest_str);

        let manifest = SUT::modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(manifest, Decimal192::nine(), securified_account);

        let expected_manifest_str =
            fixture_rtm!("confirm_account_shield_timed_recovery_with_top_up_where_payer_is_entity_confirming_recovery_with_xrd_lock");
        manifest_eq(manifest, expected_manifest_str);
    }

    #[test]
    fn confirm_timed_recovery_transaction() {
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
                    proposal: RecoveryProposal {
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

        // Set provisional config on account
        securified_account.entity.set_provisional(Some(
            ProvisionalSecurifiedConfig::FactorInstancesDerived {
                value: updated_sec_structure.clone(),
            },
        ));
        securified_account =
            SecurifiedAccount::new(securified_account.entity).unwrap();
        pretty_assertions::assert_eq!(
            securified_account
                .clone()
                .securified_entity_control
                .provisional_securified_config
                .unwrap()
                .get_security_structure_of_factor_instances(),
            updated_sec_structure.clone()
        );

        // Advance time by 14 days (+1 ms) by moving to the next consensus round
        let now_ms = ledger.get_current_proposer_timestamp_ms();
        let current_round = ledger.get_consensus_manager_state().round;
        let target_ts = now_ms + (14 * 24 * 60 * 60_000) + 1;
        let next_round = radix_engine_interface::prelude::Round::of(
            current_round.number() + 1,
        );
        ledger.advance_to_round_at_timestamp(next_round, target_ts);

        // Confirm timed recovery
        let mut manifest = TransactionManifest::confirm_timed_recovery(
            securified_account.clone(),
        );

        manifest = TransactionManifest::modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(
            manifest,
            Decimal192::one(),
            securified_account.clone(),
        );

        let notarized_tx =
            ledger.sign_and_notarize_transaction(manifest.into(), |intent| {
                securified_account.securified_entity_control.sign_intent_with_roles(intent, RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion)
            });
        ledger
            .execute_notarized_transaction(notarized_tx)
            .expect_commit_success();

        // Assert that the on ledger rule set matches the expected rule set
        let rule_set = ledger.read_access_controller_rule_set(
            securified_account.access_controller_address().clone(),
        );
        let state = ledger.read_access_controller_substate(
            securified_account.access_controller_address(),
        );

        let expected_rule_set =
            SecurityStructureOfFactorInstances::sample_other_sim()
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
