use profile_supporting_types::AnyUnsecurifiedEntity;
use radix_common::prelude::{
    ManifestGlobalAddress,
    ACCESS_CONTROLLER_PACKAGE as SCRYPTO_ACCESS_CONTROLLER_PACKAGE,
};
use radix_engine_interface::blueprints::{
    access_controller::{
        AccessControllerContributeRecoveryFeeManifestInput,
        AccessControllerCreateManifestInput as ScryptoAccessControllerCreateManifestInput,
        ACCESS_CONTROLLER_BLUEPRINT as SCRYPTO_ACCESS_CONTROLLER_BLUEPRINT,
        ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT as SCRYPTO_ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT,
        ACCESS_CONTROLLER_CREATE_IDENT as SCRYPTO_ACCESS_CONTROLLER_CREATE_IDENT,
        ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
    },
    account::AccountSecurifyManifestInput as ScryptoAccountSecurifyManifestInput,
};
use radix_transactions::prelude::ManifestBuilder;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TransactionManifestApplySecurityShieldUnsecurifiedInput {
    pub security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
}

impl TransactionManifestApplySecurityShieldUnsecurifiedInput {
    pub fn new(
        security_structure_of_factor_instances:
        SecurityStructureOfFactorInstances,
    ) -> Self {
        Self {
            security_structure_of_factor_instances,
        }
    }
}

pub trait BuilderFromManifest {
    fn with_instructions(
        instructions: impl IntoIterator<Item = ScryptoInstruction>,
    ) -> ManifestBuilder;

    fn with_manifest(manifest: TransactionManifest) -> ManifestBuilder {
        Self::with_instructions(manifest.instructions().clone())
    }
}

impl BuilderFromManifest for ManifestBuilder {
    fn with_instructions(
        instructions: impl IntoIterator<Item = ScryptoInstruction>,
    ) -> ManifestBuilder {
        instructions.into_iter().fold(
            ManifestBuilder::new(),
            |builder, instruction| {
                builder.add_instruction_advanced(instruction).0
            },
        )
    }
}

pub trait TransactionManifestSecurifyEntity: Sized {
    fn apply_security_shield_for_unsecurified_entity(
        unsecurified_entity: AnyUnsecurifiedEntity,
        input: TransactionManifestApplySecurityShieldUnsecurifiedInput,
    ) -> Result<Self>;

    /// A method modifying manifests which applies security shield. We
    /// The `manifest` which applies the security shield could not include
    /// the instructions which we append in this method since the host did
    /// not know the `payer` at the time of creating that manifest. The host
    /// will call this method when the `payer` is known, which appends instructions
    /// at the end of `manifest` for topping the XRD vault of the access controller
    /// with `top_up_amount` many XRD.
    ///
    /// N.B. We will call this method for both when `entity_applying_shield` is
    /// securified or unsecurified. In the case of unsecurified entity we will use
    /// the address reservation of `apply_security_shield_for_unsecurified_entity`
    /// (`ACCESS_CONTROLLER_ADDRESS_RESERVATION_NAME`) which we cannot access by id
    /// since Radix Engine discard those ids and uses internal ones, instead we need
    /// to use `ManifestGlobalAddress::Named(ScryptoManifestNamedAddress(0))`.
    /// If `entity_applying_shield` is securified we will use the address of the
    /// already existing access controller.
    ///
    /// If `payer` is securified we will also add a `create_proof` instruction for
    /// authenticating the withdrawal of XRD from the payer.
    ///
    /// If `top_up_amount` is `None` the `XRD_TO_AC_VAULT_FIRST_TOP_UP` will be used.
    /// We allow to pass amount so that we can top of with more or less based on
    /// token balance of `payer` and current balance of the access controller (when
    /// we use this method for securified entities.)
    fn modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_paid_by_account(
        payer: Account,
        // TODO: remove `entity_applying_shield`, this should be read out from the manifest in a throwing function, `manifest.get_address_of_entity_applying_shield()` or similar which Omar need to provide us with, oh well we need the account here, so elsewhere, in SargonOS where we have access to Profile we would call `manifest.get_address_of_entity_applying_shield` and then lookup the entity.
        entity_applying_shield: Account,
        manifest: TransactionManifest,
        top_up_amount: impl Into<Option<Decimal192>>,
    ) -> TransactionManifest {
        let address_of_paying_account = payer.address();
        let mut builder = ManifestBuilder::with_manifest(manifest);

        let address_of_access_controller_to_top_up =
            match entity_applying_shield.security_state() {
                EntitySecurityState::Securified { value: sec } => {
                    ManifestGlobalAddress::Static(
                        sec.access_controller_address.scrypto(),
                    )
                }
                EntitySecurityState::Unsecured { .. } => {
                    // We are securifying an unsecurified account => use the
                    // address reservation at index 0,
                    // which `apply_security_shield_for_unsecurified_entity`
                    // is using to create the access controller address
                    ManifestGlobalAddress::Named(ScryptoManifestNamedAddress(0))
                }
            };

        let address_of_access_controller_of_payer = {
            match payer.security_state() {
                EntitySecurityState::Securified { value: sec } => {
                    Some(ManifestGlobalAddress::Static(
                        sec.access_controller_address.scrypto(),
                    ))
                }
                EntitySecurityState::Unsecured { .. } => {
                    // No access controller to create proof for
                    None
                }
            }
        };

        // Add `create_proof` instruction for the access controller
        if let Some(address_of_access_controller_of_payer) =
            address_of_access_controller_of_payer
        {
            builder = builder.call_method(
                address_of_access_controller_of_payer,
                ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
                (),
            );
        }

        let top_up_amount = top_up_amount
            .into()
            .map(ScryptoDecimal192::from)
            .unwrap_or(XRD_TO_AC_VAULT_FIRST_TOP_UP);

        // Add withdraw XRD instruction
        builder = builder.withdraw_from_account(
            address_of_paying_account.scrypto(),
            XRD,
            top_up_amount,
        );

        // Deposit XRD into the access controllers XRD vault
        // ... by first taking the XRD from the work top
        let xrd_to_top_up_ac_vault_bucket_name =
            "xrd_to_top_up_ac_vault_bucket";
        builder = builder.take_from_worktop(
            XRD,
            top_up_amount,
            xrd_to_top_up_ac_vault_bucket_name,
        );
        let xrd_to_top_up_ac_vault_bucket =
            builder.bucket(xrd_to_top_up_ac_vault_bucket_name);

        // ... then deposit to XRD vault of access controller
        builder = builder.call_method(
            address_of_access_controller_to_top_up,
            SCRYPTO_ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT,
            AccessControllerContributeRecoveryFeeManifestInput {
                bucket: xrd_to_top_up_ac_vault_bucket,
            },
        );

        TransactionManifest::sargon_built(
            builder,
            address_of_paying_account.network_id(),
        )
    }

    fn set_rola_key(
        builder: ManifestBuilder,
        authentication_signing_factor_instance: &HierarchicalDeterministicFactorInstance,
        entity_address: &AddressOfAccountOrPersona,
    ) -> ManifestBuilder;
}

const XRD_TO_AC_VAULT_FIRST_TOP_UP: ScryptoDecimal192 =
    ScryptoDecimal192::ONE_HUNDRED;

impl TransactionManifestSecurifyEntity for TransactionManifest {
    fn set_rola_key(
        builder: ManifestBuilder,
        authentication_signing_factor_instance:
        &HierarchicalDeterministicFactorInstance,
        entity_address: &AddressOfAccountOrPersona,
    ) -> ManifestBuilder {
        let rola_key_hash = PublicKeyHash::hash(
            authentication_signing_factor_instance.public_key(),
        );
        let owner_key_hashes = vec![rola_key_hash];
        Self::set_owner_keys_hashes_on_builder(
            entity_address,
            owner_key_hashes,
            builder,
        )
    }

    /// We do NOT top of XRD vault of AccessController - yet!
    /// Host will need to call the function:
    /// `modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_paid_by_account`
    /// after user has selected account to pay in wallet GUI.
    /// (and as usual also call `modify_manifest_lock_fee`)
    fn apply_security_shield_for_unsecurified_entity(
        unsecurified_entity: AnyUnsecurifiedEntity,
        input: TransactionManifestApplySecurityShieldUnsecurifiedInput,
    ) -> Result<Self> {
        let entity_address = unsecurified_entity.address();
        let TransactionManifestApplySecurityShieldUnsecurifiedInput {
            security_structure_of_factor_instances,
        } = input.clone();

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

        // Securify the entity which will return an entity owner badge onto the worktop.
        let owner_badge_bucket_name = "owner_badge_bucket";
        {
            builder = builder.call_method(
                &entity_address,
                security_entity_identifier,
                ScryptoAccountSecurifyManifestInput {},
            );

            // Create a bucket out of the entity owner badge.
            builder = builder.take_from_worktop(
                owner_badge,
                1,
                owner_badge_bucket_name,
            );
        };

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

            let owner_badge_bucket = builder.bucket(owner_badge_bucket_name);

            builder.call_function(
                SCRYPTO_ACCESS_CONTROLLER_PACKAGE,
                SCRYPTO_ACCESS_CONTROLLER_BLUEPRINT,
                SCRYPTO_ACCESS_CONTROLLER_CREATE_IDENT,
                ScryptoAccessControllerCreateManifestInput {
                    controlled_asset: owner_badge_bucket,
                    rule_set,
                    timed_recovery_delay_in_minutes: Some(
                        *timed_recovery_delay_in_minutes,
                    ),
                    address_reservation: Some(
                        access_controller_address_reservation,
                    ),
                },
            )
        };

        // Set Rola Key
        builder = TransactionManifest::set_rola_key(
            builder,
            &security_structure_of_factor_instances
                .authentication_signing_factor_instance,
            &entity_address,
        );

        let manifest = TransactionManifest::sargon_built(
            builder,
            entity_address.network_id(),
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

#[cfg(test)]
mod tests {

    use prelude::fixture_rtm;

    use super::*;

    #[test]
    fn test_securify_unsecurified_account() {
        let expected_manifest_str =
            fixture_rtm!("create_access_controller_for_account");
        let entity = Account::sample();
        let security_structure_of_factor_instances =
            SecurityStructureOfFactorInstances::sample();

        let manifest =
            TransactionManifest::apply_security_shield_for_unsecurified_entity(
                AnyUnsecurifiedEntity::new(entity.clone().into()).unwrap(),
                TransactionManifestApplySecurityShieldUnsecurifiedInput::new(
                    security_structure_of_factor_instances.clone(),
                ),
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

        assert!(expected_manifest_str.contains(&entity.address.to_string()));

        let bob = Account::sample_bob();

        let with_ac_xrd_vault_top_up_by_unsecurified_account = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_paid_by_account(
            bob.clone(),
            entity.clone(),
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

        let with_ac_xrd_vault_top_up_by_securified_account_amount_42 = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_paid_by_account(
            grace_secure.clone(),
            entity.clone(),
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
        let entity = Persona::sample_other();
        let security_structure_of_factor_instances =
            SecurityStructureOfFactorInstances::sample_other();

        let manifest =
            TransactionManifest::apply_security_shield_for_unsecurified_entity(
                AnyUnsecurifiedEntity::new(entity.clone().into()).unwrap(),
                TransactionManifestApplySecurityShieldUnsecurifiedInput::new(
                    security_structure_of_factor_instances.clone(),
                ),
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
        let manifest =
            TransactionManifest::apply_security_shield_for_unsecurified_entity(
                AnyUnsecurifiedEntity::new(Account::sample_other().into())
                    .unwrap(),
                TransactionManifestApplySecurityShieldUnsecurifiedInput::new(
                    SecurityStructureOfFactorInstances::sample_other(),
                ),
            );
        assert_eq!(manifest, Err(CommonError::SecurityStructureOfFactorInstancesEntityDiscrepancyInEntityKind { entity_kind_of_entity: CAP26EntityKind::Account.to_string(), entity_kind_of_factor_instances: CAP26EntityKind::Identity.to_string() }));
    }

    #[test]
    fn test_mismatch_entity_kind_persona_account() {
        let manifest =
            TransactionManifest::apply_security_shield_for_unsecurified_entity(
                AnyUnsecurifiedEntity::new(Persona::sample_other().into())
                    .unwrap(),
                TransactionManifestApplySecurityShieldUnsecurifiedInput::new(
                    SecurityStructureOfFactorInstances::sample(),
                ),
            );
        assert_eq!(manifest, Err(CommonError::SecurityStructureOfFactorInstancesEntityDiscrepancyInEntityKind { entity_kind_of_entity: CAP26EntityKind::Identity.to_string(), entity_kind_of_factor_instances: CAP26EntityKind::Account.to_string() }));
    }
}
