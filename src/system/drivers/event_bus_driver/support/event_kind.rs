use crate::prelude::*;

/// A discriminator identifying the kind of `Event`, this has no associated
/// values and flattens the otherwise nested `Event` enum.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    enum_iterator::Sequence,
    uniffi::Enum,
)]
pub enum EventKind {
    /* Sort lexicographically */
    /// Profile updated with a new account.
    AccountAdded,

    /// Profile updated with new accounts.
    AccountsAdded,

    /// An existing account has been updated
    AccountUpdated,

    /// SargonOS did boot.
    Booted,

    /// Current Gateway changed
    GatewayChangedCurrent,

    /// Profile was saved.
    ProfileSaved,

    /// A profile has been imported and has been set to active profile,
    /// and saved into secure storage.
    ProfileImported,

    /// Profile was last used on another device.
    ProfileLastUsedOnOtherDevice,
}

impl EventKind {
    pub fn affect_current_accounts(&self) -> bool {
        use EventKind::*;
        matches!(
            *self,
            Booted
                | ProfileImported
                | ProfileSaved
                | AccountAdded
                | AccountsAdded
                | AccountUpdated
        )
    }

    /// Returns collection of all different EventKinds
    pub fn all() -> Vec<Self> {
        all::<Self>().collect()
    }
}

#[uniffi::export]
pub fn event_kind_affect_current_accounts(event_kind: EventKind) -> bool {
    event_kind.affect_current_accounts()
}

#[uniffi::export]
pub fn event_kind_all() -> Vec<EventKind> {
    EventKind::all()
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
    use crate::EventKind::{
        AccountAdded, AccountUpdated, AccountsAdded, Booted, ProfileImported,
        ProfileSaved,
    };

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

    #[test]
    fn test_event_kind_affect_current_accounts() {
        use EventKind::*;
        SUT::all()
            .into_iter()
            .map(|sut| (sut, sut.affect_current_accounts()))
            .for_each(|(sut, affects)| match sut {
                Booted | ProfileImported | ProfileSaved | AccountAdded
                | AccountsAdded | AccountUpdated => assert!(affects),
                _ => assert!(!affects),
            })
    }
}
