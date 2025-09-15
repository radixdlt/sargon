use crate::prelude::*;
use sargon::EventProfileModified as InternalEventProfileModified;

/// The active profile has been modified (might not have been saved yet).
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum EventProfileModified {
    /// A new account with `address` was inserted into the active profile
    AccountAdded { address: AccountAddress },

    /// New accounts with `addresses` were inserted into the active profile
    AccountsAdded { addresses: Vec<AccountAddress> },

    /// An existing account has been updated
    AccountUpdated { address: AccountAddress },

    /// Existing accounts have been updated
    AccountsUpdated { addresses: Vec<AccountAddress> },

    /// Profile updated with a new factor source.
    FactorSourceAdded { id: FactorSourceID },

    /// Profile updated with many new factor sources.
    FactorSourcesAdded { ids: Vec<FactorSourceID> },

    /// An existing factor source has been updated
    FactorSourceUpdated { id: FactorSourceID },

    /// A collection of existing factor sources have been updated
    FactorSourcesUpdated { ids: Vec<FactorSourceID> },

    /// A new persona with `address` was inserted into the active profile
    PersonaAdded { address: IdentityAddress },

    /// New personas with `addresses` were inserted into the active profile
    PersonasAdded { addresses: Vec<IdentityAddress> },

    /// An existing persona has been updated
    PersonaUpdated { address: IdentityAddress },

    /// Existing personas have been updated
    PersonasUpdated { addresses: Vec<IdentityAddress> },

    /// Profile updated with a new Security Structure.
    SecurityStructureAdded { id: SecurityStructureID },

    /// Security structures have been updated
    SecurityStructuresUpdated { ids: Vec<SecurityStructureID> },

    /// Security structure has been updated
    SecurityStructureUpdated { id: SecurityStructureID },
}
