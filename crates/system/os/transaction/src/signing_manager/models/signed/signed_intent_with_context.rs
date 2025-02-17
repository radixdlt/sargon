use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SignedIntentWithContext {
    pub signed_intent: SignedIntent,
    pub entity_applying_shield: AccountOrPersona,
    pub context: EntitySigningContext,
}

impl SignedIntentWithContext {
    pub(crate) fn as_confirm_after_delay_variant(
        &self,
    ) -> Option<Result<IntentVariantToConfirmAfterDelay>> {
        let Some(variant) = self.context.variant else {
            return None;
        };

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
