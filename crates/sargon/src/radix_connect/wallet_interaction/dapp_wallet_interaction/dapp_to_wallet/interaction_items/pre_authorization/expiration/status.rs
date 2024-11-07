use crate::prelude::*;

/// An enum that represents the expiration status of a subintent at a given time.
///
/// Useful for determining if a subintent is still valid at the moment the Host apps
/// receive the corresponding request.
#[derive(Debug, Clone, PartialEq)]
pub enum DappToWalletInteractionSubintentExpirationStatus {
    /// The subintent hasn't expired yet
    Valid,

    /// The subintent is too close to its expiration. Although it hasn't expired yet, the Host apps
    /// shouldn't allow the user dealing with it.
    TooCloseToExpiration,

    /// The subintent has already expired.
    Expired,
}

impl HasSampleValues for DappToWalletInteractionSubintentExpirationStatus {
    fn sample() -> Self {
        Self::Valid
    }

    fn sample_other() -> Self {
        Self::TooCloseToExpiration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionSubintentExpirationStatus;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
