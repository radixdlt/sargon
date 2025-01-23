use radix_common::prelude::ManifestGlobalAddress;
use radix_engine_interface::blueprints::access_controller::{
    AccessControllerContributeRecoveryFeeManifestInput,
    ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT as SCRYPTO_ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT,
    ACCESS_CONTROLLER_CREATE_PROOF_IDENT as SCRYPTO_ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
};
use radix_transactions::prelude::ManifestBuilder;

use crate::prelude::*;

const XRD_TO_AC_VAULT_FIRST_TOP_UP: ScryptoDecimal192 =
    ScryptoDecimal192::ONE_HUNDRED;

pub trait TransactionManifestAccessControllerXrdVaultToppingUp {
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
}

impl TransactionManifestAccessControllerXrdVaultToppingUp
    for TransactionManifest
{
}
