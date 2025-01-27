#![allow(dead_code)]
use crate::prelude::*;
use std::ops::Deref;

use profile_supporting_types::AnySecurifiedEntity;

pub trait TransactionManifestSecurifySecurifiedEntity:
    TransactionManifestSetRolaKey
{
    fn apply_security_shield_for_securified_entity(
        securified_entity: impl Into<AnySecurifiedEntity>,
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
        roles_combination: RolesExercisableInTransactionManifestCombination,
    ) -> Option<TransactionManifest>;
}

impl TransactionManifestSecurifySecurifiedEntity for TransactionManifest {
    /// Updates the security shield of a securified entity to `security_structure_of_factor_instances`.
    ///
    /// Also conditionally updates the Rola key of the entity - if it is new.
    ///
    /// Later once we have got a preview from Gateway - we will need to call:
    /// * `modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller`
    ///
    /// And when we know the fee we can calculate how much to top up the XRD vault of the AccessController
    /// and call
    /// * `modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_securified_account_paid_by_account`
    ///
    /// For timed confirmation - much later (`timed_recovery_delay_in_minutes` later ) the
    /// host app will need to call `confirm_timed_recovery`
    fn apply_security_shield_for_securified_entity(
        securified_entity: impl Into<AnySecurifiedEntity>,
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
        roles_combination: RolesExercisableInTransactionManifestCombination,
    ) -> Option<Self> {
        let securified_entity = securified_entity.into();
        let kind = roles_combination;
        let entity_address = securified_entity.entity.address();

        security_structure_of_factor_instances
            .assert_has_entity_kind(entity_address.get_entity_kind()).expect("Shouldn't have used wrong FactorInstance for entity - apply_security_shield_with_id_to_entities has some bug.");

        // ACCESS_CONTROLLER_CREATE_PROOF_IDENT
        let mut builder = ScryptoTransactionManifestBuilder::new();

        let access_controller_address = securified_entity
            .securified_entity_control
            .access_controller_address;

        let factors_and_time_input = &AccessControllerFactorsAndTimeInput::new(
            &security_structure_of_factor_instances,
        );

        // INITIATE RECOVERY
        let (init_method, init_input) =
            kind.input_for_initialization(factors_and_time_input);
        builder = builder.call_method(
            access_controller_address.scrypto(),
            init_method,
            (init_input.deref(),),
        );

        // QUICK CONFIRM RECOVERY - Only if we can exercise the confirmation role explicitly.
        if let Some((confirm_method, confirm_input)) =
            kind.input_for_quick_confirm(factors_and_time_input)
        {
            builder = builder.call_method(
                access_controller_address.scrypto(),
                confirm_method,
                (confirm_input.deref(),),
            );
        }

        // Set Rola Key
        let should_set_rola_key = security_structure_of_factor_instances
            .authentication_signing_factor_instance
            != securified_entity
                .current_authentication_signing_factor_instance();

        if should_set_rola_key {
            if kind.can_set_rola_key() {
                builder = TransactionManifest::set_rola_key(
                    builder,
                    &security_structure_of_factor_instances
                        .authentication_signing_factor_instance,
                    &entity_address,
                );
            } else {
                return None; // Nothing has "failed" really, but we cannot proceed with this combination.
            }
        }

        let manifest = TransactionManifest::sargon_built(
            builder,
            securified_entity.network_id(),
        );

        // N.B.
        // We will not lock fee against the XRD vault yet - we will do that
        // later when we have made a preview/dry-run of the `manifest` to get
        // the estimated fee to lock, by calling `modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller`
        //
        // Furthermore:
        // We do NOT top of XRD vault of AccessController - yet!
        // Host will need to call the function:
        // `modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_securified_account_paid_by_account`
        // after user has selected account to pay in wallet GUI. And also call
        // `modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller`

        Some(manifest)
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use prelude::fixture_rtm;
    use profile_supporting_types::{SecurifiedAccount, SecurifiedPersona};

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn classify() {
        let test =
            |combination: RolesExercisableInTransactionManifestCombination,
             expected: bool| {
                let entity_applying_shield = AnySecurifiedEntity::sample();
                let mut instances =
                    SecurityStructureOfFactorInstances::sample();

                // skip set rola
                instances.authentication_signing_factor_instance =
                    entity_applying_shield
                        .current_authentication_signing_factor_instance();
                let manifest =
                    SUT::apply_security_shield_for_securified_entity(
                        entity_applying_shield.clone(),
                        instances,
                        combination,
                    )
                    .unwrap();
                let classified = manifest.explicitly_references_primary_role();
                assert_eq!(classified, expected);
            };
        test(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation, true);
        test(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithRecovery, true);
        // test(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion, true);

        test(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation, false);
        test(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion, false);
    }

    #[test]
    fn update_shield_of_securified_account_with_top_up_where_payer_is_entity_applying_shield(
    ) {
        let entity_applying_shield = SecurifiedAccount::sample();
        assert_eq!(entity_applying_shield.securified_entity_control.access_controller_address.to_string(), "accesscontroller_rdx1cdgcq7yqee9uhyqrsp9kgud3a7h4dvz3dqmx26ws5dmjsu7g3zg23g");

        let manifest = SUT::apply_security_shield_for_securified_entity(
            entity_applying_shield.clone(),
            SecurityStructureOfFactorInstances::sample(),
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation,
        ).unwrap();

        let expected_manifest_str =
            fixture_rtm!("update_shield_of_account_init_with_P_confirm_with_C");
        manifest_eq(manifest.clone(), expected_manifest_str);
        assert!(expected_manifest_str.contains("accesscontroller_rdx1cdgcq7yqee9uhyqrsp9kgud3a7h4dvz3dqmx26ws5dmjsu7g3zg23g"));

        let manifest = SUT::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_securified_account_paid_by_account(entity_applying_shield.clone(), entity_applying_shield.clone(), manifest.clone(), Decimal192::ten()).unwrap();

        let expected_manifest_str =
        fixture_rtm!("update_shield_of_account_init_with_P_confirm_with_C_with_top_up_where_payer_is_entity_applying_shield");
        manifest_eq(manifest.clone(), expected_manifest_str);

        let manifest = SUT::modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(manifest, Decimal192::nine(), entity_applying_shield);

        let expected_manifest_str =
        fixture_rtm!("update_shield_of_account_init_with_P_confirm_with_C_with_top_up_where_payer_is_entity_applying_shield_with_xrd_lock");
        manifest_eq(manifest, expected_manifest_str);
    }

    fn test_update_shield_of_securified_persona_cond_set_rola<'a>(
        roles: RolesExercisableInTransactionManifestCombination,
        set_rola: bool,
        rtm: impl Fn() -> &'a str,
    ) {
        let entity_applying_shield = SecurifiedPersona::sample();
        let mut instances = SecurityStructureOfFactorInstances::sample_other();
        if !set_rola {
            instances.authentication_signing_factor_instance =
                entity_applying_shield
                    .current_authentication_signing_factor_instance();
        }
        let manifest = SUT::apply_security_shield_for_securified_entity(
            entity_applying_shield.clone(),
            instances,
            roles,
        )
        .unwrap();
        let expected_manifest_str = rtm();
        manifest_eq(manifest.clone(), expected_manifest_str);
    }

    fn test_update_shield_of_securified_persona<'a>(
        roles: RolesExercisableInTransactionManifestCombination,
        rtm: impl Fn() -> &'a str,
    ) {
        test_update_shield_of_securified_persona_cond_set_rola(roles, true, rtm)
    }

    #[test]
    fn update_shield_of_securified_persona_init_with_P_confirm_with_R() {
        test_update_shield_of_securified_persona(
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithRecovery,
            || fixture_rtm!("update_shield_of_persona_init_with_P_confirm_with_R")
        )
    }

    #[test]
    fn update_shield_of_securified_persona_init_with_P_confirm_with_C() {
        test_update_shield_of_securified_persona(
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation,
            || fixture_rtm!("update_shield_of_persona_init_with_P_confirm_with_C")
        )
    }

    // #[test] // requires Dugong
    // fn update_shield_of_securified_persona_init_with_P_confirm_with_T() {
    //     test_update_shield_of_securified_persona(
    //         RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion,
    //         || fixture_rtm!("update_shield_of_persona_init_with_P_confirm_with_T")
    //     )
    // }

    #[test]
    fn update_shield_of_securified_persona_init_with_R_confirm_with_C() {
        test_update_shield_of_securified_persona_cond_set_rola(
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation,
            false,
            || fixture_rtm!("update_shield_of_persona_init_with_R_confirm_with_C")
        )
    }

    #[test]
    fn update_shield_of_securified_persona_init_with_R_confirm_with_T() {
        test_update_shield_of_securified_persona_cond_set_rola(
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion,
            false,
            || fixture_rtm!("update_shield_of_persona_init_with_R_confirm_with_T")
        )
    }

    #[test]
    fn update_shield_of_securified_persona_fails_when_setting_rola_and_without_primary(
    ) {
        let entity_applying_shield = SecurifiedPersona::sample();

        let res = SUT::apply_security_shield_for_securified_entity(
            entity_applying_shield.clone(),
            SecurityStructureOfFactorInstances::sample_other(),
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion,
        );

        assert_eq!(res, None);
    }
}
