use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct IntentSetState {
    intent_set_id: IntentSetID,
    internal_state: IntentSetInternalState,
}

impl IntentSetState {
    fn can_exercise_role(&self, role_kind: RoleKind) -> bool {
        self.internal_state.can_exercise_role(role_kind)
    }

    fn new(
        intent_set_id: IntentSetID,
        shield_application: SecurityShieldApplicationWithTransactionIntents,
    ) -> Self {
        Self {
            intent_set_id,
            internal_state: IntentSetInternalState::from(shield_application),
        }
    }

    fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: EntitySignedFor,
    ) {
        assert_eq!(self.intent_set_id, intent_with_signatures.intent_set_id());
        self.internal_state
            .update_with_intent_with_signatures(intent_with_signatures);
    }
}
