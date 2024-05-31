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
    ProfileUsedOnOtherDevice,

    /// Profile updated with a new factor source.
    FactorSourceAdded,

    /// An existing factor source has been updated
    FactorSourceUpdated,
}

impl EventKind {
    /// Returns collection of all different EventKinds
    pub fn all() -> Vec<Self> {
        all::<Self>().collect()
    }

    /// If hosts should fetch account list due to an action which triggered the
    /// event of this kind to be emitted.
    ///
    /// E.g. if an account was saved into Profile, an event with the kind
    /// `EventKind::AccountAdded` will be emitted, which hosts SHOULD react to
    /// and thus fetch the account list and possibly update UI.
    pub fn affects_current_accounts(&self) -> bool {
        use EventKind::*;
        matches!(
            *self,
            Booted
                | ProfileImported
                | AccountAdded
                | AccountsAdded
                | AccountUpdated
                | GatewayChangedCurrent
        )
    }

    /// If hosts should check the current network due to an action which triggered the
    /// event of this kind to be emitted.
    ///
    /// E.g. if the current gateway was changed by the user, an event with the kind
    /// `EventKind::GatewayChangedCurrent` will be emitted, which hosts SHOULD react to
    /// and check the network of the new gateway and possibly update UI.
    pub fn affects_current_network(&self) -> bool {
        use EventKind::*;
        matches!(*self, Booted | ProfileImported | GatewayChangedCurrent)
    }

    /// If hosts should check saved gateways due to an action which triggered the
    /// event of this kind to be emitted.
    ///
    /// E.g. if the current gateway was changed by the user, an event with the kind
    /// `EventKind::GatewayChangedCurrent` will be emitted, which hosts SHOULD
    /// react to and fetch saved gateways and possibly update UI.
    pub fn affects_saved_gateways(&self) -> bool {
        use EventKind::*;
        matches!(*self, Booted | ProfileImported | GatewayChangedCurrent)
    }

    /// If hosts UI displaying factor sources (of any kind) should re-fetch
    /// the list from SargonOS.
    ///
    /// E.g. if a new account is created using factor source `X` then `x.common.last_used`,
    /// is updated and an event if kind `FactorSourceUpdated` is emitted, which does
    /// affect factor sources shown by host.
    pub fn affects_factor_sources(&self) -> bool {
        use EventKind::*;
        matches!(
            *self,
            Booted | ProfileImported | FactorSourceAdded | FactorSourceUpdated
        )
    }
}

#[uniffi::export]
pub fn event_kind_affects_current_accounts(event_kind: EventKind) -> bool {
    event_kind.affects_current_accounts()
}

#[uniffi::export]
pub fn event_kind_affects_current_network(event_kind: EventKind) -> bool {
    event_kind.affects_current_network()
}

#[uniffi::export]
pub fn event_kind_affects_saved_gateways(event_kind: EventKind) -> bool {
    event_kind.affects_saved_gateways()
}

#[uniffi::export]
pub fn event_kind_affects_factor_sources(event_kind: EventKind) -> bool {
    event_kind.affects_factor_sources()
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
    fn test_event_kind_affects_current_accounts() {
        use EventKind::*;
        SUT::all()
            .into_iter()
            .map(|sut| (sut, sut.affects_current_accounts()))
            .for_each(|(sut, affects)| match sut {
                Booted
                | ProfileImported
                | AccountAdded
                | AccountsAdded
                | AccountUpdated
                | GatewayChangedCurrent => assert!(affects),
                ProfileUsedOnOtherDevice
                | ProfileSaved
                | FactorSourceAdded
                | FactorSourceUpdated => {
                    assert!(!affects)
                }
            })
    }

    #[test]
    fn event_kind_affects_current_network() {
        use EventKind::*;
        SUT::all()
            .into_iter()
            .map(|sut| (sut, sut.affects_current_network()))
            .for_each(|(sut, affects)| match sut {
                Booted | ProfileImported | GatewayChangedCurrent => {
                    assert!(affects)
                }
                ProfileUsedOnOtherDevice
                | FactorSourceAdded
                | FactorSourceUpdated
                | ProfileSaved
                | AccountAdded
                | AccountsAdded
                | AccountUpdated => assert!(!affects),
            })
    }

    #[test]
    fn event_kind_affects_saved_gateways() {
        use EventKind::*;
        SUT::all()
            .into_iter()
            .map(|sut| (sut, sut.affects_saved_gateways()))
            .for_each(|(sut, affects)| match sut {
                Booted | ProfileImported | GatewayChangedCurrent => {
                    assert!(affects)
                }
                ProfileUsedOnOtherDevice
                | FactorSourceAdded
                | FactorSourceUpdated
                | ProfileSaved
                | AccountAdded
                | AccountsAdded
                | AccountUpdated => assert!(!affects),
            })
    }

    #[test]
    fn event_kind_affects_factor_source() {
        use EventKind::*;
        SUT::all()
            .into_iter()
            .map(|sut| (sut, sut.affects_factor_sources()))
            .for_each(|(sut, affects)| match sut {
                Booted | ProfileImported | FactorSourceAdded
                | FactorSourceUpdated => {
                    assert!(affects)
                }
                ProfileUsedOnOtherDevice
                | GatewayChangedCurrent
                | ProfileSaved
                | AccountAdded
                | AccountsAdded
                | AccountUpdated => assert!(!affects),
            })
    }
}

mod uniffi_tests {

    use super::*;
    use crate::EventKind::{
        AccountAdded, AccountUpdated, AccountsAdded, Booted, ProfileImported,
        ProfileSaved,
    };

    #[test]
    fn test_event_kind_all() {
        assert!(event_kind_all().into_iter().contains(&Booted));
    }

    #[test]
    fn test_event_kind_affects_current_accounts() {
        assert!(event_kind_affects_current_accounts(Booted));
    }

    #[test]
    fn test_event_kind_affects_current_network() {
        assert!(event_kind_affects_current_network(Booted));
    }

    #[test]
    fn test_event_kind_affects_saved_gateways() {
        assert!(event_kind_affects_saved_gateways(Booted));
    }
}
