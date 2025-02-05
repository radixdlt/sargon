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
    ) -> TransactionManifest;
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
    /// * `modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_securified_entity_paid_by_account`
    ///
    /// For timed confirmation - much later (`timed_recovery_delay_in_minutes` later ) the
    /// host app will need to call `confirm_timed_recovery`
    fn apply_security_shield_for_securified_entity(
        securified_entity: impl Into<AnySecurifiedEntity>,
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
        roles_combination: RolesExercisableInTransactionManifestCombination,
    ) -> Self {
        let securified_entity = securified_entity.into();
        let kind = roles_combination;
        let entity_address = securified_entity.entity.address();

        security_structure_of_factor_instances
            .assert_has_entity_kind(entity_address.get_entity_kind()).expect("Shouldn't have used wrong FactorInstance for entity - apply_security_shield_with_id_to_entities has some bug.");

        let mut builder = ScryptoTransactionManifestBuilder::new();

        let set_rola = |builder: ScryptoTransactionManifestBuilder| -> ScryptoTransactionManifestBuilder {
            TransactionManifest::set_rola_key(
                builder,
                &security_structure_of_factor_instances
                    .authentication_signing_factor_instance,
                &entity_address,
            )
        };

        let order_of_instruction_setting_rola = kind
            .order_of_instruction_setting_rola(
                &security_structure_of_factor_instances,
                &securified_entity,
            );

        let access_controller_address = securified_entity
            .securified_entity_control
            .access_controller_address();

        let factors_and_time_input = &AccessControllerFactorsAndTimeInput::new(
            &security_structure_of_factor_instances,
        );

        match order_of_instruction_setting_rola {
            OrderOfInstructionSettingRolaKey::BeforeInitRecovery => {
                builder = set_rola(builder);
            }
            OrderOfInstructionSettingRolaKey::AfterQuickConfirm | OrderOfInstructionSettingRolaKey::NotNeeded | OrderOfInstructionSettingRolaKey::MustSetInFutureTxForConfirmRecovery => {
                // Do nothing for now
            }
        }

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

        match order_of_instruction_setting_rola {
            OrderOfInstructionSettingRolaKey::AfterQuickConfirm => {
                builder = set_rola(builder);
            }
            OrderOfInstructionSettingRolaKey::BeforeInitRecovery | OrderOfInstructionSettingRolaKey::NotNeeded => {
                // nothing to do
            } OrderOfInstructionSettingRolaKey::MustSetInFutureTxForConfirmRecovery => {
                info!("Do not forget to set Rola key in future transaction for Confirm Recovery");
            }
        }

        // N.B.
        // We will not lock fee against the XRD vault yet - we will do that
        // later when we have made a preview/dry-run of the `manifest` to get
        // the estimated fee to lock, by calling `modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller`
        //
        // Furthermore:
        // We do NOT top of XRD vault of AccessController - yet!
        // Host will need to call the function:
        // `modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_securified_entity_paid_by_account`
        // after user has selected account to pay in wallet GUI. And also call
        // `modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller`

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
    use profile_supporting_types::{SecurifiedAccount, SecurifiedPersona};
    use radix_transactions::manifest::{
        CallMetadataMethod, CallMethod, ManifestInstruction,
    };
    use sbor::SborEnum;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn update_shield_of_securified_account_with_top_up_where_payer_is_entity_applying_shield(
    ) {
        let entity_applying_shield = SecurifiedAccount::sample();
        assert_eq!(entity_applying_shield.securified_entity_control.access_controller_address().to_string(), "accesscontroller_rdx1cdgcq7yqee9uhyqrsp9kgud3a7h4dvz3dqmx26ws5dmjsu7g3zg23g");

        let manifest = SUT::apply_security_shield_for_securified_entity(
            entity_applying_shield.clone(),
            SecurityStructureOfFactorInstances::sample(),
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation,
        );

        let expected_manifest_str =
            fixture_rtm!("update_shield_of_account_init_with_P_confirm_with_C");
        manifest_eq(manifest.clone(), expected_manifest_str);
        assert!(expected_manifest_str.contains("accesscontroller_rdx1cdgcq7yqee9uhyqrsp9kgud3a7h4dvz3dqmx26ws5dmjsu7g3zg23g"));

        let manifest = SUT::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_securified_entity_paid_by_account(entity_applying_shield.clone(), entity_applying_shield.clone(), manifest.clone(), Decimal192::ten()).unwrap();

        let expected_manifest_str =
        fixture_rtm!("update_shield_of_account_init_with_P_confirm_with_C_with_top_up_where_payer_is_entity_applying_shield");
        manifest_eq(manifest.clone(), expected_manifest_str);

        let manifest = SUT::modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(manifest, Decimal192::nine(), entity_applying_shield);

        let expected_manifest_str =
        fixture_rtm!("update_shield_of_account_init_with_P_confirm_with_C_with_top_up_where_payer_is_entity_applying_shield_with_xrd_lock");
        manifest_eq(manifest, expected_manifest_str);
    }

    fn test_update_shield_of_securified_persona<'a>(
        roles: RolesExercisableInTransactionManifestCombination,
        rtm: impl Fn() -> &'a str,
    ) -> Vec<u8> {
        let entity_applying_shield = SecurifiedPersona::sample();
        let instances = SecurityStructureOfFactorInstances::sample_other();

        let manifest = SUT::apply_security_shield_for_securified_entity(
            entity_applying_shield.clone(),
            instances,
            roles,
        );
        let expected_manifest_str = rtm();
        manifest_eq(manifest.clone(), expected_manifest_str);
        manifest
            .instructions()
            .iter()
            .map(|i| i.get_discriminator())
            .collect_vec()
    }

    #[test]
    fn update_shield_of_securified_persona_init_with_R_confirm_with_P() {
        let instruction_discriminants = test_update_shield_of_securified_persona(
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary,
            || fixture_rtm!("update_shield_of_persona_init_with_R_confirm_with_P")
        );
        assert_eq!(
            instruction_discriminants,
            vec![
                CallMethod::ID,         // init
                CallMethod::ID,         // quick confirm
                CallMetadataMethod::ID, // set ROLA key
            ]
        );
    }

    #[test]
    fn update_shield_of_securified_persona_init_with_P_confirm_with_C() {
        let instruction_discriminants = test_update_shield_of_securified_persona(
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation,
            || fixture_rtm!("update_shield_of_persona_init_with_P_confirm_with_C")
        );
        assert_eq!(
            instruction_discriminants,
            vec![
                CallMetadataMethod::ID, // set ROLA key
                CallMethod::ID,         // init
                CallMethod::ID,         // quick confirm
            ],
             "Expected to FIRST set ROLA key and THEN init - since we can set it using existing factors."
        );
    }

    #[test]
    fn update_shield_of_securified_persona_init_with_P_confirm_with_T() {
        let instruction_discriminants = test_update_shield_of_securified_persona(
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion,
            || fixture_rtm!("update_shield_of_persona_init_with_P_confirm_with_T")
        );
        assert_eq!(
            instruction_discriminants,
            vec![
                CallMethod::ID // init 
            ],
        );
    }

    #[test]
    fn update_shield_of_securified_persona_init_with_R_confirm_with_C() {
        let instruction_discriminants = test_update_shield_of_securified_persona(
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation,
            || fixture_rtm!("update_shield_of_persona_init_with_R_confirm_with_C")
        );
        assert_eq!(
            instruction_discriminants,
            vec![
                CallMethod::ID,         // init
                CallMethod::ID,         // quick confirm
                CallMetadataMethod::ID, // set ROLA key
            ]
        );
    }

    #[test]
    fn update_shield_of_securified_persona_init_with_R_confirm_with_T() {
        let instruction_discriminants = test_update_shield_of_securified_persona(
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion,
            || fixture_rtm!("update_shield_of_persona_init_with_R_confirm_with_T")
        );
        assert_eq!(
            instruction_discriminants,
            vec![CallMethod::ID],
            "init with R complete with T should not set rola key - since we cannot"
        );
    }
}
