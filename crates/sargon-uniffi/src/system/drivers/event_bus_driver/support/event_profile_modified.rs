use crate::prelude::*;
use sargon::EventProfileModified as InternalEventProfileModified;

/// The active profile has been modified (might not have been saved yet).
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum EventProfileModified {
    /// A new account with `address` was inserted into the active profile
    AccountAdded { address: AccountAddress },

    /// New accounts with `addresses` were inserted into the active profile
    AccountsAdded { addresses: Vec<AccountAddress> },

    /// An existing account has been updated
    AccountUpdated { address: AccountAddress },

    /// Profile updated with a new factor source.
    FactorSourceAdded { id: FactorSourceID },

    /// Profile updated with many new factor sources.
    FactorSourcesAdded { ids: Vec<FactorSourceID> },

    /// An existing factor source has been updated
    FactorSourceUpdated { id: FactorSourceID },

    /// Profile updated with a new Security Structure.
    SecurityStructureAdded { id: SecurityStructureID },
}
