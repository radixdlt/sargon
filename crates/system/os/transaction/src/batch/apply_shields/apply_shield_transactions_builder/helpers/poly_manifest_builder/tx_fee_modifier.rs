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
        let payer = input.paying_account.account_address();
        input.try_modifying_manifest(|m| {
            m.modify_add_lock_fee_and_proofs(
                LockFeeData::new_with_fee(payer, estimated_xrd_fee),
                IndexMap::new(),
            )
        })
    }

    fn add_fee_for_unsecurified_persona(
        &self,
        input: ApplicationInputForUnsecurifiedPersona,
    ) -> Result<ApplicationInputForUnsecurifiedPersona> {
        let payer = input.payer();
        let estimated_xrd_fee = input.estimated_xrd_fee;
        input.try_modifying_manifest(|m| {
            m.modify_add_lock_fee_and_proofs(
                LockFeeData::new_with_fee(payer.address, estimated_xrd_fee),
                IndexMap::new(),
            )
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
        input.modifying_manifest(|m| {
            TransactionManifest::modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(
                m,
                estimated_xrd_fee,
                entity_applying_shield
            )
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
            TransactionManifest::modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(
                m,
                estimated_xrd_fee,
                entity_applying_shield
            )
        })
    }
}
