use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SigningManagerOutcome(pub Vec<SignedIntent>);
