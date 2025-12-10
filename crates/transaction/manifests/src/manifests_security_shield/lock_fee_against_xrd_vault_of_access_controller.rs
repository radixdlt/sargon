use radix_engine_interface::blueprints::access_controller::{
    AccessControllerLockRecoveryFeeInput as ScryptoAccessControllerLockRecoveryFeeInput,
    ACCESS_CONTROLLER_LOCK_RECOVERY_FEE_IDENT as SCRYPTO_ACCESS_CONTROLLER_LOCK_RECOVERY_FEE_IDENT,
};

use crate::prelude::*;

impl TransactionManifestLockFeeAgainstXrdVaultOfAccessController
    for TransactionManifest
{
}

pub trait TransactionManifestLockFeeAgainstXrdVaultOfAccessController {
    /// Locks transaction fee against the XRD vault of the access controller of
    /// `entity_applying_shield` - either AC of an Account or of a Persona.
    ///
    /// We need to call this later when we have made a preview/dry-run of the
    /// `manifest` to get the actual fee to lock.
    ///
    /// `manifest` was produced by `apply_security_shield_for_securified_entity`.
    ///
    /// In fact we will be locking fee for 6 flavours of transaction manifest
    /// which updates the security shield of an entity, as returned by
    /// `RolesExercisableInTransactionManifestCombination::all()`, and we could try to
    /// be smart and run preview of each six to get a minimal fee per manifest,
    /// but we will avoid that complexity.
    ///
    /// Only relevant for securified entities - since it is only securified entities
    /// which have an access controller to lock against.
    fn modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(
        manifest: TransactionManifest,
        fee: Decimal192,
        access_controller_address: AccessControllerAddress,
    ) -> TransactionManifest {
        let mut builder = ScryptoTransactionManifestBuilder::new();

        // Lock fee against XRD vault of the access controller
        // put this instruction at index 0
        builder = builder.call_method(
            access_controller_address.scrypto(),
            SCRYPTO_ACCESS_CONTROLLER_LOCK_RECOVERY_FEE_IDENT,
            ScryptoAccessControllerLockRecoveryFeeInput {
                amount: ScryptoDecimal192::from(fee),
            },
        );

        // ... then append all instructions from the original manifest
        builder = builder.extend_builder_with_manifest(manifest);

        TransactionManifest::sargon_built(
            builder,
            access_controller_address.network_id(),
        )
    }
}
