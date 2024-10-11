use crate::prelude::*;
use sargon::EventProfileModified as InternalEventProfileModified;

/// The active profile has been modified (might not have been saved yet).
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Enum)]
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

impl From<InternalEventProfileModified> for EventProfileModified {
    fn from(value: InternalEventProfileModified) -> Self {
        match value {
            InternalEventProfileModified::AccountAdded { address } => {
                EventProfileModified::AccountAdded {
                    address: address.into(),
                }
            }
            InternalEventProfileModified::AccountsAdded { addresses } => {
                EventProfileModified::AccountsAdded {
                    addresses: addresses.into_iter().map(Into::into).collect(),
                }
            }
            InternalEventProfileModified::AccountUpdated { address } => {
                EventProfileModified::AccountUpdated {
                    address: address.into(),
                }
            }
            InternalEventProfileModified::FactorSourceAdded { id } => {
                EventProfileModified::FactorSourceAdded { id: id.into() }
            }
            InternalEventProfileModified::FactorSourcesAdded { ids } => {
                EventProfileModified::FactorSourcesAdded {
                    ids: ids.into_iter().map(Into::into).collect(),
                }
            }
            InternalEventProfileModified::FactorSourceUpdated { id } => {
                EventProfileModified::FactorSourceUpdated { id: id.into() }
            }
            InternalEventProfileModified::SecurityStructureAdded { id } => {
                EventProfileModified::SecurityStructureAdded { id: id.into() }
            }
        }
    }
}

impl Into<InternalEventProfileModified> for EventProfileModified {
    fn into(self) -> InternalEventProfileModified {
        match self {
            EventProfileModified::AccountAdded { address } => {
                InternalEventProfileModified::AccountAdded {
                    address: address.into(),
                }
            }
            EventProfileModified::AccountsAdded { addresses } => {
                InternalEventProfileModified::AccountsAdded {
                    addresses: addresses.into_iter().map(Into::into).collect(),
                }
            }
            EventProfileModified::AccountUpdated { address } => {
                InternalEventProfileModified::AccountUpdated {
                    address: address.into(),
                }
            }
            EventProfileModified::FactorSourceAdded { id } => {
                InternalEventProfileModified::FactorSourceAdded {
                    id: id.into(),
                }
            }
            EventProfileModified::FactorSourcesAdded { ids } => {
                InternalEventProfileModified::FactorSourcesAdded {
                    ids: ids.into_iter().map(Into::into).collect(),
                }
            }
            EventProfileModified::FactorSourceUpdated { id } => {
                InternalEventProfileModified::FactorSourceUpdated {
                    id: id.into(),
                }
            }
            EventProfileModified::SecurityStructureAdded { id } => {
                InternalEventProfileModified::SecurityStructureAdded {
                    id: id.into(),
                }
            }
        }
    }
}
