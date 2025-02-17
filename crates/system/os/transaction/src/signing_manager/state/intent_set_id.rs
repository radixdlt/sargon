use crate::prelude::*;

/// An ID generated for the purpose of being able to identify which "set" a
/// TransactionIntent belongs to.
#[derive(Clone, Copy, PartialEq, Eq, StdHash, derive_more::Debug)]
pub struct IntentSetID(Uuid);
impl Default for IntentSetID {
    fn default() -> Self {
        Self::new()
    }
}

impl IntentSetID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
