use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SignedIntentWithContext {
    pub signed_intent: SignedIntent,
    pub context: EntitySigningContext,
}
