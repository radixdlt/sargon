use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct IntentVariantSignaturesPerRoleState(
    IndexMap<RoleKind, IntentVariantSignaturesForRoleState>,
);
impl IntentVariantSignaturesPerRoleState {
    fn update_with_intent_with_signatures(
        &mut self,
        intent_with_signatures: EntitySignedFor,
    ) {
        let state_for_role = self
            .0
            .get_mut(&intent_with_signatures.role_kind())
            .expect("Should have created empty state for each role.");

        state_for_role
            .update_with_intent_with_signatures(intent_with_signatures);
    }

    fn new(variant: RolesExercisableInTransactionManifestCombination) -> Self {
        Self::_new_with_roles(variant.exercisable_roles())
    }
    fn _new_with_roles(roles: impl IntoIterator<Item = RoleKind>) -> Self {
        Self(
            roles
                .into_iter()
                .map(|role| {
                    (role, IntentVariantSignaturesForRoleState::new(role))
                })
                .collect::<IndexMap<_, _>>(),
        )
    }
}
