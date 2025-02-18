use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, StdHash, derive_more::Debug)]
pub(crate) struct EntitySigningContext {
    pub(crate) intent_set_id: IntentSetID, // only internally relevant
    pub role_kind: RoleKind,
    pub variant: Option<RolesExercisableInTransactionManifestCombination>,
}
impl EntitySigningContext {
    pub fn new(
        intent_set_id: IntentSetID,
        role_kind: RoleKind,
        variant: impl Into<Option<RolesExercisableInTransactionManifestCombination>>,
    ) -> Self {
        let variant = variant.into();
        if let Some(variant) = variant.as_ref() {
            assert!(variant.exercisable_roles().contains(&role_kind))
        }
        Self {
            intent_set_id,
            role_kind,
            variant,
        }
    }
}
