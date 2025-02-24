use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("{:?}", self.0)]
pub(crate) struct IntentVariantSignaturesPerRoleState(
    pub(crate) IndexMap<RoleKind, IntentVariantSignaturesForRoleState>,
);

impl IntentVariantSignaturesPerRoleState {
    pub(crate) fn update_with_entity_not_signed_for(
        &mut self,
        not_signed: EntityNotSignedFor,
    ) {
        let state_for_role = self
            .0
            .get_mut(&not_signed.role_kind())
            .expect("Should have created empty state for each role.");

        state_for_role.update_with_entity_not_signed_for(not_signed);
    }

    pub(crate) fn update_with_intent_with_signatures(
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

    pub(crate) fn has_exercised_role_for_all_entities(
        &self,
        role_kind: RoleKind,
    ) -> bool {
        if let Some(value) = self.0.get(&role_kind) {
            value.has_exercised_role_for_all_entities(role_kind)
        } else {
            false
        }
    }
    
    pub(crate) fn has_skipped_exercising_role(
        &self,
        role_kind: RoleKind,
    ) -> bool {
        if let Some(value) = self.0.get(&role_kind) {
            value.has_skipped_exercising_role(role_kind)
        } else {
            false
        }
    }

    pub(crate) fn new(
        variant: RolesExercisableInTransactionManifestCombination,
    ) -> Self {
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
