use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct IntentVariantState {
    pub(crate) intent: Immutable<TransactionIntent>,
    pub(crate) variant:
        Immutable<RolesExercisableInTransactionManifestCombination>,
    /// The `role` of the values must match the key...
    pub(crate) signatures_per_role: IntentVariantSignaturesPerRoleState,
}

impl IntentVariantState {
    pub(crate) fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: EntitySignedFor,
    ) {
        assert_eq!(intent_with_signatures.intent, *self.intent);
        let variant = intent_with_signatures
            .variant()
            .expect("Should have variant");
        assert_eq!(variant, *self.variant);

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
