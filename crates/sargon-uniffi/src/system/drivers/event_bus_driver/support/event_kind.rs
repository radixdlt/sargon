use crate::prelude::*;
use sargon::EventKind as InternalEventKind;

/// A discriminator identifying the kind of `Event`, this has no associated
/// values and flattens the otherwise nested `Event` enum.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
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
            InternalEventKind::GatewayChangedCurrent => {
                EventKind::GatewayChangedCurrent
            }
            InternalEventKind::ProfileSaved => EventKind::ProfileSaved,
            InternalEventKind::ProfileImported => EventKind::ProfileImported,
            InternalEventKind::ProfileUsedOnOtherDevice => {
                EventKind::ProfileUsedOnOtherDevice
            }
            InternalEventKind::FactorSourceAdded => {
                EventKind::FactorSourceAdded
            }
            InternalEventKind::FactorSourcesAdded => {
                EventKind::FactorSourcesAdded
            }
            InternalEventKind::FactorSourceUpdated => {
                EventKind::FactorSourceUpdated
            }
            InternalEventKind::SecurityStructureAdded => {
                EventKind::SecurityStructureAdded
            }
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
            EventKind::GatewayChangedCurrent => {
                InternalEventKind::GatewayChangedCurrent
            }
            EventKind::ProfileSaved => InternalEventKind::ProfileSaved,
            EventKind::ProfileImported => InternalEventKind::ProfileImported,
            EventKind::ProfileUsedOnOtherDevice => {
                InternalEventKind::ProfileUsedOnOtherDevice
            }
            EventKind::FactorSourceAdded => {
                InternalEventKind::FactorSourceAdded
            }
            EventKind::FactorSourcesAdded => {
                InternalEventKind::FactorSourcesAdded
            }
            EventKind::FactorSourceUpdated => {
                InternalEventKind::FactorSourceUpdated
            }
            EventKind::SecurityStructureAdded => {
                InternalEventKind::SecurityStructureAdded
            }
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
