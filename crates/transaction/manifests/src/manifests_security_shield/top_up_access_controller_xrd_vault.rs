use profile_supporting_types::{AnySecurifiedEntity, AnyUnsecurifiedEntity};
use radix_common::prelude::ManifestGlobalAddress;
use radix_engine_interface::blueprints::access_controller::{
    AccessControllerContributeRecoveryFeeManifestInput,
    ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT as SCRYPTO_ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT,
    ACCESS_CONTROLLER_CREATE_PROOF_IDENT as SCRYPTO_ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
};

use crate::prelude::*;

const XRD_TO_AC_VAULT_FIRST_TOP_UP: ScryptoDecimal192 =
    ScryptoDecimal192::ONE_HUNDRED;

pub fn xrd_amount_for_initial_xrd_contribution_of_vault_of_access_controller(
) -> Decimal192 {
    XRD_TO_AC_VAULT_FIRST_TOP_UP.into()
}

pub fn xrd_target_for_access_controller() -> Decimal192 {
    XRD_TO_AC_VAULT_FIRST_TOP_UP.into()
}

pub trait TransactionManifestAccessControllerXrdVaultToppingUp {
    /// A method modifying manifests which applies security shield. We
    /// The `manifest` which applies the security shield could not include
    /// the instructions which we append in this method since the host did
    /// not know the `payer` at the time of creating that manifest. The host
    /// will call this method when the `payer` is known, which appends instructions
    /// at the end of `manifest` for topping the XRD vault of the access controller
    /// with `top_up_amount` many XRD.
    ///
    /// We will use the address reservation of `apply_security_shield_for_unsecurified_entity`
    /// (`ACCESS_CONTROLLER_ADDRESS_RESERVATION_NAME`) which we cannot access by id
    /// since Radix Engine discard those ids and uses internal ones, instead we need
    /// to use `ManifestGlobalAddress::Named(ScryptoManifestNamedAddress(0))`.
    ///
    /// If `payer` is securified we will also add a `create_proof` instruction for
    /// authenticating the withdrawal of XRD from the payer.
    ///
    /// If `top_up_amount` is `None` the `XRD_TO_AC_VAULT_FIRST_TOP_UP` will be used.
    /// We allow to pass amount so that we can top of with more or less based on
    /// token balance of `payer` and current balance of the access controller (when
    /// we use this method for securified entities.)
    fn modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_entity_paid_by_account(
        payer: impl Into<Account>,
        // TODO: remove `unsecurified_entity_applying_shield`, this should be read out from the manifest in a throwing function, `manifest.get_address_of_entity_applying_shield()` or similar which Omar need to provide us with, oh well we need the account here, so elsewhere, in SargonOS where we have access to Profile we would call `manifest.get_address_of_entity_applying_shield` and then lookup the entity.
        unsecurified_entity_applying_shield: AnyUnsecurifiedEntity,
        manifest: TransactionManifest,
        top_up_amount: impl Into<Option<Decimal192>>,
    ) -> TransactionManifest {
        Self::_modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_paid_by_account(
            payer,
            unsecurified_entity_applying_shield.entity,
            manifest,
            top_up_amount,
            true, false
        ).expect("Should never fail")
    }

    fn modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_securified_entity_paid_by_account(
        payer: impl Into<Account>,
        // TODO: remove `securified_entity_applying_shield`, this should be read out from the manifest in a throwing function, `manifest.get_address_of_entity_applying_shield()` or similar which Omar need to provide us with, oh well we need the account here, so elsewhere, in SargonOS where we have access to Profile we would call `manifest.get_address_of_entity_applying_shield` and then lookup the entity.
        securified_entity_applying_shield: impl Into<AnySecurifiedEntity>,
        manifest: TransactionManifest,
        top_up_amount: impl Into<Option<Decimal192>>,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<TransactionManifest> {
        Self::_modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_paid_by_account(
            payer,
            securified_entity_applying_shield.into().entity,
            manifest,
            top_up_amount,
            manifest_variant.can_exercise_primary_role(),
            manifest_variant.can_quick_confirm(),
        )
    }

    fn _modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_paid_by_account(
        payer: impl Into<Account>,
        // TODO: remove `entity_applying_shield`, this should be read out from the manifest in a throwing function, `manifest.get_address_of_entity_applying_shield()` or similar which Omar need to provide us with, oh well we need the account here, so elsewhere, in SargonOS where we have access to Profile we would call `manifest.get_address_of_entity_applying_shield` and then lookup the entity.
        entity_applying_shield: AccountOrPersona,
        manifest: TransactionManifest,
        top_up_amount: impl Into<Option<Decimal192>>,
        can_exercise_primary_role: bool,
        can_quick_confirm: bool,
    ) -> Result<TransactionManifest> {
        let mut builder = ScryptoTransactionManifestBuilder::new();
        let payer = payer.into();
        let address_of_paying_account = payer.address();

        let address_of_access_controller_to_top_up =
            match entity_applying_shield.security_state() {
                EntitySecurityState::Securified { value: sec } => {
                    ManifestGlobalAddress::Static(
                        sec.access_controller_address().scrypto(),
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
                        sec.access_controller_address().scrypto(),
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
                SCRYPTO_ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
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

        builder = builder.extend_builder_with_manifest(manifest);

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

        let manifest = TransactionManifest::sargon_built(
            builder,
            address_of_paying_account.network_id(),
        );

        Ok(manifest)
    }
}

impl TransactionManifestAccessControllerXrdVaultToppingUp
    for TransactionManifest
{
}
