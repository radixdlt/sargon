use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct SecurifiedIntentSetInternalState {
    account_paying_for_transaction: Immutable<ApplicationInputPayingAccount>,
    entity_applying_shield: Immutable<AnySecurifiedEntity>,
    initiate_with_recovery_complete_with_primary: IntentVariantState,
    initiate_with_recovery_complete_with_confirmation: IntentVariantState,
    initiate_with_recovery_delayed_completion: IntentVariantState,
    initiate_with_primary_complete_with_confirmation: IntentVariantState,
    initiate_with_primary_delayed_completion: IntentVariantState,
}
impl SecurifiedIntentSetInternalState {
    fn paying_account(&self) -> Account {
        self.account_paying_for_transaction.account()
    }

    fn transaction_intent_hashes(&self) -> IndexSet<TransactionIntentHash> {
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

    fn variants_for_role(
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

    fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: EntitySignedFor,
    ) {
        assert_eq!(
            intent_with_signatures.entity.address(),
            self.entity_applying_shield.address()
        );
        let variant = intent_with_signatures
            .variant()
            .expect("Should have variant for securified");
        let variant_state = self.get_variant_state(variant);
        variant_state
            .update_with_intent_with_signatures(intent_with_signatures);
    }

    fn new(
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
        Self {
            account_paying_for_transaction: account_paying_for_transaction
                .into(),
            entity_applying_shield: entity_applying_shield.into(),
            initiate_with_recovery_complete_with_primary,
            initiate_with_recovery_complete_with_confirmation,
            initiate_with_recovery_delayed_completion,
            initiate_with_primary_complete_with_confirmation,
            initiate_with_primary_delayed_completion,
        }
    }
}
impl From<SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents>
    for SecurifiedIntentSetInternalState
{
    fn from(
        shield_application: SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents,
    ) -> Self {
        Self::new(
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
