use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SigningManagerState {
    pub(crate) current_role: Option<RoleKind>, // None at start and when signing for fee payers.
    pub(crate) per_set_state: IndexMap<IntentSetID, IntentSetState>,
}

impl SigningManagerState {
    /// N.B. does not include fee payers
    pub(crate) fn entities_not_signed_for_with_recovery(
        &self,
    ) -> IndexSet<AccountOrPersona> {
        self.per_set_state
        .values()
        .filter_map(|s| s.entities_not_signed_for_with_recovery())
        .collect()
    }
    
    /// N.B. does not include fee payers
    pub(crate) fn entities_signed_for_with_recovery_but_not_with_confirmation(&self) -> IndexSet<AccountOrPersona> {
        self.per_set_state
        .values()
        .filter_map(|s| s.entities_signed_for_with_recovery_but_not_with_confirmation())
        .collect()
    }

    pub(crate) fn update_with_exercise_role_outcome(
        &mut self,
        outcome: ExerciseRoleOutcome,
    ) {
        // println!(
        //     "🐌 outcome.entities_signed_for - SetIDs: {:?}",
        //     outcome
        //         .entities_signed_for
        //         .iter()
        //         .map(|e| e.context.intent_set_id)
        //         .collect_vec()
        // );
        self.update_with_each_entity_signed_for(outcome.entities_signed_for);

        self.update_with_entities_not_signed_for(
            outcome.entities_not_signed_for,
        );
    }

    pub(crate) fn update_with_each_entity_signed_for(
        &mut self,
        entities_signed_for: EntitiesSignedFor,
    ) {
        entities_signed_for
            .0
            .into_iter()
            .for_each(|entity_signed_for| {
                self.update_with_entity_signed_for(entity_signed_for);
            })
    }

    pub(crate) fn update_with_entities_not_signed_for(
        &mut self,
        entities_not_signed_for: EntitiesNotSignedFor,
    ) {
        entities_not_signed_for.0.into_iter().for_each(
            |entity_not_signed_for| {
                self.update_with_entity_not_signed_for(entity_not_signed_for);
            },
        )
    }

    fn update_with_entity_not_signed_for(
        &mut self,
        not_signed: EntityNotSignedFor,
    ) {
        let key = not_signed.intent_set_id();
        let existing = self
            .per_set_state
            .get_mut(&key)
            .expect("Should have created");
        existing.update_with_entity_not_signed_for(not_signed);
    }

    fn update_with_entity_signed_for(
        &mut self,
        intent_with_signatures: EntitySignedFor,
    ) {
        let key = intent_with_signatures.intent_set_id();
        let existing = self
            .per_set_state
            .get_mut(&key)
            .expect("Should have created");
        existing.update_with_entity_signed_for(intent_with_signatures);
    }
}

impl SigningManagerState {
    pub(crate) fn new(
        intent_sets: impl IntoIterator<
            Item = SecurityShieldApplicationWithTransactionIntents,
        >,
    ) -> Self {
        Self {
            current_role: None, // we start with Recovery
            per_set_state: intent_sets
                .into_iter()
                .map(|shield_application| {
                    let intent_set_id = IntentSetID::new();
                    let value =
                        IntentSetState::new(intent_set_id, shield_application);
                    (intent_set_id, value)
                })
                .collect::<IndexMap<IntentSetID, _>>(),
        }
    }
}
