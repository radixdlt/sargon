use crate::prelude::*;

pub async fn sign_entity_securify(
    os: &SargonOS,
    transaction_intent: TransactionIntent,
    entity_address: AddressOfAccountOrPersona,
) -> Result<SignedIntent> {
    let profile = os.profile()?;

    let (fee_paying_account_address, fee) = transaction_intent
        .extract_fee_payer_info()
        .expect("Should have a fee payer configured");

    let fee_payer_account =
        profile.account_by_address(fee_paying_account_address)?;
    let lock_fee_data = LockFeeData::new_with_account(
        fee_payer_account.clone(),
        fee,
        Decimal192::zero(), // Balance is not needed here
    );

    let securified_entity_applying_shield =
        profile.entity_by_address(entity_address)?;
    let unsecurified_entity =
        AnyUnsecurifiedEntity::new(securified_entity_applying_shield)?;

    // We do basically recreate the manifest here, because the Wallet would have added the lock fee, and may need to insert instructions after the lock fee, but before the securify instructions.
    // Not ideal, and should be improved in the future by not having the Wallet add the lock fee here, but for now this is the simplest way.
    let securify_manifest =
        TransactionManifest::apply_security_shield_for_unsecurified_entity(
            unsecurified_entity.clone(),
            unsecurified_entity
                .provisional_securified_config
                .clone()
                .unwrap()
                .get_security_structure_of_factor_instances(),
        )?;

    let mut top_up_ac_vault = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_entity_paid_by_account(
        fee_payer_account,
        unsecurified_entity.clone(),
        securify_manifest,
        fee
    );

    top_up_ac_vault = top_up_ac_vault.modify_add_lock_fee(lock_fee_data)?;

    let updated_transaction_intent = TransactionIntent::new(
        transaction_intent.header,
        top_up_ac_vault,
        transaction_intent.message,
    )?;

    os.sign(
        updated_transaction_intent.clone(),
        os.sign_transactions_interactor(),
        SigningPurpose::sign_transaction(RoleKind::Primary),
    )
    .await
}
