use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplyShieldTransactionsPolyManifestBuilder: Send + Sync {
    fn create_many_manifest_variants_per_roles_combination(
        &self,
        manifests_with_entities_with_xrd_balance: Vec<ShieldApplicationInput>,
    ) -> Result<Vec<SecurityShieldApplication>>;
}

pub struct ApplyShieldTransactionsPolyManifestBuilderImpl {}
impl ApplyShieldTransactionsPolyManifestBuilderImpl {
    pub fn new() -> Self {
        Self {}
    }

    fn modify_manifest_add_fee_securified_persona(
        input: ApplicationInputForSecurifiedPersona,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<ApplicationInputForSecurifiedPersona> {
        todo!()
    }
    fn modify_manifest_add_fee_securified_account(
        input: ApplicationInputForSecurifiedAccount,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<ApplicationInputForSecurifiedAccount> {
        todo!()
    }

    fn modify_manifest_add_fee_for_unsecurified_account(
        input: ApplicationInputForUnsecurifiedAccount,
    ) -> Result<ApplicationInputForUnsecurifiedAccount> {
        // Self::_modify_manifest_add_fee(input, None)
        todo!()
    }

    fn modify_manifest_add_fee_for_unsecurified_persona(
        input: ApplicationInputForUnsecurifiedPersona,
    ) -> Result<ApplicationInputForUnsecurifiedPersona> {
        // Self::_modify_manifest_add_fee(input, None)
        todo!()
    }

    fn modify_manifest_add_xrd_vault_contribution_for_unsecurified_account_applying_shield(
        input: ApplicationInputForUnsecurifiedAccount,
    ) -> Result<ApplicationInputForUnsecurifiedAccount> {

        let payer_balance = input.xrd_balance_of_paying_account();

        if payer_with_balance.balance < input.xrd_needed_for_tx_fee_and_initial_xrd_vault_contributition() {
            return Err(CommonError::Unknown); // CommonError::InsufficientXrdBalance
        }

        let payer = payer_with_balance.entity;
        let unsecurified_account_applying_shield =
            unsecurified_account_applying_shield_with_balance.entity;

        input.modifying_manifest(|m| {
                let m = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_account_paid_by_account(payer, unsecurified_account_applying_shield.into(), m, None);

                Ok(m)
            })
    }

    fn modify_manifest_add_xrd_vault_contribution_for_unsecurified_persona_applying_shield(
        input: ApplicationInputForUnsecurifiedPersona,
    ) -> Result<(
        ApplicationInputForUnsecurifiedPersona,
        ApplicationInputPayingAccount,
    )> {
        // let payer_with_balance: ! = input.payer_with_balance()?;

        // if payer_with_balance.balance < input.needed_xrd_for_fee_and_topup() {
        //     return Err(CommonError::Unknown); // CommonError::InsufficientXrdBalance
        // }

        // let unsecurified_persona_applying_shield = input
        //     .get_entity_applying_shield_and_balance()
        //     .entity
        //     .clone()
        //     .into();

        // input.modifying_manifest(|m| {
        //         let m = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_account_paid_by_account(payer_with_balance.entity.clone(), unsecurified_persona_applying_shield, m, None);

        //         Ok(m)
        //     }).map(|m| (m, payer_with_balance.entity))
        todo!()
    }

    fn shield_application_for_unsecurified_account(
        &self,
        input: ApplicationInputForUnsecurifiedAccount,
    ) -> Result<SecurityShieldApplicationForUnsecurifiedAccount> {
        let input =
            Self::modify_manifest_add_fee_for_unsecurified_account(input)?;
        let input = Self::modify_manifest_add_xrd_vault_contribution_for_unsecurified_account_applying_shield(input)?;

        Ok(SecurityShieldApplicationForUnsecurifiedAccount::with_modified_manifest(
            input.entity_input.unsecurified_entity.clone(),
            input.maybe_paying_account,
            input.reviewed_manifest,
        ))
    }

    fn shield_application_for_unsecurified_persona(
        &self,
        input: ApplicationInputForUnsecurifiedPersona,
    ) -> Result<SecurityShieldApplicationForUnsecurifiedPersona> {
        let input =
            Self::modify_manifest_add_fee_for_unsecurified_persona(input)?;
        let (input, paying_account) = Self::modify_manifest_add_xrd_vault_contribution_for_unsecurified_persona_applying_shield(input)?;

        Ok(SecurityShieldApplicationForUnsecurifiedPersona::with_modified_manifest(
            input.entity_input.clone(),
            paying_account,
            input.reviewed_manifest,
        ))
    }

    fn shield_application_for_unsecurified_entity(
        &self,
        input: ApplicationInputForUnsecurifiedEntity,
    ) -> Result<SecurityShieldApplicationForUnsecurifiedEntity> {
        match input {
            ApplicationInputForUnsecurifiedEntity::Account(a) => self
                .shield_application_for_unsecurified_account(a)
                .map(SecurityShieldApplicationForUnsecurifiedEntity::Account),
            ApplicationInputForUnsecurifiedEntity::Persona(p) => self
                .shield_application_for_unsecurified_persona(p)
                .map(SecurityShieldApplicationForUnsecurifiedEntity::Persona),
        }
    }

    fn modify_manifest_add_xrd_vault_contribution_for_securified_persona_applying_shield(
        input: ApplicationInputForSecurifiedPersona,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<ApplicationInputForSecurifiedPersona> {
        todo!()
    }

    fn modify_manifest_add_xrd_vault_contribution_for_securified_account_applying_shield(
        input: ApplicationInputForSecurifiedAccount,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<ApplicationInputForSecurifiedAccount> {
        todo!()
    }

    fn shield_application_for_securified_account(
        &self,
        input: ApplicationInputForSecurifiedAccount,
    ) -> Result<SecurityShieldApplicationForSecurifiedAccount> {
        let manifest_for_variant =
            |variant: RolesExercisableInTransactionManifestCombination| {
                let manifest_with = input.clone();

                let manifest_with =
                    Self::modify_manifest_add_fee_securified_account(
                        manifest_with,
                        variant,
                    )?;

                let manifest_with = Self::modify_manifest_add_xrd_vault_contribution_for_securified_account_applying_shield(manifest_with, variant)?;

                Ok(manifest_with.reviewed_manifest)
            };

        let initiate_with_recovery_complete_with_primary = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary)?;

        let initiate_with_recovery_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation)?;

        let initiate_with_recovery_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion)?;

        let initiate_with_primary_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation)?;

        let initiate_with_primary_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion)?;

        let account_with_optional_paying_account = SecurityShieldApplicationForSecurifiedAccountWithOptionalPayingAccount::new(input.entity_input.securified_account.clone(), input.maybe_paying_account);

        Ok(SecurityShieldApplicationForSecurifiedAccount::new(
            account_with_optional_paying_account,
            initiate_with_recovery_complete_with_primary,
            initiate_with_recovery_complete_with_confirmation,
            initiate_with_recovery_delayed_completion,
            initiate_with_primary_complete_with_confirmation,
            initiate_with_primary_delayed_completion,
        ))
    }

    fn shield_application_for_securified_persona(
        &self,
        input: ApplicationInputForSecurifiedPersona,
    ) -> Result<SecurityShieldApplicationForSecurifiedPersona> {
        let manifest_for_variant =
            |variant: RolesExercisableInTransactionManifestCombination| {
                let manifest_with = input.clone();

                let manifest_with =
                    Self::modify_manifest_add_fee_securified_persona(
                        manifest_with,
                        variant,
                    )?;

                let manifest_with = Self::modify_manifest_add_xrd_vault_contribution_for_securified_persona_applying_shield(manifest_with, variant)?;

                Ok(manifest_with.reviewed_manifest)
            };

        let initiate_with_recovery_complete_with_primary = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary)?;

        let initiate_with_recovery_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation)?;

        let initiate_with_recovery_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion)?;

        let initiate_with_primary_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation)?;

        let initiate_with_primary_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion)?;

        let payer =
            SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount::new(
                input.entity_input.securified_persona.clone(),
                input.maybe_paying_account, // TODO Should we fail here if the an account doing top up is not specified?
            );

        Ok(SecurityShieldApplicationForSecurifiedPersona::new(
            payer,
            initiate_with_recovery_complete_with_primary,
            initiate_with_recovery_complete_with_confirmation,
            initiate_with_recovery_delayed_completion,
            initiate_with_primary_complete_with_confirmation,
            initiate_with_primary_delayed_completion,
        ))
    }

    fn shield_application_for_securified_entity(
        &self,
        input: ApplicationInputForSecurifiedEntity,
    ) -> Result<SecurityShieldApplicationForSecurifiedEntity> {
        match input {
            ApplicationInputForSecurifiedEntity::Account(a) => self
                .shield_application_for_securified_account(a)
                .map(SecurityShieldApplicationForSecurifiedEntity::Account),
            ApplicationInputForSecurifiedEntity::Persona(p) => self
                .shield_application_for_securified_persona(p)
                .map(SecurityShieldApplicationForSecurifiedEntity::Persona),
        }
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsPolyManifestBuilder
    for ApplyShieldTransactionsPolyManifestBuilderImpl
{
    fn create_many_manifest_variants_per_roles_combination(
        &self,
        manifests_with_entities_with_xrd_balance: Vec<ShieldApplicationInput>,
    ) -> Result<Vec<SecurityShieldApplication>> {
        manifests_with_entities_with_xrd_balance
            .into_iter()
            .map(|manifest_with_payer| match manifest_with_payer {
                ShieldApplicationInput::Unsecurified(unsec) => self
                    .shield_application_for_unsecurified_entity(unsec)
                    .map(SecurityShieldApplication::unsecurified),
                ShieldApplicationInput::Securified(sec) => self
                    .shield_application_for_securified_entity(sec)
                    .map(SecurityShieldApplication::securified),
            })
            .collect::<Result<Vec<SecurityShieldApplication>>>()
    }
}
