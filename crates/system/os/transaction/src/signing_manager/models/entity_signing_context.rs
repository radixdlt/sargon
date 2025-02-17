use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, StdHash, derive_more::Debug)]
pub struct EntitySigningContext {
    pub intent_set_id: IntentSetID,
    pub role_kind: RoleKind,
    pub variant: Option<RolesExercisableInTransactionManifestCombination>,
}
impl EntitySigningContext {
    pub fn new(
        intent_set_id: IntentSetID,
        role_kind: RoleKind,
        variant: Option<RolesExercisableInTransactionManifestCombination>,
    ) -> Self {
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
