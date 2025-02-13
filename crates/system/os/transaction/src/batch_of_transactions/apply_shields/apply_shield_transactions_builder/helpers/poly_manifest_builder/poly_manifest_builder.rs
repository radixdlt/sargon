use crate::prelude::*;

use super::{
    ApplyShieldTransactionsManifestTxFeeModifier,
    ApplyShieldTransactionsManifestTxFeeModifierImpl,
    ApplyShieldTransactionsManifestXrdVaultContributor,
    ApplyShieldTransactionsManifestXrdVaultContributorImpl,
};

pub trait ApplyShieldTransactionsPolyManifestBuilder: Send + Sync {
    fn create_many_manifest_variants_per_roles_combination(
        &self,
        manifests_with_entities_with_xrd_balance: Vec<ShieldApplicationInput>,
    ) -> Result<Vec<SecurityShieldApplication>>;
}

pub struct ApplyShieldTransactionsPolyManifestBuilderImpl {
    fee_adder: Arc<dyn ApplyShieldTransactionsManifestTxFeeModifier>,
    xrd_vault_contributor:
        Arc<dyn ApplyShieldTransactionsManifestXrdVaultContributor>,
}

impl Default for ApplyShieldTransactionsPolyManifestBuilderImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplyShieldTransactionsPolyManifestBuilderImpl {
    pub fn new() -> Self {
        Self {
            fee_adder: Arc::new(
                ApplyShieldTransactionsManifestTxFeeModifierImpl::default(),
            ),
            xrd_vault_contributor: Arc::new(
                ApplyShieldTransactionsManifestXrdVaultContributorImpl::default(
                ),
            ),
        }
    }

    fn shield_application_for_unsecurified_account(
        &self,
        input: ApplicationInputForUnsecurifiedAccount,
    ) -> Result<SecurityShieldApplicationForUnsecurifiedAccount> {
        let input = self.fee_adder.add_fee_for_unsecurified_account(input)?;
        let input = self
            .xrd_vault_contributor
            .add_xrd_vault_contribution_for_unsecurified_account(input)?;
        let fee_tip_percentage = input.fee_tip_percentage();
        Ok(SecurityShieldApplicationForUnsecurifiedAccount::with_modified_manifest(
            input.entity_input.unsecurified_entity.clone(),
            input.paying_account,
            input.reviewed_manifest,
            fee_tip_percentage
        ))
    }

    fn shield_application_for_unsecurified_persona(
        &self,
        input: ApplicationInputForUnsecurifiedPersona,
    ) -> Result<SecurityShieldApplicationForUnsecurifiedPersona> {
        let input = self.fee_adder.add_fee_for_unsecurified_persona(input)?;
        let (input, paying_account) = self
            .xrd_vault_contributor
            .add_xrd_vault_contribution_for_unsecurified_persona(input)?;

        let fee_tip_percentage = input.fee_tip_percentage();
        Ok(SecurityShieldApplicationForUnsecurifiedPersona::with_modified_manifest(
            input.entity_input.clone(),
            paying_account,
            input.reviewed_manifest,
            fee_tip_percentage,
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

    fn shield_application_for_securified_account(
        &self,
        input: ApplicationInputForSecurifiedAccount,
    ) -> Result<SecurityShieldApplicationForSecurifiedAccount> {
        let manifest_for_variant =
            |variant: RolesExercisableInTransactionManifestCombination| {
                let manifest_with = input.clone();

                let manifest_with = self
                    .fee_adder
                    .add_fee_securified_account(manifest_with, variant)?;

                let manifest_with = self
                    .xrd_vault_contributor
                    .add_xrd_vault_contribution_for_securified_account(
                        manifest_with,
                        variant,
                    )?;

                Ok(manifest_with.reviewed_manifest)
            };

        let initiate_with_recovery_complete_with_primary = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary)?;

        let initiate_with_recovery_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation)?;

        let initiate_with_recovery_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion)?;

        let initiate_with_primary_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation)?;

        let initiate_with_primary_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion)?;

        let fee_tip_percentage = input.fee_tip_percentage();
        let account_with_optional_paying_account =
            SecurityShieldApplicationForSecurifiedAccountWithPayingAccount::new(
                input.entity_input.securified_account.clone(),
                input.paying_account,
                fee_tip_percentage,
            );

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

                let manifest_with = self
                    .fee_adder
                    .add_fee_for_securified_persona(manifest_with, variant)?;

                let manifest_with = self
                    .xrd_vault_contributor
                    .add_xrd_vault_contribution_for_securified_persona(
                        manifest_with,
                        variant,
                    )?;

                Ok(manifest_with.reviewed_manifest)
            };

        let initiate_with_recovery_complete_with_primary = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary)?;

        let initiate_with_recovery_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation)?;

        let initiate_with_recovery_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion)?;

        let initiate_with_primary_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation)?;

        let initiate_with_primary_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion)?;

        let fee_tip_percentage = input.fee_tip_percentage();
        let payer =
            SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount::new(
                input.entity_input.securified_persona.clone(),
                input.paying_account,
                fee_tip_percentage,
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
