use crate::prelude::*;

use thiserror::Error as ThisError;

#[derive(Clone, Debug, ThisError, PartialEq)]
pub enum FactorRulesViolation {
    
    #[error("Non-Canonical Primary Role: contains multiple Device Factors which is not allowed.")]
    NonCanonicalPrimaryRoleContainsMultipleDeviceFactors,
    
    #[error("Non-Canonical Recovery Role: contains Threshold Factors which is not allowed.")]
    NonCanonicalRecoveryRoleContainsThresholdFactors,
    
    #[error("Non-Canonical Confirmation Role: contains Threshold Factors which is not allowed.")]
    NonCanonicalConfirmationRoleContainsThresholdFactors,
}

impl From<FactorRulesViolation> for CommonError {
    fn from(_err: FactorRulesViolation) -> Self {
        todo!()
    }
}
