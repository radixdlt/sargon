use crate::prelude::*;

/// The event of having changed profile.
#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum EventProfileChange {
    /// A new account with `address` was inserted into the active profile
    AddedAccount { address: AccountAddress },

    /// New accounts with `addresses` were inserted into the active profile
    AddedAccounts { addresses: Vec<AccountAddress> },

    /// An existing account has been updated
    UpdatedAccount { address: AccountAddress },
}

impl HasEventKind for EventProfileChange {
    fn kind(&self) -> EventKind {
        match self {
            Self::UpdatedAccount { address: _ } => EventKind::UpdatedAccount,
            Self::AddedAccount { address: _ } => EventKind::AddedAccount,
            Self::AddedAccounts { addresses: _ } => EventKind::AddedAccounts,
        }
    }
}

impl HasSampleValues for EventProfileChange {
    fn sample() -> Self {
        Self::AddedAccount {
            address: AccountAddress::sample(),
        }
    }

    fn sample_other() -> Self {
        Self::AddedAccounts {
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
    type SUT = EventProfileChange;

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
            SUT::AddedAccount {
                address: AccountAddress::sample(),
            },
            EventKind::AddedAccount,
        );
        test(
            SUT::UpdatedAccount {
                address: AccountAddress::sample(),
            },
            EventKind::UpdatedAccount,
        );
        test(
            SUT::AddedAccounts {
                addresses: vec![AccountAddress::sample()],
            },
            EventKind::AddedAccounts,
        );
    }
}
