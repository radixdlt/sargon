use crate::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ApplySecurityShieldSignedPayload {
    /// Only one transaction per application - for Securified Entities we will have had 5 manifests
    /// and we select "the best" (quick confirm if possible) depending on the outcome of the
    /// signing process
    pub notarized_transactions: Vec<NotarizedTransaction>,
}
