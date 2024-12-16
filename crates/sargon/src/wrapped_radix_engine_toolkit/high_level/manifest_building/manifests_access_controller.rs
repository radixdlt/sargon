use radix_engine_interface::blueprints::access_controller::AccessControllerCreateManifestInput as ScryptoAccessControllerCreateManifestInput;

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

        Self::_securify_unsecurified_entity(
            Into::<AddressOfAccountOrPersona>::into(entity.address()),
            security_structure_of_factor_instances,
        )
    }

    fn _securify_unsecurified_entity(
        entity_address: AddressOfAccountOrPersona,
        security_structure_of_factor_instances: SecurityStructureOfFactorInstances,
    ) -> Result<Self> {
        security_structure_of_factor_instances
            .assert_has_entity_kind(entity_address.get_entity_kind())?;

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

        let manifest = TransactionManifest::sargon_built(
            builder,
            entity_address.network_id(),
        );

        Ok(manifest)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_securify_unsecurified_account() {
        let expected_manifest = include_str!(concat!(
            env!("FIXTURES_TX"),
            "create_access_controller_for_account.rtm"
        ));
        let manifest = TransactionManifest::securify_unsecurified_entity(
            Account::sample(),
            SecurityStructureOfFactorInstances::sample(),
        )
        .unwrap();
        manifest_eq(manifest, expected_manifest);
    }

    #[test]
    fn test_securify_unsecurified_persona() {
        let expected_manifest = include_str!(concat!(
            env!("FIXTURES_TX"),
            "create_access_controller_for_persona.rtm"
        ));
        let manifest = TransactionManifest::securify_unsecurified_entity(
            Persona::sample_other(),
            SecurityStructureOfFactorInstances::sample_other(),
        )
        .unwrap();
        manifest_eq(manifest, expected_manifest);
    }

    #[test]
    fn test_mismatch_entity_kind_account_persona() {
        let manifest = TransactionManifest::securify_unsecurified_entity(
            Account::sample_other(),
            SecurityStructureOfFactorInstances::sample_other(),
        );
        assert_eq!(manifest, Err(CommonError::SecurityStructureOfFactorInstancesEntityDiscrepancyInEntityKind { entity_kind_of_entity: CAP26EntityKind::Account, entity_kind_of_factor_instances: CAP26EntityKind::Identity }));
    }

    #[test]
    fn test_mismatch_entity_kind_persona_account() {
        let manifest = TransactionManifest::securify_unsecurified_entity(
            Persona::sample_other(),
            SecurityStructureOfFactorInstances::sample(),
        );
        assert_eq!(manifest, Err(CommonError::SecurityStructureOfFactorInstancesEntityDiscrepancyInEntityKind { entity_kind_of_entity: CAP26EntityKind::Identity, entity_kind_of_factor_instances: CAP26EntityKind::Account }));
    }
}
