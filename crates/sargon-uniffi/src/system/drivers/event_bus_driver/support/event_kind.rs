use crate::prelude::*;
use sargon::EventKind as InternalEventKind;

/// A discriminator identifying the kind of `Event`, this has no associated
/// values and flattens the otherwise nested `Event` enum.
#[derive(
    Debug,
    Clone,
    
    PartialEq,
    Eq,
    Hash,
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

    /// Profile updated with multiple new factor sources.
    FactorSourcesAdded,

    /// An existing factor source has been updated
    FactorSourceUpdated,

    /// Profile updated with a new Security Structure.
    SecurityStructureAdded,
}

impl From<InternalEventKind> for EventKind {
    fn from(value: InternalEventKind) -> Self {
        match value {
            InternalEventKind::AccountAdded => EventKind::AccountAdded,
            InternalEventKind::AccountsAdded => EventKind::AccountsAdded,
            InternalEventKind::AccountUpdated => EventKind::AccountUpdated,
            InternalEventKind::Booted => EventKind::Booted,
            InternalEventKind::GatewayChangedCurrent => EventKind::GatewayChangedCurrent,
            InternalEventKind::ProfileSaved => EventKind::ProfileSaved,
            InternalEventKind::ProfileImported => EventKind::ProfileImported,
            InternalEventKind::ProfileUsedOnOtherDevice => EventKind::ProfileUsedOnOtherDevice,
            InternalEventKind::FactorSourceAdded => EventKind::FactorSourceAdded,
            InternalEventKind::FactorSourcesAdded => EventKind::FactorSourcesAdded,
            InternalEventKind::FactorSourceUpdated => EventKind::FactorSourceUpdated,
            InternalEventKind::SecurityStructureAdded => EventKind::SecurityStructureAdded,
        }
    }
}

impl Into<InternalEventKind> for EventKind {
    fn into(self) -> InternalEventKind {
        match self {
            EventKind::AccountAdded => InternalEventKind::AccountAdded,
            EventKind::AccountsAdded => InternalEventKind::AccountsAdded,
            EventKind::AccountUpdated => InternalEventKind::AccountUpdated,
            EventKind::Booted => InternalEventKind::Booted,
            EventKind::GatewayChangedCurrent => InternalEventKind::GatewayChangedCurrent,
            EventKind::ProfileSaved => InternalEventKind::ProfileSaved,
            EventKind::ProfileImported => InternalEventKind::ProfileImported,
            EventKind::ProfileUsedOnOtherDevice => InternalEventKind::ProfileUsedOnOtherDevice,
            EventKind::FactorSourceAdded => InternalEventKind::FactorSourceAdded,
            EventKind::FactorSourcesAdded => InternalEventKind::FactorSourcesAdded,
            EventKind::FactorSourceUpdated => InternalEventKind::FactorSourceUpdated,
            EventKind::SecurityStructureAdded => InternalEventKind::SecurityStructureAdded,
        }
    }
}

#[uniffi::export]
pub fn event_kind_affects_current_accounts(event_kind: EventKind) -> bool {
    event_kind.into_internal().affects_current_accounts()
}

#[uniffi::export]
pub fn event_kind_affects_current_network(event_kind: EventKind) -> bool {
    event_kind.into_internal().affects_current_network()
}

#[uniffi::export]
pub fn event_kind_affects_saved_gateways(event_kind: EventKind) -> bool {
    event_kind.into_internal().affects_saved_gateways()
}

#[uniffi::export]
pub fn event_kind_affects_factor_sources(event_kind: EventKind) -> bool {
    event_kind.into_internal().affects_factor_sources()
}

#[uniffi::export]
pub fn event_kind_affects_security_structures(event_kind: EventKind) -> bool {
    event_kind.into_internal().affects_security_structures()
}

#[uniffi::export]
pub fn event_kind_all() -> Vec<EventKind> {
    InternalEventKind::all().into_vec()
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
                | SecurityStructureAdded
                | FactorSourceAdded
                | FactorSourcesAdded
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
                | SecurityStructureAdded
                | FactorSourcesAdded
                | AccountsAdded
                | AccountUpdated => assert!(!affects),
            })
    }

    #[test]
    fn event_kind_affects_security_structures() {
        use EventKind::*;
        SUT::all()
            .into_iter()
            .map(|sut| (sut, sut.affects_security_structures()))
            .for_each(|(sut, affects)| match sut {
                Booted | ProfileImported | SecurityStructureAdded => {
                    assert!(affects)
                }
                ProfileUsedOnOtherDevice
                | FactorSourceAdded
                | FactorSourceUpdated
                | ProfileSaved
                | AccountAdded
                | GatewayChangedCurrent
                | FactorSourcesAdded
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
                | FactorSourcesAdded
                | SecurityStructureAdded
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
                | FactorSourcesAdded | FactorSourceUpdated => {
                    assert!(affects)
                }
                ProfileUsedOnOtherDevice
                | GatewayChangedCurrent
                | ProfileSaved
                | SecurityStructureAdded
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

    #[test]
    fn test_event_kind_affects_security_structures() {
        assert!(event_kind_affects_security_structures(Booted));
    }

    #[test]
    fn test_event_kind_affects_factor_sources() {
        assert!(event_kind_affects_factor_sources(Booted));
    }
}
