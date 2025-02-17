use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedIntentWithContext {
    pub signed_intent: SignedIntent,
    pub context: EntitySigningContext,
}
