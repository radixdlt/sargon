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
        apply_shield_manifest_kind: TransactionManifestApplySecurityShieldKind,
    ) -> Result<TransactionManifest>;
}

impl TransactionManifestSecurifySecurifiedEntity for TransactionManifest {
    fn apply_security_shield_for_securified_entity(
        securified_entity: AnySecurifiedEntity,
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
        apply_shield_manifest_kind: TransactionManifestApplySecurityShieldKind,
    ) -> Result<Self> {
        let kind = apply_shield_manifest_kind;
        let entity_address = securified_entity.entity.address();

        // ACCESS_CONTROLLER_CREATE_PROOF_IDENT
        let mut builder = TransactionManifest::produce_owner_badge(
            ScryptoTransactionManifestBuilder::new(),
            &securified_entity.entity,
        );

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

        // CONFIRM RECOVERY
        // TODO: for timed, should we really call it here, now? Should
        // we not call it AFTER the time has elapsed???
        let (confirm_method, confirm_input) =
            kind.input_for_confirm(factors_and_time_input);
        builder = builder.call_method(
            access_controller_address.scrypto(),
            confirm_method,
            (confirm_input.deref(),),
        );

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
        // We do NOT top of XRD vault of AccessController - yet!
        // Host will need to call the function:
        // `modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_paid_by_account`
        // after user has selected account to pay in wallet GUI.
        // (and as usual also call `modify_manifest_lock_fee`)

        Ok(manifest)
    }
}
