use crate::prelude::*;

/// A discriminator identifying the kind of `Event`, this has no associated
/// values and flattens the otherwise nested `Event` enum.
#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum EventKind {
    /* Sort lexicographically */
    /// Profile updated with a new account.
    AddedAccount,

    /// Profile updated with new accounts.
    AddedAccounts,

    /// SargonOS did boot.
    Booted,

    /// Current Gateway changed
    GatewayChangedCurrent,

    /// Profile was saved.
    ProfileSaved,

    /// Profile was last used on another device.
    ProfileLastUsedOnOtherDevice,
}

impl HasSampleValues for EventKind {
    fn sample() -> Self {
        Self::Booted
    }

    fn sample_other() -> Self {
        Self::ProfileSaved
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EventKind;

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
