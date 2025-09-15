use crate::prelude::*;

/// The active profile has been modified (might not have been saved yet).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventProfileModified {
    /// A new account with `address` was inserted into the active profile
    AccountAdded { address: AccountAddress },

    /// New accounts with `addresses` were inserted into the active profile
    AccountsAdded { addresses: Vec<AccountAddress> },

    /// An existing account has been updated
    AccountUpdated { address: AccountAddress },

    /// Existing accounts have been updated
    AccountsUpdated { addresses: Vec<AccountAddress> },

    /// A new persona with `address` was inserted into the active profile
    PersonaAdded { address: IdentityAddress },

    /// New personas with `addresses` were inserted into the active profile
    PersonasAdded { addresses: Vec<IdentityAddress> },

    /// An existing persona has been updated
    PersonaUpdated { address: IdentityAddress },

    /// Existing personas have been updated
    PersonasUpdated { addresses: Vec<IdentityAddress> },

    /// Profile updated with a new factor source.
    FactorSourceAdded { id: FactorSourceID },

    /// Profile updated with many new factor sources.
    FactorSourcesAdded { ids: Vec<FactorSourceID> },

    /// An existing factor source has been updated
    FactorSourceUpdated { id: FactorSourceID },

    /// A collection of existing factor sources have been updated
    FactorSourcesUpdated { ids: Vec<FactorSourceID> },

    /// Profile updated with a new Security Structure.
    SecurityStructureAdded { id: SecurityStructureID },

    /// Security structures have been updated
    SecurityStructuresUpdated { ids: Vec<SecurityStructureID> },

    /// Security structure has been updated
    SecurityStructureUpdated { id: SecurityStructureID },
}

impl HasEventKind for EventProfileModified {
    fn kind(&self) -> EventKind {
        match self {
            Self::PersonaAdded { address: _ } => EventKind::PersonaAdded,
            Self::PersonaUpdated { address: _ } => EventKind::PersonaUpdated,
            Self::PersonasUpdated { addresses: _ } => {
                EventKind::PersonasUpdated
            }
            Self::AccountsUpdated { addresses: _ } => {
                EventKind::AccountsUpdated
            }
            Self::PersonasAdded { addresses: _ } => EventKind::PersonasAdded,
            Self::AccountUpdated { address: _ } => EventKind::AccountUpdated,
            Self::AccountAdded { address: _ } => EventKind::AccountAdded,
            Self::AccountsAdded { addresses: _ } => EventKind::AccountsAdded,
            Self::FactorSourcesAdded { ids: _ } => {
                EventKind::FactorSourcesAdded
            }
            Self::FactorSourceAdded { id: _ } => EventKind::FactorSourceAdded,
            Self::FactorSourceUpdated { id: _ } => {
                EventKind::FactorSourceUpdated
            }
            Self::FactorSourcesUpdated { ids: _ } => {
                EventKind::FactorSourcesUpdated
            }
            Self::SecurityStructureAdded { id: _ } => {
                EventKind::SecurityStructureAdded
            }
            Self::SecurityStructuresUpdated { ids: _ } => {
                EventKind::SecurityStructuresUpdated
            }
            Self::SecurityStructureUpdated { id: _ } => {
                EventKind::SecurityStructureUpdated
            }
        }
    }
}

pub trait IsProfileModifiedEvent<Address: Eq + std::hash::Hash> {
    fn profile_modified_event(
        is_update: bool,
        addresses: IndexSet<Address>,
    ) -> Option<EventProfileModified>;
}

impl IsProfileModifiedEvent<AccountAddress> for Account {
    fn profile_modified_event(
        is_update: bool,
        addresses: IndexSet<AccountAddress>,
    ) -> Option<EventProfileModified> {
        let address = addresses.iter().last().cloned()?;
        let addresses = addresses.clone().into_iter().collect_vec();
        let is_many = addresses.len() > 1;
        match (is_update, is_many) {
            (true, true) => {
                Some(EventProfileModified::AccountsUpdated { addresses })
            }
            (false, true) => {
                Some(EventProfileModified::AccountsAdded { addresses })
            }
            (true, false) => {
                Some(EventProfileModified::AccountUpdated { address })
            }
            (false, false) => {
                Some(EventProfileModified::AccountAdded { address })
            }
        }
    }
}

impl IsProfileModifiedEvent<IdentityAddress> for Persona {
    fn profile_modified_event(
        is_update: bool,
        addresses: IndexSet<IdentityAddress>,
    ) -> Option<EventProfileModified> {
        let address = addresses.iter().last().cloned()?;
        let addresses = addresses.clone().into_iter().collect_vec();
        let is_many = addresses.len() > 1;
        match (is_update, is_many) {
            (true, true) => {
                Some(EventProfileModified::PersonasUpdated { addresses })
            }
            (false, true) => {
                Some(EventProfileModified::PersonasAdded { addresses })
            }
            (true, false) => {
                Some(EventProfileModified::PersonaUpdated { address })
            }
            (false, false) => {
                Some(EventProfileModified::PersonaAdded { address })
            }
        }
    }
}

impl HasSampleValues for EventProfileModified {
    fn sample() -> Self {
        Self::AccountAdded {
            address: AccountAddress::sample(),
        }
    }

    fn sample_other() -> Self {
        Self::AccountsAdded {
            addresses: vec![
                AccountAddress::sample_mainnet_other(),
                AccountAddress::sample_mainnet(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EventProfileModified;

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
    fn test_kind() {
        let test = |s: SUT, exp: EventKind| {
            assert_eq!(s.kind(), exp);
        };
        test(
            SUT::AccountAdded {
                address: AccountAddress::sample(),
            },
            EventKind::AccountAdded,
        );

        test(
            SUT::AccountUpdated {
                address: AccountAddress::sample(),
            },
            EventKind::AccountUpdated,
        );
        test(
            SUT::AccountsAdded {
                addresses: vec![AccountAddress::sample()],
            },
            EventKind::AccountsAdded,
        );

        test(
            SUT::PersonaAdded {
                address: IdentityAddress::sample(),
            },
            EventKind::PersonaAdded,
        );

        test(
            SUT::PersonaUpdated {
                address: IdentityAddress::sample(),
            },
            EventKind::PersonaUpdated,
        );
        test(
            SUT::PersonasAdded {
                addresses: vec![IdentityAddress::sample()],
            },
            EventKind::PersonasAdded,
        );

        test(
            SUT::FactorSourcesAdded {
                ids: vec![FactorSourceID::sample()],
            },
            EventKind::FactorSourcesAdded,
        );

        test(
            SUT::FactorSourceAdded {
                id: FactorSourceID::sample(),
            },
            EventKind::FactorSourceAdded,
        );

        test(
            SUT::SecurityStructureAdded {
                id: SecurityStructureID::sample(),
            },
            EventKind::SecurityStructureAdded,
        );

        test(
            SUT::FactorSourceUpdated {
                id: FactorSourceID::sample(),
            },
            EventKind::FactorSourceUpdated,
        );

        test(
            SUT::FactorSourcesUpdated {
                ids: vec![FactorSourceID::sample()],
            },
            EventKind::FactorSourcesUpdated,
        );

        test(
            SUT::SecurityStructuresUpdated {
                ids: vec![SecurityStructureID::sample()],
            },
            EventKind::SecurityStructuresUpdated,
        );
    }
}
