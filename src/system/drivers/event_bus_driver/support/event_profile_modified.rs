use crate::prelude::*;

/// The active profile has been modified (might not have been saved yet).
#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum EventProfileModified {
    /// A new account with `address` was inserted into the active profile
    AccountAdded { address: AccountAddress },

    /// New accounts with `addresses` were inserted into the active profile
    AccountsAdded { addresses: Vec<AccountAddress> },

    /// An existing account has been updated
    AccountUpdated { address: AccountAddress },

    /// Profile updated with a new factor source.
    FactorSourceAdded { id: FactorSourceID },

    /// An existing factor source has been updated
    FactorSourceUpdated { id: FactorSourceID },
}

impl HasEventKind for EventProfileModified {
    fn kind(&self) -> EventKind {
        match self {
            Self::AccountUpdated { address: _ } => EventKind::AccountUpdated,
            Self::AccountAdded { address: _ } => EventKind::AccountAdded,
            Self::AccountsAdded { addresses: _ } => EventKind::AccountsAdded,
            Self::FactorSourceAdded { id: _ } => EventKind::FactorSourceAdded,
            Self::FactorSourceUpdated { id: _ } => {
                EventKind::FactorSourceUpdated
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
            SUT::FactorSourceAdded {
                id: FactorSourceID::sample(),
            },
            EventKind::FactorSourceAdded,
        );

        test(
            SUT::FactorSourceUpdated {
                id: FactorSourceID::sample(),
            },
            EventKind::FactorSourceUpdated,
        );
    }
}
