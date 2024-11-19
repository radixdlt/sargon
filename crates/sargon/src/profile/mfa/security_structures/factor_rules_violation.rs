use crate::prelude::*;

use thiserror::Error as ThisError;

#[derive(Clone, Debug, ThisError, PartialEq)]
pub enum FactorRulesViolation {
    #[error("Unknown Error")]
    Unknown = 10000,
}

impl From<FactorRulesViolation> for CommonError {
    fn from(err: FactorRulesViolation) -> Self {
        todo!()
    }
}
