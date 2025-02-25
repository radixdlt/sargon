use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct IntentSetState {
    pub(crate) intent_set_id: IntentSetID,
    pub(crate) internal_state: IntentSetInternalState,
}

impl IntentSetState {
    pub(crate) fn get_signed_intent_set(&self) -> Result<SignedIntentSet> {
        let signed = self.internal_state.get_signed_intents()?;
        Ok(SignedIntentSet::new(self.intent_set_id, signed))
    }

    pub(crate) fn can_exercise_role(&self, role_kind: RoleKind) -> bool {
        self.internal_state.can_exercise_role(role_kind)
    }

    pub(crate) fn has_exercised_recovery_and_confirmation_role_for_all_entities(
        &self,
    ) -> bool {
        self.internal_state
            .has_exercised_recovery_and_confirmation_role_for_all_entities()
    }

    /// N.B. does not include fee payers
    pub(crate) fn entities_not_signed_for_with_recovery(
        &self,
    ) -> Option<AccountOrPersona> {
        self.internal_state.entities_not_signed_for_with_recovery()
    }

    /// N.B. does not include fee payers
    pub(crate) fn entities_signed_for_with_recovery_but_not_with_confirmation(
        &self,
    ) -> Option<AccountOrPersona> {
        self.internal_state
            .entities_signed_for_with_recovery_but_not_with_confirmation()
    }

    pub(crate) fn new(
        intent_set_id: IntentSetID,
        shield_application: SecurityShieldApplicationWithTransactionIntents,
    ) -> Self {
        Self {
            intent_set_id,
            internal_state: IntentSetInternalState::from((
                shield_application,
                intent_set_id,
            )),
        }
    }

    pub(crate) fn update_with_entity_signed_for(
        &mut self,
        intent_with_signatures: EntitySignedFor,
    ) {
        assert_eq!(self.intent_set_id, intent_with_signatures.intent_set_id());
        self.internal_state
            .update_with_entity_signed_for(intent_with_signatures);
    }

    pub(crate) fn update_with_entity_not_signed_for(
        &mut self,
        not_signed: EntityNotSignedFor,
    ) {
        assert_eq!(self.intent_set_id, not_signed.intent_set_id());
        self.internal_state
            .update_with_entity_not_signed_for(not_signed);
    }
}
