use profile_supporting_types::AnyUnsecurifiedEntity;
use radix_common::prelude::ACCESS_CONTROLLER_PACKAGE as SCRYPTO_ACCESS_CONTROLLER_PACKAGE;
use radix_engine_interface::blueprints::access_controller::{
    AccessControllerCreateManifestInput as ScryptoAccessControllerCreateManifestInput,
    ACCESS_CONTROLLER_BLUEPRINT as SCRYPTO_ACCESS_CONTROLLER_BLUEPRINT,
    ACCESS_CONTROLLER_CREATE_IDENT as SCRYPTO_ACCESS_CONTROLLER_CREATE_IDENT,
};

use crate::prelude::*;

pub trait TransactionManifestSecurifyUnsecurifiedEntity:
    Sized + TransactionManifestSetRolaKey
{
    fn apply_security_shield_for_unsecurified_entity(
        unsecurified_entity: AnyUnsecurifiedEntity,
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
    ) -> Result<Self>;
}

impl TransactionManifestSecurifyUnsecurifiedEntity for TransactionManifest {
    /// Securifies the entity, creating an access controller for it, and does
    /// nothing else, so that it's classified as `EntitySecurify` and shows
    /// all it does. Once the access controller exists, its recovery fee
    /// vault is topped up and the entity's ROLA key set, each in a
    /// transaction of its own: see
    /// `TransactionManifest::top_up_access_controller_xrd_vault` and
    /// `TransactionManifest::set_rola_key_of_securified_entity`.
    fn apply_security_shield_for_unsecurified_entity(
        unsecurified_entity: AnyUnsecurifiedEntity,
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
    ) -> Result<Self> {
        let entity_address = unsecurified_entity.address();

        security_structure_of_factor_instances
            .assert_has_entity_kind(entity_address.get_entity_kind())?;

        // Securify the entity which will return an entity owner badge onto the worktop.
        let (mut builder, owner_badge_bucket) = Self::put_owner_badge_in_bucket(
            ScryptoTransactionManifestBuilder::new(),
            &unsecurified_entity,
        );

        // Create an access controller for the entity.
        builder = {
            let access_controller_reservation_identifier =
                "access_controller_reservation";

            builder = builder.allocate_global_address(
                SCRYPTO_ACCESS_CONTROLLER_PACKAGE,
                SCRYPTO_ACCESS_CONTROLLER_BLUEPRINT,
                access_controller_reservation_identifier,
                "access_controller_named_address",
            );

            let access_controller_address_reservation = builder
                .address_reservation(access_controller_reservation_identifier);

            let timed_recovery_delay_in_minutes =
                &security_structure_of_factor_instances
                    .timed_recovery_delay_in_minutes();

            let rule_set = ScryptoRuleSet::from(
                security_structure_of_factor_instances
                    .matrix_of_factors
                    .clone(),
            );

            builder.call_function(
                SCRYPTO_ACCESS_CONTROLLER_PACKAGE,
                SCRYPTO_ACCESS_CONTROLLER_BLUEPRINT,
                SCRYPTO_ACCESS_CONTROLLER_CREATE_IDENT,
                ScryptoAccessControllerCreateManifestInput {
                    controlled_asset: owner_badge_bucket,
                    rule_set: rule_set.into(),
                    timed_recovery_delay_in_minutes: Some(
                        *timed_recovery_delay_in_minutes,
                    ),
                    address_reservation: Some(
                        access_controller_address_reservation,
                    ),
                },
            )
        };

        let manifest = TransactionManifest::sargon_built(
            builder,
            entity_address.network_id(),
        );

        Ok(manifest)
    }
}

#[cfg(test)]
mod tests {

    use prelude::fixture_rtm;

    use super::*;

    #[test]
    fn test_securify_unsecurified_account() {
        let expected_manifest_str =
            fixture_rtm!("create_access_controller_for_account");
        let account = Account::sample();
        let security_structure_of_factor_instances =
            SecurityStructureOfFactorInstances::sample();

        let manifest =
            TransactionManifest::apply_security_shield_for_unsecurified_entity(
                AnyUnsecurifiedEntity::new(account.clone()).unwrap(),
                security_structure_of_factor_instances.clone(),
            )
            .unwrap();
        manifest_eq(manifest.clone(), expected_manifest_str);
        assert!(expected_manifest_str.contains("securify"));
        assert!(expected_manifest_str.contains(
            &security_structure_of_factor_instances
                .timed_recovery_delay_in_minutes()
                .to_string()
        ));

        // The rules hold every factor instance but the ROLA key's, which is
        // set in a transaction of its own once the entity is securified.
        let rola_key = &security_structure_of_factor_instances
            .authentication_signing_factor_instance;
        for fi in security_structure_of_factor_instances
            .unique_all_factor_instances()
            .into_iter()
            .filter_map(|f| f.try_as_hd_factor_instances().ok())
        {
            let hash = PublicKeyHash::hash(fi.public_key()).to_string();
            assert_eq!(expected_manifest_str.contains(&hash), &fi != rola_key);
        }

        assert!(expected_manifest_str.contains(&account.address.to_string()));
    }

    #[test]
    fn test_securify_unsecurified_persona() {
        let expected_manifest_str =
            fixture_rtm!("create_access_controller_for_persona");
        let persona = Persona::sample_other();
        let security_structure_of_factor_instances =
            SecurityStructureOfFactorInstances::sample_other();

        let manifest =
            TransactionManifest::apply_security_shield_for_unsecurified_entity(
                AnyUnsecurifiedEntity::new(persona.clone()).unwrap(),
                security_structure_of_factor_instances.clone(),
            )
            .unwrap();
        manifest_eq(manifest.clone(), expected_manifest_str);
        assert!(expected_manifest_str.contains("securify"));
        assert!(expected_manifest_str.contains(
            &security_structure_of_factor_instances
                .timed_recovery_delay_in_minutes()
                .to_string()
        ));

        // The rules hold every factor instance but the ROLA key's, which is
        // set in a transaction of its own once the entity is securified.
        let rola_key = &security_structure_of_factor_instances
            .authentication_signing_factor_instance;
        for fi in security_structure_of_factor_instances
            .unique_all_factor_instances()
            .into_iter()
            .filter_map(|f| f.try_as_hd_factor_instances().ok())
        {
            let hash = PublicKeyHash::hash(fi.public_key()).to_string();
            assert_eq!(expected_manifest_str.contains(&hash), &fi != rola_key);
        }

        assert!(expected_manifest_str.contains(&persona.address.to_string()));
    }

    #[test]
    fn test_mismatch_entity_kind_account_persona() {
        let manifest =
            TransactionManifest::apply_security_shield_for_unsecurified_entity(
                AnyUnsecurifiedEntity::new(Account::sample_other()).unwrap(),
                SecurityStructureOfFactorInstances::sample_other(),
            );
        assert_eq!(manifest, Err(CommonError::SecurityStructureOfFactorInstancesEntityDiscrepancyInEntityKind { entity_kind_of_entity: CAP26EntityKind::Account.to_string(), entity_kind_of_factor_instances: CAP26EntityKind::Identity.to_string() }));
    }

    #[test]
    fn test_mismatch_entity_kind_persona_account() {
        let manifest =
            TransactionManifest::apply_security_shield_for_unsecurified_entity(
                AnyUnsecurifiedEntity::new(Persona::sample_other()).unwrap(),
                SecurityStructureOfFactorInstances::sample(),
            );
        assert_eq!(manifest, Err(CommonError::SecurityStructureOfFactorInstancesEntityDiscrepancyInEntityKind { entity_kind_of_entity: CAP26EntityKind::Identity.to_string(), entity_kind_of_factor_instances: CAP26EntityKind::Account.to_string() }));
    }
}
