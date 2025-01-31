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
    pub fn new<'a>(os: &'a SargonOS) -> Self {
        todo!()
    }

    fn _modify_manifest_add_fee<Entity>(
        input: AbstractShieldApplicationInput<Entity>,
        // None if unsecurified
        manifest_variant: Option<
            RolesExercisableInTransactionManifestCombination,
        >,
    ) -> Result<AbstractShieldApplicationInput<Entity>>
    where
        Entity: HasEntityAddress + Clone,
    {
        //    if let Some(other_payer) = input.
        todo!()
    }

    fn modify_manifest_add_fee_securified<T>(
        input: AbstractShieldApplicationInput<AbstractSecurifiedEntity<T>>,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<AbstractShieldApplicationInput<AbstractSecurifiedEntity<T>>>
    where
        T: IsBaseEntity + std::hash::Hash + Eq + Clone,
    {
        Self::_modify_manifest_add_fee(input, Some(manifest_variant))
    }

    fn modify_manifest_add_fee_for_unsecurified_entity_applying_shield<T>(
        input: AbstractShieldApplicationInput<AbstractUnsecurifiedEntity<T>>,
    ) -> Result<AbstractShieldApplicationInput<AbstractUnsecurifiedEntity<T>>>
    where
        T: IsBaseEntity + std::hash::Hash + Eq + Clone,
    {
        Self::_modify_manifest_add_fee(input, None)
    }

    fn modify_manifest_add_xrd_vault_contribution_for_unsecurified_account_applying_shield(
        input: UnsecurifiedAccountShieldApplicationInput,
    ) -> Result<UnsecurifiedAccountShieldApplicationInput> {
        let unsecurified_account_applying_shield_with_balance =
            input.get_entity_applying_shield_and_balance();

        let payer_with_balance = input.payer_with_balance();

        if payer_with_balance.balance < input.needed_xrd_for_fee_and_topup() {
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
        input: UnsecurifiedPersonaShieldApplicationInput,
    ) -> Result<(UnsecurifiedPersonaShieldApplicationInput, Account)> {
        let payer_with_balance = input.payer_with_balance()?;

        if payer_with_balance.balance < input.needed_xrd_for_fee_and_topup() {
            return Err(CommonError::Unknown); // CommonError::InsufficientXrdBalance
        }

        let unsecurified_persona_applying_shield = input
            .get_entity_applying_shield_and_balance()
            .entity
            .clone()
            .into();

        input.modifying_manifest(|m| {
                let m = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_account_paid_by_account(payer_with_balance.entity.clone(), unsecurified_persona_applying_shield, m, None);

                Ok(m)
            }).map(|m| (m, payer_with_balance.entity))
    }

    fn shield_application_for_unsecurified_account(
        &self,
        input: UnsecurifiedAccountShieldApplicationInput,
    ) -> Result<SecurityShieldApplicationForUnsecurifiedAccount> {
        let input = Self::modify_manifest_add_fee_for_unsecurified_entity_applying_shield(input)?;
        let input = Self::modify_manifest_add_xrd_vault_contribution_for_unsecurified_account_applying_shield(input)?;

        Ok(SecurityShieldApplicationForUnsecurifiedAccount::with_modified_manifest(
            input.entity_applying_shield.clone(),
            input.maybe_other_payer_and_balance().map(|p| p.entity),
            input.manifest,
        ))
    }

    fn shield_application_for_unsecurified_persona(
        &self,
        input: UnsecurifiedPersonaShieldApplicationInput,
    ) -> Result<SecurityShieldApplicationForUnsecurifiedPersona> {
        let input = Self::modify_manifest_add_fee_for_unsecurified_entity_applying_shield(input)?;
        let (input, paying_account) = Self::modify_manifest_add_xrd_vault_contribution_for_unsecurified_persona_applying_shield(input)?;

        Ok(SecurityShieldApplicationForUnsecurifiedPersona::with_modified_manifest(
            input.entity_applying_shield.clone(),
            paying_account,
            input.manifest,
        ))
    }

    fn shield_application_for_unsecurified_entity(
        &self,
        input: AnyUnsecurifiedShieldApplicationInput,
    ) -> Result<SecurityShieldApplicationForUnsecurifiedEntity> {
        let entity = &input.entity_applying_shield;
        match &entity.entity {
            AccountOrPersona::AccountEntity(a) => self
                .shield_application_for_unsecurified_account(
                    UnsecurifiedAccountShieldApplicationInput::from((
                        input.clone(),
                        UnsecurifiedAccount::with_unsecured_entity_control(
                            a.clone(),
                            entity.unsecured_entity_control.clone(),
                        ),
                    )),
                )
                .map(SecurityShieldApplicationForUnsecurifiedEntity::Account),
            AccountOrPersona::PersonaEntity(p) => self
                .shield_application_for_unsecurified_persona(
                    UnsecurifiedPersonaShieldApplicationInput::from((
                        input.clone(),
                        UnsecurifiedPersona::with_unsecured_entity_control(
                            p.clone(),
                            entity.unsecured_entity_control.clone(),
                        ),
                    )),
                )
                .map(SecurityShieldApplicationForUnsecurifiedEntity::Persona),
        }
    }

    fn shield_application_for_securified_account(
        &self,
        input: SecurifiedAccountShieldApplicationInput,
    ) -> Result<SecurityShieldApplicationForSecurifiedAccount> {
        let manifest_for_variant =
            |variant: RolesExercisableInTransactionManifestCombination| {
                let manifest_with = input.clone();

                let manifest_with = Self::modify_manifest_add_fee_securified(
                    manifest_with,
                    variant,
                )?;

                let manifest_with = Self::modify_manifest_add_xrd_vault_contribution_for_securified_account_applying_shield(manifest_with, variant)?;

                Ok(manifest_with.manifest)
            };

        let initiate_with_recovery_complete_with_primary = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary)?;

        let initiate_with_recovery_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation)?;

        let initiate_with_recovery_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion)?;

        let initiate_with_primary_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation)?;

        let initiate_with_primary_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion)?;

        let account_with_optional_paying_account = SecurityShieldApplicationForSecurifiedAccountWithOptionalPayingAccount::new(input.entity_applying_shield.clone(), input.maybe_other_payer_and_balance().map(|p| p.entity));

        Ok(SecurityShieldApplicationForSecurifiedAccount::new(
            account_with_optional_paying_account,
            initiate_with_recovery_complete_with_primary,
            initiate_with_recovery_complete_with_confirmation,
            initiate_with_recovery_delayed_completion,
            initiate_with_primary_complete_with_confirmation,
            initiate_with_primary_delayed_completion,
        ))
    }

    fn modify_manifest_add_xrd_vault_contribution_for_securified_persona_applying_shield(
        input: SecurifiedPersonaShieldApplicationInput,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<SecurifiedPersonaShieldApplicationInput> {
        todo!()
    }

    fn modify_manifest_add_xrd_vault_contribution_for_securified_account_applying_shield(
        input: SecurifiedAccountShieldApplicationInput,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<SecurifiedAccountShieldApplicationInput> {
        todo!()
    }

    fn shield_application_for_securified_persona(
        &self,
        input: SecurifiedPersonaShieldApplicationInput,
    ) -> Result<SecurityShieldApplicationForSecurifiedPersona> {
        let manifest_for_variant =
            |variant: RolesExercisableInTransactionManifestCombination| {
                let manifest_with = input.clone();

                let manifest_with = Self::modify_manifest_add_fee_securified(
                    manifest_with,
                    variant,
                )?;

                let manifest_with = Self::modify_manifest_add_xrd_vault_contribution_for_securified_persona_applying_shield(manifest_with, variant)?;

                Ok(manifest_with.manifest)
            };

        let initiate_with_recovery_complete_with_primary = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary)?;

        let initiate_with_recovery_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation)?;

        let initiate_with_recovery_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion)?;

        let initiate_with_primary_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation)?;

        let initiate_with_primary_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion)?;

        let payer =
            SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount::new(
                input.entity_applying_shield.clone(),
                input.maybe_other_payer_and_balance().map(|p| p.entity), // TODO Should we fail here if the an account doing top up is not specified?
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
        input: AnySecurifiedShieldApplicationInput,
    ) -> Result<SecurityShieldApplicationForSecurifiedEntity> {
        let entity = &input.entity_applying_shield;
        match &entity.entity {
            AccountOrPersona::AccountEntity(a) => self
                .shield_application_for_securified_account(
                    SecurifiedAccountShieldApplicationInput::from((
                        input.clone(),
                        SecurifiedAccount::with_securified_entity_control(
                            a.clone(),
                            entity.securified_entity_control(),
                        ),
                    )),
                )
                .map(SecurityShieldApplicationForSecurifiedEntity::Account),
            AccountOrPersona::PersonaEntity(p) => self
                .shield_application_for_securified_persona(
                    SecurifiedPersonaShieldApplicationInput::from((
                        input.clone(),
                        SecurifiedPersona::with_securified_entity_control(
                            p.clone(),
                            entity.securified_entity_control(),
                        ),
                    )),
                )
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
            .map(|manifest_with_payer| {
                match &manifest_with_payer.entity_applying_shield {
                    EntityApplyingShield::Unsecurified(entity) => self
                        .shield_application_for_unsecurified_entity(
                            AnyUnsecurifiedShieldApplicationInput::from((
                                manifest_with_payer.clone(),
                                entity.clone(),
                            )),
                        )
                        .map(SecurityShieldApplication::unsecurified),
                    EntityApplyingShield::Securified(entity) => self
                        .shield_application_for_securified_entity(
                            AnySecurifiedShieldApplicationInput::from((
                                manifest_with_payer.clone(),
                                entity.clone(),
                            )),
                        )
                        .map(SecurityShieldApplication::securified),
                }
            })
            .collect::<Result<Vec<SecurityShieldApplication>>>()
    }
}
