use crate::prelude::*;
use sargon::EventKind as InternalEventKind;

/// A discriminator identifying the kind of `Event`, this has no associated
/// values and flattens the otherwise nested `Event` enum.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
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
