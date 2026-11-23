//! The transactions the wallet builds to apply a security shield, each of
//! which must be of the class whose review shows all it does.

use crate::prelude::*;
use profile_supporting_types::{
    AnySecurifiedEntity, AnyUnsecurifiedEntity, SecurifiedAccount,
    SecurifiedPersona,
};

fn classes(manifest: &TransactionManifest) -> IndexSet<ManifestClassification> {
    manifest.summary().unwrap().classification
}

#[track_caller]
fn assert_class(manifest: &TransactionManifest, class: ManifestClassification) {
    let classes = classes(manifest);
    assert!(classes.contains(&class), "{classes:?}");
}

/// A shield's factor instances for the kind of `entity`.
fn instances_for(
    entity: &AnySecurifiedEntity,
) -> SecurityStructureOfFactorInstances {
    if entity.entity.is_account_entity() {
        SecurityStructureOfFactorInstances::sample()
    } else {
        SecurityStructureOfFactorInstances::sample_other()
    }
}

fn with_fee_locked(manifest: TransactionManifest) -> [TransactionManifest; 2] {
    let access_controller =
        SecurifiedAccount::sample().access_controller_address();
    [
        manifest
            .clone()
            .modify_add_lock_fee(LockFeeData::new_with_unsecurified_fee_payer(
                AccountAddress::sample_mainnet(),
                Decimal192::two(),
            ))
            .unwrap(),
        TransactionManifest::modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(
            manifest,
            Decimal192::two(),
            access_controller,
        ),
    ]
}

#[test]
fn securifying_is_entity_securify() {
    for (entity, instances) in [
        (
            AnyUnsecurifiedEntity::new(Account::sample()).unwrap(),
            SecurityStructureOfFactorInstances::sample(),
        ),
        (
            AnyUnsecurifiedEntity::new(Persona::sample()).unwrap(),
            SecurityStructureOfFactorInstances::sample_other(),
        ),
    ] {
        let manifest =
            TransactionManifest::apply_security_shield_for_unsecurified_entity(
                entity, instances,
            )
            .unwrap();
        let [from_account, _] = with_fee_locked(manifest.clone());
        for manifest in [manifest, from_account] {
            assert_class(&manifest, ManifestClassification::EntitySecurify);
        }
    }
}

#[test]
fn updating_a_shield_is_a_recovery_or_timed_recovery() {
    use RolesExercisableInTransactionManifestCombination::*;
    let entities: [AnySecurifiedEntity; 2] = [
        SecurifiedAccount::sample().into(),
        SecurifiedPersona::sample().into(),
    ];
    for entity in entities {
        for (combination, class) in [
            (
                InitiateWithRecoveryCompleteWithPrimary,
                ManifestClassification::AccessControllerRecovery,
            ),
            (
                InitiateWithRecoveryCompleteWithConfirmation,
                ManifestClassification::AccessControllerRecovery,
            ),
            (
                InitiateWithPrimaryCompleteWithConfirmation,
                ManifestClassification::AccessControllerRecovery,
            ),
            (
                InitiateWithRecoveryDelayedCompletion,
                ManifestClassification::AccessControllerTimedRecovery,
            ),
        ] {
            let manifest =
                TransactionManifest::apply_security_shield_for_securified_entity(
                    entity.clone(),
                    instances_for(&entity),
                    combination,
                );
            let [from_account, from_access_controller] =
                with_fee_locked(manifest.clone());
            for manifest in [manifest, from_account, from_access_controller] {
                assert_class(&manifest, class);
            }
        }
    }
}

#[test]
fn topping_up_is_a_recovery_fee_transaction() {
    let access_controller =
        SecurifiedAccount::sample().access_controller_address();
    for payer in [Account::sample(), SecurifiedAccount::sample().entity] {
        let manifest = TransactionManifest::top_up_access_controller_xrd_vault(
            access_controller,
            &payer,
            Decimal192::ten(),
        );
        let [from_account, _] = with_fee_locked(manifest.clone());
        for manifest in [manifest, from_account] {
            assert_class(
                &manifest,
                ManifestClassification::AccessControllerRecoveryFee,
            );
        }
    }
}

/// Setting the ROLA key is no access controller operation, nor of any other
/// class: the wallet builds it as a transaction of its own, which updates the
/// entity's owner keys.
#[test]
fn setting_the_rola_key_is_a_transaction_of_its_own() {
    let entities: [AnySecurifiedEntity; 2] = [
        SecurifiedAccount::sample().into(),
        SecurifiedPersona::sample().into(),
    ];
    for entity in entities {
        let instances = instances_for(&entity);
        let rola_key = instances.authentication_signing_factor_instance;
        let manifest = TransactionManifest::set_rola_key_of_securified_entity(
            &entity, &rola_key,
        );
        let summary = manifest.summary().unwrap();
        assert_eq!(summary.classification, IndexSet::new());
        let update = if entity.entity.is_account_entity() {
            ReservedInstruction::AccountUpdateOwnerKeysMetadataField
        } else {
            ReservedInstruction::IdentityUpdateOwnerKeysMetadataField
        };
        assert_eq!(
            summary.reserved_instructions,
            vec![ReservedInstruction::AccessControllerMethod, update]
        );
        assert!(manifest
            .manifest_string()
            .contains(&PublicKeyHash::hash(rola_key.public_key()).to_string()));
    }
}
