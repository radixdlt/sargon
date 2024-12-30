use radix_engine_interface::blueprints::access_controller::AccessControllerCreateManifestInput as ScryptoAccessControllerCreateManifestInput;

use crate::prelude::*;

impl TransactionManifest {
    pub fn securify_unsecurified_entity<E: IsEntity>(
        entity: E,
        security_structure_of_factor_instances: SecurityStructureOfFactorInstances,
    ) -> Result<Self> {
        let Ok(unsecurified) = entity.security_state().into_unsecured() else {
            return Err(CommonError::CannotSecurifyEntityItIsAlreadySecurifiedAccordingToProfile);
        };

        if unsecurified.provisional_securified_config.is_some() {
            return Err(
                CommonError::CannotSecurifyEntityHasProvisionalSecurityConfig,
            );
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
    fn cannot_securify_entity_it_is_already_securified_according_to_profile() {
        let account = Account::sample_at(2);
        assert!(account.is_securified());
        let res = TransactionManifest::securify_unsecurified_entity(
            account,
            SecurityStructureOfFactorInstances::sample(),
        );
        assert_eq!(res, Err(CommonError::CannotSecurifyEntityItIsAlreadySecurifiedAccordingToProfile));
    }

    #[test]
    fn cannot_securify_entity_with_provisional() {
        let mut account = Account::sample_alice();
        assert!(!account.is_securified());
        account
            .security_state
            .set_provisional(ProvisionalSecurifiedConfig::ShieldSelected {
                value: SecurityStructureID::sample(),
            })
            .unwrap();
        let res = TransactionManifest::securify_unsecurified_entity(
            account,
            SecurityStructureOfFactorInstances::sample(),
        );
        assert_eq!(
            res,
            Err(CommonError::CannotSecurifyEntityHasProvisionalSecurityConfig)
        );
    }

    #[test]
    fn test_securify_unsecurified_account() {
        let expected_manifest_str = include_str!(concat!(
            env!("FIXTURES_TX"),
            "create_access_controller_for_account.rtm"
        ));
        let entity = Account::sample();
        let security_structure_of_factor_instances =
            SecurityStructureOfFactorInstances::sample();
        let manifest = TransactionManifest::securify_unsecurified_entity(
            entity.clone(),
            security_structure_of_factor_instances.clone(),
        )
        .unwrap();
        manifest_eq(manifest, expected_manifest_str);
        assert!(expected_manifest_str.contains("securify"));
        assert!(expected_manifest_str.contains(
            &security_structure_of_factor_instances
                .timed_recovery_delay_in_minutes()
                .to_string()
        ));

        for fi in security_structure_of_factor_instances
            .unique_all_factor_instances()
            .into_iter()
            .filter_map(|f| f.try_as_hd_factor_instances().ok())
        {
            assert!(expected_manifest_str
                .contains(&PublicKeyHash::hash(fi.public_key()).to_string()));
        }

        assert!(expected_manifest_str.contains(&entity.address.to_string()));
    }

    #[test]
    fn test_securify_unsecurified_persona() {
        let expected_manifest_str = include_str!(concat!(
            env!("FIXTURES_TX"),
            "create_access_controller_for_persona.rtm"
        ));
        let entity = Persona::sample_other();
        let security_structure_of_factor_instances =
            SecurityStructureOfFactorInstances::sample_other();
        let manifest = TransactionManifest::securify_unsecurified_entity(
            entity.clone(),
            security_structure_of_factor_instances.clone(),
        )
        .unwrap();
        manifest_eq(manifest, expected_manifest_str);

        assert!(expected_manifest_str.contains("securify"));
        assert!(expected_manifest_str.contains(
            &security_structure_of_factor_instances
                .timed_recovery_delay_in_minutes()
                .to_string()
        ));

        for fi in security_structure_of_factor_instances
            .unique_all_factor_instances()
            .into_iter()
            .filter_map(|f| f.try_as_hd_factor_instances().ok())
        {
            assert!(expected_manifest_str
                .contains(&PublicKeyHash::hash(fi.public_key()).to_string()));
        }

        assert!(expected_manifest_str.contains(&entity.address.to_string()));
    }

    #[test]
    fn test_mismatch_entity_kind_account_persona() {
        let manifest = TransactionManifest::securify_unsecurified_entity(
            Account::sample_other(),
            SecurityStructureOfFactorInstances::sample_other(),
        );
        assert_eq!(manifest, Err(CommonError::SecurityStructureOfFactorInstancesEntityDiscrepancyInEntityKind { entity_kind_of_entity: CAP26EntityKind::Account.to_string(), entity_kind_of_factor_instances: CAP26EntityKind::Identity.to_string() }));
    }

    #[test]
    fn test_mismatch_entity_kind_persona_account() {
        let manifest = TransactionManifest::securify_unsecurified_entity(
            Persona::sample_other(),
            SecurityStructureOfFactorInstances::sample(),
        );
        assert_eq!(manifest, Err(CommonError::SecurityStructureOfFactorInstancesEntityDiscrepancyInEntityKind { entity_kind_of_entity: CAP26EntityKind::Identity.to_string(), entity_kind_of_factor_instances: CAP26EntityKind::Account.to_string() }));
    }
}
