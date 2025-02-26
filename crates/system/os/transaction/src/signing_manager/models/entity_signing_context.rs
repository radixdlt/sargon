use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, StdHash, derive_more::Debug)]
pub(crate) struct EntitySigningContext {
    pub(crate) intent_set_id: IntentSetID, // only internally relevant
    pub role_kind: RoleKind,
}
impl EntitySigningContext {
    pub fn new(intent_set_id: IntentSetID, role_kind: RoleKind) -> Self {
        Self {
            intent_set_id,
            role_kind,
        }
    }
}
