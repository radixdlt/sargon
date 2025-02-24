use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SignedIntentWithContext {
    intent_set_id: IntentSetID,
    pub signed_intent: SignedIntent,
    pub entity_applying_shield: AccountOrPersona,
    pub variant: Option<RolesExercisableInTransactionManifestCombination>,
}

impl SignedIntentWithContext {
    pub fn new(
        intent_set_id: IntentSetID,
        signed_intent: SignedIntent,
        entity_applying_shield: AccountOrPersona,
        variant: Option<RolesExercisableInTransactionManifestCombination>,
    ) -> Self {
        Self {
            intent_set_id,
            signed_intent,
            entity_applying_shield,
            variant,
        }
    }
    pub(crate) fn intent_set_id(&self) -> IntentSetID {
        self.intent_set_id
    }
    pub(crate) fn as_confirm_after_delay_variant(
        &self,
    ) -> Option<Result<IntentVariantToConfirmAfterDelay>> {
        let variant = self.variant?;

        if variant.can_quick_confirm() {
            None
        } else {
            Some(
                self.entity_applying_shield
                .get_provisional()
                .ok_or(CommonError::EntityHasNoProvisionalSecurityConfigSet)
                .and_then(|provisional| {
                    provisional.as_factor_instances_derived()
                    .cloned()
                    .ok_or(CommonError::ProvisionalConfigInWrongStateExpectedInstancesDerived)
                })
                .map(|provisional| IntentVariantToConfirmAfterDelay::new(
                variant,
                self.signed_intent.intent.clone(),
                self.entity_applying_shield.address(),
                provisional.timed_recovery_delay_in_minutes()
            )))
        }
    }
}
