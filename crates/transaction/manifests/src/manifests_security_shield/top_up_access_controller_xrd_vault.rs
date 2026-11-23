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

pub trait TransactionManifestTopUpAccessControllerXrdVault {
    /// A transaction of its own that withdraws `amount` XRD from `payer` and
    /// contributes it to the recovery fee vault of `access_controller`, which
    /// must already exist. It's classified as an access controller recovery
    /// fee transaction, which shows the XRD moved.
    ///
    /// If `payer` is securified, a proof of its access controller's badge
    /// authorizes the withdrawal.
    fn top_up_access_controller_xrd_vault(
        access_controller: AccessControllerAddress,
        payer: &Account,
        amount: Decimal192,
    ) -> TransactionManifest {
        let mut builder = ScryptoTransactionManifestBuilder::new();
        if let EntitySecurityState::Securified { value } =
            payer.security_state()
        {
            builder = builder.call_method(
                value.access_controller_address().scrypto(),
                SCRYPTO_ACCESS_CONTROLLER_CREATE_PROOF_IDENT,
                (),
            );
        }
        let amount = ScryptoDecimal192::from(amount);
        let bucket = "xrd_to_top_up_ac_vault_bucket";
        builder = builder
            .withdraw_from_account(payer.address.scrypto(), XRD, amount)
            .take_from_worktop(XRD, amount, bucket);
        let xrd = builder.bucket(bucket);
        builder = builder.call_method(
            access_controller.scrypto(),
            SCRYPTO_ACCESS_CONTROLLER_CONTRIBUTE_RECOVERY_FEE_IDENT,
            AccessControllerContributeRecoveryFeeManifestInput { bucket: xrd },
        );
        TransactionManifest::sargon_built(
            builder,
            access_controller.network_id(),
        )
    }
}

impl TransactionManifestTopUpAccessControllerXrdVault for TransactionManifest {}
