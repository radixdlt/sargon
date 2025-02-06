use crate::prelude::*;

pub(super) trait ApplyShieldTransactionsManifestXrdVaultContributor:
    Send + Sync
{
    fn add_xrd_vault_contribution_for_unsecurified_account(
        &self,
        input: ApplicationInputForUnsecurifiedAccount,
    ) -> Result<ApplicationInputForUnsecurifiedAccount>;

    fn add_xrd_vault_contribution_for_unsecurified_persona(
        &self,
        input: ApplicationInputForUnsecurifiedPersona,
    ) -> Result<(
        ApplicationInputForUnsecurifiedPersona,
        ApplicationInputPayingAccount,
    )>;

    fn add_xrd_vault_contribution_for_securified_account(
        &self,
        input: ApplicationInputForSecurifiedAccount,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<ApplicationInputForSecurifiedAccount>;

    fn add_xrd_vault_contribution_for_securified_persona(
        &self,
        input: ApplicationInputForSecurifiedPersona,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<ApplicationInputForSecurifiedPersona>;
}

pub(super) struct ApplyShieldTransactionsManifestXrdVaultContributorImpl {}
impl Default for ApplyShieldTransactionsManifestXrdVaultContributorImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplyShieldTransactionsManifestXrdVaultContributorImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl ApplyShieldTransactionsManifestXrdVaultContributor
    for ApplyShieldTransactionsManifestXrdVaultContributorImpl
{
    fn add_xrd_vault_contribution_for_unsecurified_account(
        &self,
        input: ApplicationInputForUnsecurifiedAccount,
    ) -> Result<ApplicationInputForUnsecurifiedAccount> {
        let payer_balance = input.xrd_balance_of_paying_account();

        let needed_balance =
            input.xrd_needed_for_tx_fee_and_initial_xrd_vault_contributition();

        if payer_balance < needed_balance {
            return Err(CommonError::UnableContributeToAcXrdVaultInsufficientBalanceOfPayer {
                payer: input.payer().address.to_string(),
                vault_of_entity: input.entity_input.unsecurified_entity.address().to_string(),
                payer_balance: payer_balance.to_string(),
                needed_balance: needed_balance.to_string(),
            });
        }

        let payer = input
            .maybe_paying_account
            .clone()
            .map(|p| p.account())
            .unwrap_or(input.entity_input.unsecurified_entity.entity.clone());
        let entity = input.entity_input.unsecurified_entity.clone().into();

        input.modifying_manifest(|m| {
                let m = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_entity_paid_by_account(payer, entity, m, None);

                Ok(m)
            })
    }

    fn add_xrd_vault_contribution_for_unsecurified_persona(
        &self,
        input: ApplicationInputForUnsecurifiedPersona,
    ) -> Result<(
        ApplicationInputForUnsecurifiedPersona,
        ApplicationInputPayingAccount,
    )> {
        let payer_balance = input.xrd_balance_of_paying_account();
        let needed_balance =
            input.xrd_needed_for_tx_fee_and_initial_xrd_vault_contributition();
        if payer_balance
            < input.xrd_needed_for_tx_fee_and_initial_xrd_vault_contributition()
        {
            return Err(CommonError::UnableContributeToAcXrdVaultInsufficientBalanceOfPayer {
                payer: input.payer().address.to_string(),
                vault_of_entity: input.entity_input.address().to_string(),
                payer_balance: payer_balance.to_string(),
                needed_balance: needed_balance.to_string(),
            });
        }

        let payer_info = input.paying_account.clone();
        let payer = payer_info.account();

        let entity = input.entity_input.clone().into();

        input.modifying_manifest(|m| {
                let m = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_entity_paid_by_account(payer, entity, m, None);

                Ok(m)
            }).map(|modified| (modified, payer_info))
    }

    fn add_xrd_vault_contribution_for_securified_account(
        &self,
        input: ApplicationInputForSecurifiedAccount,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<ApplicationInputForSecurifiedAccount> {
        if !manifest_variant.can_quick_confirm()
            && !manifest_variant.can_exercise_primary_role()
        {
            // If we cannot quick confirm the topping up of the XRD vault instructions
            // must not go into this manifest for initiate recovery, we should
            // include the top up in the Confirm Recovery Manifest happening later (
            // after time delay).
            //
            // If we can exercise the primary role, we can auth with Primary Role of
            // old factors
            return Ok(input);
        }

        let entity = input.entity_input.clone().securified_account;
        let payer = input.xrd_balance_and_account_topping_up();
        let needed_balance = input.xrd_needed_for_tx_fee_and_xrd_vault_top_up();
        if payer.balance < needed_balance {
            return Err(CommonError::UnableContributeToAcXrdVaultInsufficientBalanceOfPayer {
                payer: payer.entity.address.to_string(),
                vault_of_entity: entity.address().to_string(),
                payer_balance: payer.balance.to_string(),
                needed_balance: needed_balance.to_string(),
            });
        }

        let payer = payer.entity.clone();

        input.modifying_manifest(|m| {
                TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_securified_entity_paid_by_account(payer, entity, m, None, manifest_variant)
            })
    }

    fn add_xrd_vault_contribution_for_securified_persona(
        &self,
        input: ApplicationInputForSecurifiedPersona,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<ApplicationInputForSecurifiedPersona> {
        if !manifest_variant.can_quick_confirm() {
            // If we cannot quick confirm the topping up of the XRD vault instructions
            // must not go into this manifest for initiate recovery, we should
            // include the top up in the Confirm Recovery Manifest happening later (
            // after time delay).
            return Ok(input);
        }

        let Some(payer) = input.xrd_balance_and_account_topping_up() else {
            return Err(
                CommonError::UnableContributeToAcXrdVaultPersonaRequiresPayer,
            );
        };

        let entity = input.entity_input.clone().securified_persona;
        let payer_info = payer.clone();
        let needed_balance = input.xrd_needed_for_tx_fee_and_xrd_vault_top_up();
        if payer.xrd_balance_of_account()
            < input.xrd_needed_for_tx_fee_and_xrd_vault_top_up()
        {
            return Err(CommonError::UnableContributeToAcXrdVaultInsufficientBalanceOfPayer {
                payer: payer_info.account().to_string(),
                vault_of_entity: entity.address().to_string(),
                payer_balance: payer.xrd_balance_of_account().to_string(),
                needed_balance: needed_balance.to_string(),
            });
        }
        let payer = payer_info.account();

        input.modifying_manifest(|m| {
                TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_securified_entity_paid_by_account(payer, entity, m, None, manifest_variant)
            })
    }
}
