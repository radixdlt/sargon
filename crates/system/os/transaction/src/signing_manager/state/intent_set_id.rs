use crate::prelude::*;

/// An ID generated for the purpose of being able to identify which "set" a
/// TransactionIntent belongs to.
#[derive(Clone, Copy, PartialEq, Eq, StdHash, Debug)]
pub(crate) struct IntentSetID(Uuid);
impl Default for IntentSetID {
    fn default() -> Self {
        Self::new()
    }
}

impl IntentSetID {
    /// Generates a new UUID and wraps it in an `IntentSetID`.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
