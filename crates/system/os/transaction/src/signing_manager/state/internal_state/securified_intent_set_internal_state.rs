use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SecurifiedIntentSetInternalState {
    intent_set_id: Immutable<IntentSetID>,
    account_paying_for_transaction: Immutable<ApplicationInputPayingAccount>,
    pub(crate) entity_applying_shield: Immutable<AnySecurifiedEntity>,
    initiate_with_recovery_complete_with_primary: IntentVariantState,
    initiate_with_recovery_complete_with_confirmation: IntentVariantState,
    initiate_with_recovery_delayed_completion: IntentVariantState,
    initiate_with_primary_complete_with_confirmation: IntentVariantState,
    initiate_with_primary_delayed_completion: IntentVariantState,
}

impl TryFrom<(IntentSetID, IntentVariantState, AnySecurifiedEntity)>
    for EntitySignedForWithVariant
{
    type Error = CommonError;
    fn try_from(
        value: (IntentSetID, IntentVariantState, AnySecurifiedEntity),
    ) -> Result<Self> {
        let (intent_set_id, intent_variant_state, entity) = value;
        let signer = entity.entity.address();
        let variant = *intent_variant_state.variant;
        let signatures_per_role = intent_variant_state.signatures_per_role.0.into_iter().map(|(role, sigs)| {
            let sigs_per_entity = sigs.signatures_non_empty_map_with_non_empty_values()?;
            assert_eq!(sigs_per_entity.len(), 1, "expected only one entity to have sign - we have not signed with payer yet.");
            let sigs = sigs_per_entity.get(&signer).ok_or(CommonError::Unknown)?.clone();
            Ok((role, sigs))
        }).collect::<Result<IndexMap<RoleKind, IndexSet<_>>>>()?;

        let relevant_roles = variant.exercisable_roles();
        let mut signatures = IndexSet::new();
        for role in relevant_roles {
            let sigs =
                signatures_per_role.get(&role).ok_or(CommonError::Unknown)?; // TODO specific error variant

            if sigs.is_empty() {
                return Err(CommonError::Unknown); // TODO specific error variant
            }
            signatures.extend(sigs.clone());
        }

        Ok(EntitySignedForWithVariant::new(
            intent_set_id,
            (*intent_variant_state.intent).clone(),
            entity.entity,
            signatures,
            Some(*intent_variant_state.variant),
        ))
    }
}

impl SecurifiedIntentSetInternalState {
    pub(crate) fn get_signed_intents(
        &self,
    ) -> Result<Vec<EntitySignedForWithVariant>> {
        Ok(self
            ._all_intent_variant_states()
            .into_iter()
            .map(|v| {
                (
                    *self.intent_set_id,
                    v.clone(),
                    (*self.entity_applying_shield).clone(),
                )
            })
            .map(EntitySignedForWithVariant::try_from)
            .collect::<Result<Vec<_>>>()?)
    }

    pub(crate) fn paying_account(&self) -> Account {
        self.account_paying_for_transaction.account()
    }

    pub(crate) fn transaction_intent_hashes(
        &self,
    ) -> IndexSet<TransactionIntentHash> {
        self._all_intent_variant_states()
            .iter()
            .map(|v| v.intent.transaction_intent_hash())
            .collect()
    }

    fn _all_intent_variant_states(&self) -> Vec<&IntentVariantState> {
        vec![
            &self.initiate_with_recovery_complete_with_primary,
            &self.initiate_with_recovery_complete_with_confirmation,
            &self.initiate_with_recovery_delayed_completion,
            &self.initiate_with_primary_complete_with_confirmation,
            &self.initiate_with_primary_delayed_completion,
        ]
    }

    pub(crate) fn variants_for_role(
        &self,
        role_kind: RoleKind,
    ) -> Vec<&IntentVariantState> {
        self._all_intent_variant_states()
            .into_iter()
            .filter(|v| v.variant.exercisable_roles().contains(&role_kind))
            .collect()
    }

    fn get_variant_state(
        &mut self,
        variant: RolesExercisableInTransactionManifestCombination,
    ) -> &mut IntentVariantState {
        match variant {
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary => {
                &mut self.initiate_with_recovery_complete_with_primary
            },
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation => {
                &mut self.initiate_with_recovery_complete_with_confirmation
            },
            RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion => {
                &mut self.initiate_with_recovery_delayed_completion
            },
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation => {
                &mut self.initiate_with_primary_complete_with_confirmation
            },
            RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion => {
                &mut self.initiate_with_primary_delayed_completion
            },
        }
    }

    pub(crate) fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: EntitySignedFor,
    ) {
        assert_eq!(
            intent_with_signatures.entity.address(),
            self.entity_applying_shield.address()
        );

        RolesExercisableInTransactionManifestCombination::variants_for_role(
            intent_with_signatures.role_kind(),
        )
        .into_iter()
        .for_each(|variant| {
            let variant_state = self.get_variant_state(variant);
            variant_state.update_with_intent_with_signatures(
                intent_with_signatures.clone(),
            );
        });
    }

    fn new(
        intent_set_id: impl Into<Immutable<IntentSetID>>,
        account_paying_for_transaction: impl Into<
            Immutable<ApplicationInputPayingAccount>,
        >,
        entity_applying_shield: impl Into<Immutable<AnySecurifiedEntity>>,
        initiate_with_recovery_complete_with_primary: IntentVariantState,
        initiate_with_recovery_complete_with_confirmation: IntentVariantState,
        initiate_with_recovery_delayed_completion: IntentVariantState,
        initiate_with_primary_complete_with_confirmation: IntentVariantState,
        initiate_with_primary_delayed_completion: IntentVariantState,
    ) -> Self {
        let self_ = Self {
            intent_set_id: intent_set_id.into(),
            account_paying_for_transaction: account_paying_for_transaction
                .into(),
            entity_applying_shield: entity_applying_shield.into(),
            initiate_with_recovery_complete_with_primary,
            initiate_with_recovery_complete_with_confirmation,
            initiate_with_recovery_delayed_completion,
            initiate_with_primary_complete_with_confirmation,
            initiate_with_primary_delayed_completion,
        };

        self_._all_intent_variant_states().iter().for_each(|v| {
            v.intent
                .validate_required_signers_are([
                    self_.entity_applying_shield.address(),
                    self_
                        .account_paying_for_transaction
                        .account_address()
                        .into(),
                ])
                .expect("Discrepancy! invalid manifest (payer, entity) combo")
        });

        self_
    }
}
impl
    From<(
        SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents,
        IntentSetID,
    )> for SecurifiedIntentSetInternalState
{
    fn from(
        (shield_application, intent_set_id): (
            SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents,
            IntentSetID,
        ),
    ) -> Self {
        Self::new(
            intent_set_id,
            shield_application.paying_account(),
            shield_application.entity_applying_shield(),
            IntentVariantState::new(
                shield_application.initiate_with_recovery_complete_with_primary(),
                RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary
            ),
            IntentVariantState::new(
                shield_application.initiate_with_recovery_complete_with_confirmation(),
                RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation
            ),
            IntentVariantState::new(
                shield_application.initiate_with_recovery_delayed_completion(),
                RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion
            ),
            IntentVariantState::new(
                shield_application.initiate_with_primary_complete_with_confirmation(),
                RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation
            ),
            IntentVariantState::new(
                shield_application.initiate_with_primary_delayed_completion(),
                RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion
            ),
        )
    }
}
