use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
pub(crate) struct IntentVariantState {
    #[debug("TXIntent omitted")]
    pub(crate) intent: Immutable<TransactionIntent>,

    pub(crate) variant:
        Immutable<RolesExercisableInTransactionManifestCombination>,

    /// The `role` of the values must match the key...
    pub(crate) signatures_per_role: IntentVariantSignaturesPerRoleState,
}

impl IntentVariantState {
    pub(crate) fn update_with_entity_not_signed_for(
        &mut self,
        not_signed: EntityNotSignedFor,
    ) {
        assert!(self
            .variant
            .can_exercise_role(not_signed.role_kind()));
        assert_eq!(not_signed.intent, *self.intent);

        self.signatures_per_role
        .update_with_entity_not_signed_for(intent_wnot_signedith_signatures)
    }

    pub(crate) fn update_with_entity_signed_for(
        &mut self,
        intent_with_signatures: EntitySignedFor,
    ) {
        assert!(self
            .variant
            .can_exercise_role(intent_with_signatures.role_kind()));
        assert_eq!(intent_with_signatures.intent, *self.intent);

        self.signatures_per_role
            .update_with_intent_with_signatures(intent_with_signatures)
    }

    pub(crate) fn new(
        intent: impl Into<Immutable<TransactionIntent>>,
        variant: impl Into<
            Immutable<RolesExercisableInTransactionManifestCombination>,
        >,
    ) -> Self {
        let variant = variant.into();
        let variant_ = *variant;
        Self {
            variant,
            intent: intent.into(),
            signatures_per_role: IntentVariantSignaturesPerRoleState::new(
                variant_,
            ),
        }
    }
}
