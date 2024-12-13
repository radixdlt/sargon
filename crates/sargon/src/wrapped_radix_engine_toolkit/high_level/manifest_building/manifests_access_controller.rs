use radix_engine_interface::blueprints::{
    access_controller::AccessControllerCreateManifestInput as ScryptoAccessControllerCreateManifestInput,
    account::ACCOUNT_SECURIFY_IDENT as SCRYPTO_ACCOUNT_SECURIFY_IDENT,
    identity::IDENTITY_SECURIFY_IDENT as SCRYPTO_IDENTITY_SECURIFY_IDENT,
};

use crate::prelude::*;

impl TransactionManifest {
    pub fn securify_unsecurified_entity<E: IsEntity>(
        entity: E,
        security_structure_of_factor_instances: SecurityStructureOfFactorInstances,
    ) -> Result<Self> {
        let Ok(unsecurified) = entity.security_state().into_unsecured() else {
            return Err(CommonError::Unknown);
        };

        if unsecurified.provisional.is_some() {
            return Err(CommonError::Unknown);
        };

        Ok(Self::_securify_unsecurified_entity(
            Into::<AddressOfAccountOrPersona>::into(entity.address()),
            security_structure_of_factor_instances,
        ))
    }

    fn _securify_unsecurified_entity(
        entity_address: AddressOfAccountOrPersona,
        security_structure_of_factor_instances: SecurityStructureOfFactorInstances,
    ) -> Self {
        let (security_entity_identifier, owner_badge) =
            if entity_address.is_identity() {
                (
                    SCRYPTO_IDENTITY_SECURIFY_IDENT,
                    SCRYPTO_IDENTITY_OWNER_BADGE,
                )
            } else {
                (SCRYPTO_ACCOUNT_SECURIFY_IDENT, SCRYPTO_ACCOUNT_OWNER_BADGE)
            };

        let mut builder = ScryptoTransactionManifestBuilder::new();
        let bucket_factory = BucketFactory::default();

        // Securify the entity which will return an entity owner badge onto the worktop.
        let owner_badge_bucket = &{
            builder = builder.call_method(
                &entity_address,
                security_entity_identifier,
                (),
            );

            // Create a bucket out of the entity owner badge.
            let owner_badge_bucket = bucket_factory.next();
            builder =
                builder.take_from_worktop(owner_badge, 1, &owner_badge_bucket);
            owner_badge_bucket
        };

        // Create an access controller for the entity.
        {
            let timed_recovery_delay_in_minutes =
                security_structure_of_factor_instances
                    .timed_recovery_delay_in_minutes();
            let rule_set = ScryptoRuleSet::from(
                security_structure_of_factor_instances.matrix_of_factors,
            );

            builder = builder.create_access_controller(
                owner_badge_bucket,
                rule_set.primary_role,
                rule_set.recovery_role,
                rule_set.confirmation_role,
                Some(timed_recovery_delay_in_minutes),
            );
        }

        // Set Rola Key
        {
            let rola_key_hash = PublicKeyHash::hash(
                security_structure_of_factor_instances
                    .authentication_signing_factor_instance
                    .public_key(),
            );
            let owner_key_hashes = vec![rola_key_hash];
            builder = Self::set_owner_keys_hashes_on_builder(
                &entity_address,
                owner_key_hashes,
                builder,
            );
        }

        TransactionManifest::sargon_built(builder, entity_address.network_id())
    }
}
