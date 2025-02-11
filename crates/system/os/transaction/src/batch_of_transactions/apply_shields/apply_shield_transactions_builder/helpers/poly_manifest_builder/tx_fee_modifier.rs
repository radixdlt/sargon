use crate::prelude::*;

pub(super) trait ApplyShieldTransactionsManifestTxFeeModifier:
    Send + Sync
{
    fn add_fee_for_unsecurified_account(
        &self,
        input: ApplicationInputForUnsecurifiedAccount,
    ) -> Result<ApplicationInputForUnsecurifiedAccount>;

    fn add_fee_for_unsecurified_persona(
        &self,
        input: ApplicationInputForUnsecurifiedPersona,
    ) -> Result<ApplicationInputForUnsecurifiedPersona>;

    fn add_fee_for_securified_persona(
        &self,
        input: ApplicationInputForSecurifiedPersona,
        _manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<ApplicationInputForSecurifiedPersona>;

    fn add_fee_securified_account(
        &self,
        input: ApplicationInputForSecurifiedAccount,
        _manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<ApplicationInputForSecurifiedAccount>;
}

pub(super) struct ApplyShieldTransactionsManifestTxFeeModifierImpl {}
impl Default for ApplyShieldTransactionsManifestTxFeeModifierImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplyShieldTransactionsManifestTxFeeModifierImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl ApplyShieldTransactionsManifestTxFeeModifier
    for ApplyShieldTransactionsManifestTxFeeModifierImpl
{
    fn add_fee_for_unsecurified_account(
        &self,
        input: ApplicationInputForUnsecurifiedAccount,
    ) -> Result<ApplicationInputForUnsecurifiedAccount> {
        let estimated_xrd_fee = input.estimated_xrd_fee;

        input.clone().modifying_manifest(|m| {
            if AddressOfAccountOrPersona::from(
                input.paying_account.account_address(),
            ) != input.entity_input.unsecurified_entity.address()
            {
                let other_payer = input.paying_account.account_address();
                // We lock fee against the other account (but not against its AC vault)
                let m = m.modify_add_lock_fee(&other_payer, estimated_xrd_fee);
                Ok(m)
            } else {
                // Not another account paying, and entity is unsecurified => use lock fee
                let self_payer =
                    input.entity_input.unsecurified_entity.entity.address;
                let m = m.modify_add_lock_fee(&self_payer, estimated_xrd_fee);
                Ok(m)
            }
        })
    }

    fn add_fee_for_unsecurified_persona(
        &self,
        input: ApplicationInputForUnsecurifiedPersona,
    ) -> Result<ApplicationInputForUnsecurifiedPersona> {
        let payer = input.payer();
        let estimated_xrd_fee = input.estimated_xrd_fee;
        input.modifying_manifest(|m| {
            let m = m.modify_add_lock_fee(&payer.address, estimated_xrd_fee);
            Ok(m)
        })
    }

    fn add_fee_for_securified_persona(
        &self,
        input: ApplicationInputForSecurifiedPersona,
        _manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<ApplicationInputForSecurifiedPersona> {
        let entity_applying_shield =
            input.entity_input.securified_persona.clone();
        let estimated_xrd_fee = input.estimated_xrd_fee;
        input.clone().modifying_manifest(|m| {
            let m = TransactionManifest::modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(
                m,
                estimated_xrd_fee,
                entity_applying_shield
            );
            Ok(m)
        })
    }

    fn add_fee_securified_account(
        &self,
        input: ApplicationInputForSecurifiedAccount,
        _manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<ApplicationInputForSecurifiedAccount> {
        let entity_applying_shield =
            input.entity_input.securified_account.clone();
        let estimated_xrd_fee = input.estimated_xrd_fee;
        input.modifying_manifest(|m| {
            let m = TransactionManifest::modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(
                m,
                estimated_xrd_fee,
                entity_applying_shield
            );
            Ok(m)
        })
    }
}
