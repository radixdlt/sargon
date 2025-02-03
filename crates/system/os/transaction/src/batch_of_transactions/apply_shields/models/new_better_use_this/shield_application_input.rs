use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShieldApplicationInput {
    Unsecurified(ApplicationInputForUnsecurifiedEntity),
    Securified(ApplicationInputForSecurifiedEntity),
}
