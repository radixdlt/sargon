use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SigningManagerOutcome(pub Vec<SignedIntent>);
