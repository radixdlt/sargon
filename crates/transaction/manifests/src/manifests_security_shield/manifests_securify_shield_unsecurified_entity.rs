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
    /// We do NOT top of XRD vault of AccessController - yet!
    /// Host will need to call the function:
    /// `modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_entity_paid_by_account`
    /// after user has selected account to pay in wallet GUI.
    /// (and as usual also call `modify_manifest_lock_fee`)
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

        // Obtain the badge for rola key
        let owner_badge_proof = "rola_key_proof";
        builder = builder.create_proof_from_bucket_of_all(
            owner_badge_bucket,
            owner_badge_proof,
        );
        builder = builder.push_to_auth_zone(owner_badge_proof);

        // Set Rola Key
        builder = TransactionManifest::set_rola_key(
            builder,
            &security_structure_of_factor_instances
                .authentication_signing_factor_instance,
            &entity_address,
        );

        builder = builder.drop_auth_zone_proofs();

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

        // N.B.
        // We do NOT top of XRD vault of AccessController - yet!
        // Host will need to call the function:
        // `modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_entity_paid_by_account`
        // after user has selected account to pay in wallet GUI.
        // (and as usual also call `modify_manifest_lock_fee`)

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

        for fi in security_structure_of_factor_instances
            .unique_all_factor_instances()
            .into_iter()
            .filter_map(|f| f.try_as_hd_factor_instances().ok())
        {
            assert!(expected_manifest_str
                .contains(&PublicKeyHash::hash(fi.public_key()).to_string()));
        }

        assert!(expected_manifest_str.contains(&account.address.to_string()));

        let bob = Account::sample_bob();

        let with_ac_xrd_vault_top_up_by_unsecurified_account = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_entity_paid_by_account(
            bob.clone(),
            AnyUnsecurifiedEntity::new( account.clone()).unwrap(),
            manifest.clone(),
            None,
        );

        let expected_manifest_str =
        fixture_rtm!("create_access_controller_for_account_with_ac_xrd_vault_top_up_by_unsecurified_account");

        manifest_eq(
            with_ac_xrd_vault_top_up_by_unsecurified_account,
            expected_manifest_str,
        );

        let grace_secure = Account::sample_securified_mainnet(
            "Grace",
            6,
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(0),
            || {
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r6(
                HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Account,
                    Hardened::from_local_key_space(6u32, IsSecurified(true)).unwrap(),
                ))
            },
        );

        let with_ac_xrd_vault_top_up_by_securified_account_amount_42 = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_entity_paid_by_account(
            grace_secure.clone(),
            AnyUnsecurifiedEntity::new(account.clone()).unwrap(),
            manifest.clone(),
            Decimal192::from(42),
        );

        let expected_manifest_str =
        fixture_rtm!("create_access_controller_for_account_with_ac_xrd_vault_top_up_by_securified_account_amount_42");

        manifest_eq(
            with_ac_xrd_vault_top_up_by_securified_account_amount_42,
            expected_manifest_str,
        );
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

        for fi in security_structure_of_factor_instances
            .unique_all_factor_instances()
            .into_iter()
            .filter_map(|f| f.try_as_hd_factor_instances().ok())
        {
            assert!(expected_manifest_str
                .contains(&PublicKeyHash::hash(fi.public_key()).to_string()));
        }

        assert!(expected_manifest_str.contains(&persona.address.to_string()));

        let bob = Account::sample_bob();

        let with_ac_xrd_vault_top_up_by_unsecurified_account = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_entity_paid_by_account(
                bob.clone(),
                AnyUnsecurifiedEntity::new(persona.clone()).unwrap(),
                manifest.clone(),
                None,
            );

        let expected_manifest_str =
            fixture_rtm!("create_access_controller_for_persona_with_ac_xrd_vault_top_up_by_unsecurified_account");

        manifest_eq(
            with_ac_xrd_vault_top_up_by_unsecurified_account,
            expected_manifest_str,
        );

        let grace_secure = Account::sample_securified_mainnet(
                "Grace",
                6,
                HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(0),
                || {
                    GeneralRoleWithHierarchicalDeterministicFactorInstances::r6(
                    HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                        CAP26EntityKind::Account,
                        Hardened::from_local_key_space(6u32, IsSecurified(true)).unwrap(),
                    ))
                },
            );

        let with_ac_xrd_vault_top_up_by_securified_account_amount_42 = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_entity_paid_by_account(
                grace_secure.clone(),
                AnyUnsecurifiedEntity::new( persona.clone()).unwrap(),
                manifest.clone(),
                Decimal192::from(42),
            );

        let expected_manifest_str =
            fixture_rtm!("create_access_controller_for_persona_with_ac_xrd_vault_top_up_by_securified_account_amount_42");

        manifest_eq(
            with_ac_xrd_vault_top_up_by_securified_account_amount_42,
            expected_manifest_str,
        );
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
