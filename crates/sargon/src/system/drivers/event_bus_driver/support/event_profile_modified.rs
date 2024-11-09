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

    /// A new persona with `address` was inserted into the active profile
    PersonaAdded { address: IdentityAddress },

    /// New personas with `addresses` were inserted into the active profile
    PersonasAdded { addresses: Vec<IdentityAddress> },

    /// An existing persona has been updated
    PersonaUpdated { address: IdentityAddress },

    /// Profile updated with a new factor source.
    FactorSourceAdded { id: FactorSourceID },

    /// Profile updated with many new factor sources.
    FactorSourcesAdded { ids: Vec<FactorSourceID> },

    /// An existing factor source has been updated
    FactorSourceUpdated { id: FactorSourceID },

    /// Profile updated with a new Security Structure.
    SecurityStructureAdded { id: SecurityStructureID },
}

impl HasEventKind for EventProfileModified {
    fn kind(&self) -> EventKind {
        match self {
            Self::PersonaAdded { address: _ } => EventKind::PersonaAdded,
            Self::PersonaUpdated { address: _ } => EventKind::PersonaUpdated,
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
            Self::SecurityStructureAdded { id: _ } => {
                EventKind::SecurityStructureAdded
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
    }
}
