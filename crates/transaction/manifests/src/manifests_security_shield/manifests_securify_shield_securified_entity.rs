#![allow(dead_code)]
use crate::prelude::*;
use std::ops::Deref;

use profile_supporting_types::AnySecurifiedEntity;

pub trait TransactionManifestSecurifySecurifiedEntity:
    TransactionManifestSetRolaKey
{
    fn apply_security_shield_for_securified_entity(
        securified_entity: AnySecurifiedEntity,
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
        apply_shield_manifest_kind: RolesExercisableInTransactionManifestCombination,
    ) -> Result<TransactionManifest>;
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
    /// * `modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_paid_by_account`
    ///
    /// For timed confirmation - much later (`timed_recovery_delay_in_minutes` later ) the
    /// host app will need to call `confirm_timed_recovery`
    fn apply_security_shield_for_securified_entity(
        securified_entity: AnySecurifiedEntity,
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
        apply_shield_manifest_kind: RolesExercisableInTransactionManifestCombination,
    ) -> Result<Self> {
        let kind = apply_shield_manifest_kind;
        let entity_address = securified_entity.entity.address();

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
                return Err(CommonError::Unknown); // TODO: new error variant
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
        // `modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_paid_by_account`
        // after user has selected account to pay in wallet GUI.
        // (and as usual also call `modify_manifest_lock_fee`)

        Ok(manifest)
    }
}
